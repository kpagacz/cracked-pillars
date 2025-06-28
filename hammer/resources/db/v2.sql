-- User tables
CREATE TABLE IF NOT EXISTS users (
  email TEXT UNIQUE PRIMARY KEY,
  role TEXT NOT NULL
);

INSERT INTO users (email, role) values ("konrad.pagacz@gmail.com", "admin");
