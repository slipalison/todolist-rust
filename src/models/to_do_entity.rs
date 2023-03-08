use uuid::Uuid;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{status_enum::Status, to_do_update_command::ToDoUpdateCommand, to_do_crate_command::ToDoCrateCommand};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToDoEntity {
    pub id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    pub status: Status,
}

impl ToDoEntity {
     pub fn create(commad:ToDoCrateCommand ) -> ToDoEntity {
        ToDoEntity::new(commad.name, commad.deadline, None, None)          
     }

     pub fn update(commad:ToDoUpdateCommand,  id: Uuid ) -> ToDoEntity {
        ToDoEntity::new(commad.name, commad.deadline, commad.status, Some(id))          
     }

    fn new(name: String, deadline: Option<DateTime<Utc>>, status: Option<Status>, id:Option<Uuid>) -> Self{
        ToDoEntity {
            id: match id {
                Some(i)=> i,
                None=> Uuid::new_v4()
            } ,
            deadline: deadline.clone(),
            name: name.clone(),
            status: match status {
                Some(s) => s,
                None => Status::ToDo                
            }
        }
    }
}
