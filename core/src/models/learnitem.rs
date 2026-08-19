type RemLearns= usize;

#[derive(Debug)]
pub enum Learnstate {
    Finished,
    Learning(RemLearns),
    NotStarted
}

impl Learnstate {
    pub(crate) fn to_str(&self) -> &str {
        match self {
            Learnstate::Learning(_) => "Learning",
            Learnstate::NotStarted => "NotStarted",
            Learnstate::Finished => "Finished"
        }
    }
}

#[derive(Debug)]
pub struct Learnitem {
    pub learnitem_id: usize,
    pub lernset_id: usize,
    pub origin_meaning: String,
    pub trans_meaning: String,
    pub learnstate: Learnstate,
}
