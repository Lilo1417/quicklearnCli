use crate::{Lernset, Learnitem, models::Learnstate, LearnitemRepository, LernsetRepository};
use rusqlite::{Connection, params};

pub struct SqliteLernsetRepository {
    conn: Connection,
}
pub struct SqlLIteLearnitemRepository {
    conn: Connection
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

impl LearnitemRepository for SqlLIteLearnitemRepository {
    fn create(&self, lernset_id: usize, origin_meaning: String, trans_meaning: String, learnstate: crate::models::Learnstate) -> rusqlite::Result<crate::Learnitem> {
        let remaining: usize = match learnstate {
            Learnstate::Learning(rem) => rem,
            _ => 0
        };
        let learnstate_str = learnstate.to_str();
        self.conn.execute("INSERT INTO learnitem (lernset_id, origin_meaning, trans_meaning, learnstate, remaining) VALUES (?1, ?2, ?3, ?4, ?5)", params![
            lernset_id as i32,
            origin_meaning,
            trans_meaning,
            learnstate_str,
            remaining as i32
        ])?;
        let new_id = self.conn.last_insert_rowid();
        Ok(Learnitem {
            learnitem_id: new_id as usize,
            lernset_id,
            origin_meaning,
            trans_meaning,
            learnstate
        })
    }
    fn get(&self, learnitem_id: usize) -> rusqlite::Result<Learnitem> {
       self.conn.query_row("SELECT learnitem_id, lernset_id, origin_meaning, trans_meaning, learnstate, remaining FROM learnitem WHERE learnitem_id = (?1)", 
            params![learnitem_id as i64],
            |row| {
                let learnstate = match row.get(4)? as &str {
                    "Finished" => Learnstate::Finished,
                    "NotStarted" => Learnstate::NotStarted,
                    "Learning" => {
                        let rem = row.get(5)? as usize;
                        Learnstate::Learning(rem)
                    }
                };
                Ok(Learnitem {
                    learnitem_id,
                    lernset_id: row.get(1)? as usize,
                    origin_meaning: row.get(2)?,
                    trans_meaning: row.get(3)?,
                    learnstate,
                })
        })
    }
}
