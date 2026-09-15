use core::fmt;
use std::fmt::write;

use crate::core_error::{self, CoreError, Result};

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

impl fmt::Display for Learnstate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Learnstate::Finished => write!(f, "Finished"),
            Learnstate::NotStarted => write!(f, "Not started"),
            Learnstate::Learning(rem) => write!(f, "Learning ({rem})")
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
            Learnstate::Learning(rem) => {
                if rem<=0 {
                    return Err(CoreError::Learning("A learnitem with less then 1 remaining learn go entered.".to_string()))
                }
                if rem==1 { 
                    self.learnstate = Learnstate::Finished;
                } else {
                    self.learnstate = Learnstate::Learning(rem-1);
                }
                Ok(self)
            },
            Learnstate::Finished => Ok(self)
        }
    }
}
