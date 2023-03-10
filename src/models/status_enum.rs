use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}