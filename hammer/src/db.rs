use crate::error::Error;
use std::path::{Path, PathBuf};

use rusqlite::{CachedStatement, Connection};

const DB_PATH: &str = "./hammer.db3";
const MIGRATION_TABLE: &str = "migrations";
const MIGRATION_FILE_NAME_COLUMN: &str = "name";
const MIGRATION_FILES_PATH: &str = "./resources/db";

pub(crate) fn get_connection() -> Result<Connection, Error> {
    // Connection::open is idempotent.
    Connection::open(DB_PATH).map_err(|_| "Failed to open a connection to the database".into())
}

pub(crate) fn synchronize_db(connection: &Connection) -> Result<(), Error> {
    create_migration_table(connection)?;
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
    let done_migrations = list_done_migrations(connection)?;
    let mut stmt = connection
        .prepare_cached(&format!(
            "INSERT INTO {} ({}) VALUES (?1)",
            MIGRATION_TABLE, MIGRATION_FILE_NAME_COLUMN
        ))
        .map_err(|_| "Failed to prepare the cached statement inserting migration files into db")?;
    files
        .iter()
        .filter(|migration_file| !done_migrations.contains(migration_file))
        .try_for_each(|migration_file| {
            execute_migration(migration_file, connection)?;
            mark_migration_done(migration_file, &mut stmt)
        })
}

fn list_migration_files() -> Result<Vec<PathBuf>, Error> {
    let paths = std::fs::read_dir(MIGRATION_FILES_PATH)
        .map_err(|_| "Failed to read migration directory")?;
    paths
        .map(|res| {
            res.map(|entry| entry.path())
                .map_err(|e| format!("Failed to process directory entry: {}", e).into())
        })
        .collect()
}

fn list_done_migrations(connection: &Connection) -> Result<Vec<PathBuf>, Error> {
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
        done_migrations.push(PathBuf::from(migration));
    }
    Ok(done_migrations)
}

fn execute_migration(file: &Path, connection: &Connection) -> Result<(), Error> {
    println!("Executing the migration from file: {file:?}");
    let commands =
        std::fs::read_to_string(file).map_err(|_| "Failed to read the migration file")?;
    connection
        .execute_batch(&commands)
        .map_err(|_| "Failed to execute the migration file")?;
    Ok(())
}

fn mark_migration_done(file: &Path, stmt: &mut CachedStatement) -> Result<(), Error> {
    stmt.execute([&file.to_str().unwrap()])
        .map_err(|_| "Failed to mark the migration as done")?;
    Ok(())
}
