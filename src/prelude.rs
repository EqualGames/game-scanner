use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    pub _type: String,
    pub id: String,
    pub name: String,
    pub path: String,
    pub launch_command: Vec<String>,
}
