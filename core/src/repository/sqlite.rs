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
    fn get(&self, id: usize) -> rusqlite::Result<Lernset> {
        self.conn.query_row(
            "SELECT id, name FROM lernset WHERE id = ?1",
            [id as i32],
            |row| {
                let id: i64 = row.get(0)?;
                Ok(Lernset {
                    lernset_id: id as usize,
                    name: row.get(1)?
                })
            })
    }
    fn delete(&self, id: usize) -> rusqlite::Result<()> {
        let rows_affected = self.conn.execute(
        "DELETE FROM items WHERE id = ?1",
        [id as i32],
    )?;

    if rows_affected == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    Ok(())}
    fn list(&self) -> rusqlite::Result<Vec<Lernset>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM lernset")?;
        let lernset_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;
            Ok(Lernset {
                lernset_id: id as usize,
                name: row.get(1)?
            })
        })?;
        let mut lernsets = Vec::new();

        for lernset in lernset_iter {
            lernsets.push(lernset.expect("got no lernsets."));
        }
        Ok(lernsets)
    }
}
