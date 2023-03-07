use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoUpdateCommand {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl ToDoUpdateCommand {
    pub fn new(
        name: String,
        deadline: Option<DateTime<Utc>>,
        status: Option<Status>,
    ) -> ToDoUpdateCommand {
        ToDoUpdateCommand {
            name: name,
            deadline: deadline,
            status: status,
        }
    }
}



