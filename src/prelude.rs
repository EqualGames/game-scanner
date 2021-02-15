use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Game {
    pub _type: String,
    pub id: String,
    pub name: String,
    pub path: String,
    pub launch_command: Vec<String>,
}
