use crate::{core_error::Result, Learnitem, Lernset, models::Learnstate};

pub trait LernsetRepository {
    fn create(&self, name: &str) -> Result<Lernset>;
    fn get(&self, id: usize) -> Result<Lernset>;
    fn list(&self) -> Result<Vec<Lernset>>;
    fn delete(&self, id: usize) -> Result<()>;
    fn update(&self, lernset: &Lernset) -> Result<()>;
}

pub trait LearnitemRepository {
    fn create(&self, lernset_id: usize, origin_meaning: String, trans_meaning: String, learnstate: Learnstate) -> Result<Learnitem>;
    fn list_from_lernset(&self, lernset_id: usize) -> Result<Vec<Learnitem>>;
    fn get(&self, learnitem_id: usize) -> Result<Learnitem>;
    fn delete(&self, id: usize) -> Result<()>;
    fn update(&self, learnitem: &Learnitem) -> Result<()>;
}
