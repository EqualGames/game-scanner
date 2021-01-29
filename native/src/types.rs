use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  pub(crate) _type: String,
  pub(crate) id: String,
  pub(crate) name: String,
  pub(crate) path: String,
  pub(crate) launch_command: String,
}