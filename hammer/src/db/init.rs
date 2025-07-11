use crate::error::{Error, ErrorType};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

use crate::models::CONFIG;
use rusqlite::{CachedStatement, Connection, config::DbConfig};

const MIGRATION_TABLE: &str = "migrations";
const MIGRATION_FILE_NAME_COLUMN: &str = "name";

pub(crate) fn get_connection() -> Result<Connection, Error> {
    // Connection::open is idempotent.
    let conn = Connection::open(&CONFIG.db_path)
        .map_err(|_| "Failed to open a connection to the database")?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_FKEY, true)?;
    conn.set_db_config(DbConfig::SQLITE_DBCONFIG_ENABLE_TRIGGER, true)?;
    Ok(conn)
}

pub(crate) fn synchronize_db(connection: &Connection) -> Result<(), Error> {
    create_migration_table(connection)?;
    tracing::event!(
        tracing::Level::TRACE,
        "Created the migration table if it did not exist"
    );
    execute_missing_migrations(connection)?;
    Ok(())
}

fn create_migration_table(connection: &Connection) -> Result<(), Error> {
    connection
        .execute_batch(&format!(
            "BEGIN;
                CREATE TABLE if NOT EXISTS {} ( {} TEXT PRIMARY KEY);
                COMMIT;
            ",
            MIGRATION_TABLE, MIGRATION_FILE_NAME_COLUMN
        ))
        .map_err(|_| "Failed to create the migration table".into())
        .map(|_| ())
}

fn execute_missing_migrations(connection: &Connection) -> Result<(), Error> {
    let files = list_migration_files()?;
    tracing::trace!("Migration files found: {files:?}");
    let done_migrations = list_done_migrations(connection)?;
    tracing::trace!("Done migrations: {done_migrations:?}");
    let mut stmt = connection
        .prepare_cached(&format!(
            "INSERT INTO {} ({}) VALUES (?1)",
            MIGRATION_TABLE, MIGRATION_FILE_NAME_COLUMN
        ))
        .map_err(|_| "Failed to prepare the cached statement inserting migration files into db")?;
    let unexecuted_migrations = find_unexecuted_migrations(&files, &done_migrations)?;
    unexecuted_migrations
        .into_iter()
        .try_for_each(|migration_file| {
            execute_migration(migration_file, connection)?;
            mark_migration_done(migration_file, &mut stmt)
        })
}

fn find_unexecuted_migrations<'a>(
    files: &'a [PathBuf],
    done_migrations: &'a [OsString],
) -> Result<Vec<&'a PathBuf>, Error> {
    Ok(files
        .iter()
        .filter(|&file| {
            !done_migrations.contains(&file.file_name().expect("File name exists").to_os_string())
        })
        .collect())
}

fn list_migration_files() -> Result<Vec<PathBuf>, Error> {
    let paths = std::fs::read_dir(&CONFIG.db_migrations)
        .map_err(|_| "Failed to read migration directory")?;
    let mut migration_files: Vec<_> = paths
        .map(|res| {
            res.map(|entry| entry.path()).map_err(|e| {
                Error(
                    format!("Failed to process directory entry: {}", e),
                    ErrorType::Runtime,
                )
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    migration_files.sort();
    Ok(migration_files)
}

fn list_done_migrations(connection: &Connection) -> Result<Vec<OsString>, Error> {
    let mut stmt = connection
        .prepare(&format!(
            "SELECT {} FROM {}",
            MIGRATION_FILE_NAME_COLUMN, MIGRATION_TABLE
        ))
        .map_err(|_| "Failed to prepare the migration query")?;
    let mut done_migrations = vec![];
    let mut rows = stmt
        .query([])
        .map_err(|_| "Failed to query for done migrations")?;
    while let Some(row) = rows
        .next()
        .map_err(|_| "Failed to get the next migration row")?
    {
        let migration: String = row
            .get(0)
            .map_err(|_| "Failed to get value from the migration row")?;
        done_migrations.push(migration.into());
    }
    Ok(done_migrations)
}

fn execute_migration(file: &Path, connection: &Connection) -> Result<(), Error> {
    tracing::trace!("Executing the migration from file: {file:?}");
    let commands =
        std::fs::read_to_string(file).map_err(|_| "Failed to read the migration file")?;
    connection
        .execute_batch(&commands)
        .map_err(|e| format!("Failed to execute the migration file: {e:?}"))?;
    Ok(())
}

fn mark_migration_done(file: &Path, stmt: &mut CachedStatement) -> Result<(), Error> {
    stmt.execute([file.file_name().map(|name| name.to_str()).unwrap()])
        .map_err(|_| "Failed to mark the migration as done")?;
    Ok(())
}
