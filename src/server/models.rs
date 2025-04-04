use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub gamer_id: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Leaderboard {
    pub gamer_id: String,
    pub high_score: i32,
    pub time: String,
}