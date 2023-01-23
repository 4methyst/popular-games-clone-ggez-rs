use ggez::{
    Context,
};
use std::{fs, io::Write};
// use serde; //::{Serialize, Deserialize};
use ron::{ser, de};
use crate::game::entity::Score;

pub struct LeaderBoard {
    scores: Vec<Score>
}

impl LeaderBoard {
    pub fn new(ctx: &Context) -> Self {
        let str = fs::read_to_string("./saves/scores.ron");
        let scores: Vec<Score> = de::from_str(&str).unwrap();
        LeaderBoard {
            scores,
        }
    }
}