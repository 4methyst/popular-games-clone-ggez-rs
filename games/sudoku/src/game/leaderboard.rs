use std::time::Duration;
use serde::{Serialize, Deserialize};
use super::entity::Difficulty;

#[derive(Serialize, Deserialize)]
pub struct score {
    name: String,
    difficulty: Difficulty,
    time: Duration,
}