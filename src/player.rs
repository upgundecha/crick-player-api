use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: Option<i32>,
    pub name: String,
    pub country: String,
    pub speciality: String
}