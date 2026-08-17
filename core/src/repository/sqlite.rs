use crate::{Lernset, repository::traits::LernsetRepository};
use rusqlite::{Connection, params};

pub struct SqliteLernsetRepository {
    conn: Connection,
}

impl LernsetRepository for SqliteLernsetRepository {
    fn create(&self, name: &str) -> rusqlite::Result<crate::Lernset> {
        self.conn.execute("INSERT INTO lernset (name) VALUES (?1)", params![name]);
        
        let new_id = self.conn.last_insert_rowid();
        Ok(Lernset {
            lernset_id: new_id as usize,
            name: name.to_string()
        })
    }
}
