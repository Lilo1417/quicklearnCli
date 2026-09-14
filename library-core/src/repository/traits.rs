use crate::{core_error::Result, Learnitem, Lernset, models::Learnstate};

pub trait LernsetRepository {
    fn create_lernset(&self, name: &str) -> Result<Lernset>;
    fn get_lernset(&self, id: usize) -> Result<Lernset>;
    fn list_lernsets(&self) -> Result<Vec<Lernset>>;
    fn delete_lernset(&self, id: usize) -> Result<()>;
    fn update_lernset(&self, lernset: &Lernset) -> Result<()>;
}

pub trait LearnitemRepository {
    fn create_learnitem(&self, lernset_id: usize, origin_meaning: String, trans_meaning: String, learnstate: Learnstate) -> Result<Learnitem>;
    fn list_from_lernset(&self, lernset_id: usize) -> Result<Vec<Learnitem>>;
    fn get_learnitem(&self, learnitem_id: usize) -> Result<Learnitem>;
    fn delete_learnitem(&self, id: usize) -> Result<()>;
    fn update_learnitem(&self, learnitem: &Learnitem) -> Result<()>;
}
