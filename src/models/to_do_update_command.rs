use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::status_enum::Status;

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDoUpdateCommand {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}


impl ToDoUpdateCommand {

    pub fn validate(&self) -> Result<bool, String> {
        let len = self.name.trim().chars().count();
        if len < 3 || len > 60 {
            return Err("O nome deve conter entre 3 e 60 letras".to_string());
        }

        if self.esta_atrasada() {
            return Err("A data de termino não pode der menor que a data atual".to_string());
        }

        Ok(true)
    }
    fn esta_atrasada(&self) -> bool {
        let agora = Utc::now();
        match self.deadline {
            Some(deadline) => deadline < agora,
            None => false,
        }
    }
}



