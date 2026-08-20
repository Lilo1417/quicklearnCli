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
        "DELETE FROM lernset WHERE lernset_id = ?1",
        [id as i64],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        Ok(())
    }
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
                let state: String = row.get(4)?;
                let rem: Option<i64> = row.get(5)?;
                let learnstate = SqlLIteLearnitemRepository::conv_state(state.as_str(), rem)?;

                let lern_id: i64 = row.get(1)?;
                Ok(Learnitem {
                    learnitem_id,
                    lernset_id: lern_id as usize,
                    origin_meaning: row.get(2)?,
                    trans_meaning: row.get(3)?,
                    learnstate,
                })
        })
    }
    fn delete(&self, id: usize) -> rusqlite::Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM learnitem WHERE learnitem_id",
            [id as i64]
        )?;
        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }
        Ok(())
    }
    fn list_from_lernset(&self, lernset_id: usize) -> rusqlite::Result<Vec<Learnitem>> {
        let mut stmt = self.conn.prepare("SELECT learnitem_id, lernset_id, origin_meaning, trans_meaning, learnstate, remaining FROM learnitem WHERE lernset_id = (?1)")?;
        let rows = stmt.query_map(params![lernset_id as i64],
            |row| {
                let state: String = row.get(4)?;
                let rem: Option<i64> = row.get(5)?;
                let learnstate = SqlLIteLearnitemRepository::conv_state(state.as_str(), rem)?;

                let row_learnitem_id: i64 = row.get(0)?;
                let row_lernset_id: i64 = row.get(1)?;

                Ok(Learnitem {
                    learnitem_id: row_learnitem_id as usize,
                    lernset_id: row_lernset_id as usize,
                    origin_meaning: row.get(2)?,
                    trans_meaning: row.get(3)?,
                    learnstate
                })
            })?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
    }
}
impl SqlLIteLearnitemRepository {
    fn conv_state(state: &str, rem: Option<i64>) -> rusqlite::Result<Learnstate> {
        match state {
            "Finished" => return Ok(Learnstate::Finished),
            "NotStarted" => return Ok(Learnstate::NotStarted),
            "Learning" => {
                return Ok(Learnstate::Learning(rem.unwrap_or(5) as usize))
            },
            _ => return Err(rusqlite::Error::IntegralValueOutOfRange(5, rem.unwrap_or(5)))
        };
    }
}
