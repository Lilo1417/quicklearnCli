use crate::core_error::{self, Result};

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
    pub(crate) fn remaining(&self) -> usize {
        match self {
            Learnstate::Learning(rem) => *rem,
            _ => 0
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

impl Learnitem {
    pub fn update_learnstate(&mut self) -> Result<&Self> {
        match self.learnstate {
            Learnstate::NotStarted => {
                self.learnstate = Learnstate::Learning(4);
                Ok(self)
            },
            Learnstate::Learning(mut rem) => {
                rem -=1;
                if rem>0 { 
                    self.learnstate = Learnstate::Learning(rem);
                } else {
                    self.learnstate = Learnstate::Finished;
                }
                Ok(self)
            },
            Learnstate::Finished => Ok(self)
        }
    }
}
