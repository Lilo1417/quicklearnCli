use std::path::Path;

use rusqlite::{Connection, Result};

pub(crate) fn open(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_key = ON;")?;
    Ok(conn)
}

pub(crate) fn run_migration(conn: &Connection) -> Result<()> {

    conn.execute_batch("CREATE TABLE IF NOT EXIST lernset (
    lernset_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
);")?;

    conn.execute_batch("CREATE TABLE IF NOT EXISTS learnitem (
    learnitem_id   INTEGER PRIMARY KEY,
    lernset_id     INTEGER NOT NULL,
    origin_meaning TEXT NOT NULL,
    trans_meaning  TEXT NOT NULL,
    learnstate     TEXT NOT NULL,
    remaining      INTEGER,
    FOREIGN KEY (lernset_id) REFERENCES lernset(lernset_id) ON DELETE CASCADE
);")?;
    Ok(())
}
