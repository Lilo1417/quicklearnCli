use std::path::PathBuf;

use crate::{Learnitem, LearnitemRepository, Lernset, LernsetRepository, core_error::Result, db::open, models::Learnstate};
use rusqlite::{Connection, params};

pub struct Repository {
    pub sqlite_lernset: SqliteLernsetRepository,
    pub sqlite_learnitem: SqlLiteLearnitemRepository,
}
impl Repository {
    pub fn new(path: &PathBuf) -> Self {
        Repository { sqlite_lernset: SqliteLernsetRepository::new(path), sqlite_learnitem: SqlLiteLearnitemRepository::new(path) }
    }
}
pub struct SqliteLernsetRepository {
    conn: Connection,
}

impl SqliteLernsetRepository {
    pub fn new(path: &PathBuf) -> Self {
        let conn = match open(&path) {
            Ok(conn) => conn,
            Err(err) => panic!("Erro while opening the connection: {}", err)
        };
        SqliteLernsetRepository { conn }
    }
}
pub struct SqlLiteLearnitemRepository {
    conn: Connection
}
impl SqlLiteLearnitemRepository {
    pub fn new(path: &PathBuf) -> Self {
        let conn = match open(&path) {
            Ok(conn) => conn,
            Err(err) => panic!("Erro while opening the connection: {}", err)
        };
        SqlLiteLearnitemRepository { conn }
    }
}

impl LernsetRepository for SqliteLernsetRepository {
    fn create(&self, name: &str) -> Result<crate::Lernset> {
        self.conn.execute("INSERT INTO lernset (name) VALUES (?1)", params![name])?;
        
        let new_id = self.conn.last_insert_rowid();
        Ok(Lernset {
            lernset_id: new_id as usize,
            name: name.to_string()
        })
    }
    fn get(&self, id: usize) -> Result<Lernset> {
        Ok(self.conn.query_row(
            "SELECT lernset_id, name FROM lernset WHERE id = ?1",
            [id as i32],
            |row| {
                let id: i64 = row.get(0)?;
                Ok(Lernset {
                    lernset_id: id as usize,
                    name: row.get(1)?
                })
            })?)
    }
    fn delete(&self, id: usize) -> Result<()> {
        let rows_affected = self.conn.execute(
        "DELETE FROM lernset WHERE lernset_id = ?1",
        [id as i64],
        )?;

        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }

        Ok(())
    }
    fn list(&self) -> Result<Vec<Lernset>> {
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
            lernsets.push(lernset?);
        }
        Ok(lernsets)
    }
    fn update(&self, lernset: &Lernset) -> Result<()> {
        let rows_affected = self.conn.execute(
            "UPDATE lernset  SET name = ?1 WHERE lernset_id = ?2", params![lernset.name, lernset.lernset_id as i64])?;
        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }
        Ok(())
    }
}

impl LearnitemRepository for SqlLiteLearnitemRepository {
    fn create(&self, lernset_id: usize, origin_meaning: String, trans_meaning: String, learnstate: crate::models::Learnstate) -> Result<crate::Learnitem> {
        let remaining = learnstate.remaining();
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
    fn get(&self, learnitem_id: usize) -> Result<Learnitem> {
       Ok(self.conn.query_row("SELECT learnitem_id, lernset_id, origin_meaning, trans_meaning, learnstate, remaining FROM learnitem WHERE learnitem_id = (?1)", 
            params![learnitem_id as i64],
            |row| {
                let state: String = row.get(4)?;
                let rem: Option<i64> = row.get(5)?;
                let learnstate = SqlLiteLearnitemRepository::conv_state(state.as_str(), rem)?;

                let lern_id: i64 = row.get(1)?;
                Ok(Learnitem {
                    learnitem_id,
                    lernset_id: lern_id as usize,
                    origin_meaning: row.get(2)?,
                    trans_meaning: row.get(3)?,
                    learnstate,
                })
        })?)
    }
    fn delete(&self, id: usize) -> Result<()> {
        let rows_affected = self.conn.execute(
            "DELETE FROM learnitem WHERE learnitem_id = ?1",
            [id as i64]
        )?;
        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }
        Ok(())
    }
    fn list_from_lernset(&self, lernset_id: usize) -> Result<Vec<Learnitem>> {
        let mut stmt = self.conn.prepare("SELECT learnitem_id, lernset_id, origin_meaning, trans_meaning, learnstate, remaining FROM learnitem WHERE lernset_id = (?1)")?;
        let rows = stmt.query_map(params![lernset_id as i64],
            |row| {
                let state: String = row.get(4)?;
                let rem: Option<i64> = row.get(5)?;
                let learnstate = SqlLiteLearnitemRepository::conv_state(state.as_str(), rem)?;

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
        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }
    fn update(&self, learnitem: &Learnitem) -> Result<()> {
        let remaining = learnitem.learnstate.remaining();
        let learnstate_str = learnitem.learnstate.to_str();
        let rows_affected = self.conn.execute(
            "UPDATE learnitem SET lernset_id = ?1, origin_meaning = ?2, trans_meaning = ?3, learnstate = ?4, remaining = ?5 WHERE learnitem_id = ?6",
            params![learnitem.lernset_id as i64, learnitem.origin_meaning, learnitem.trans_meaning, learnstate_str, remaining as i64, learnitem.learnitem_id as i64])?;
        if rows_affected == 0 {
            return Err(rusqlite::Error::QueryReturnedNoRows.into());
        }

        Ok(())
    }
}
impl SqlLiteLearnitemRepository {
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
