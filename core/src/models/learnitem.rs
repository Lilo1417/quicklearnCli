type RemLearns= usize;

#[derive(Debug)]
enum Learnstate {
    Finished,
    Learning(RemLearns),
    NotStarted
}

#[derive(Debug)]
pub struct Learnitem {
    learnitem_id: usize,
    lernset_id: usize,
    origin_meaning: String,
    trans_meaning: String,
}
