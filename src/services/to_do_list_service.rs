use std::sync::Mutex;
use uuid::Uuid;

use crate::models::{
    to_do_crate_command::ToDoCrateCommand, to_do_entity::ToDoEntity,
    to_do_update_command::ToDoUpdateCommand,
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ITEMS: Mutex<Vec<ToDoEntity>> = Mutex::new(Vec::new());
}

pub fn add_item(item: ToDoCrateCommand) -> Result<Option<ToDoEntity>, &'static str> {
    let entity = ToDoEntity::create(item);
    ITEMS.lock().unwrap().push(entity);
    Ok(get_last_item())
}

pub fn get_items() -> Result<Vec<ToDoEntity>, String> {
    let items = ITEMS.lock().unwrap();
    let cloned_items = items.clone();
    Ok(cloned_items)
}

fn get_last_item() -> Option<ToDoEntity> {
    let items = ITEMS.lock().unwrap();
    items.last().cloned()
}

pub fn update_item(id: Uuid, commad: ToDoUpdateCommand) -> Result<ToDoEntity, &'static str> {
    let mut items = ITEMS.lock().unwrap();

    for item in items.iter_mut() {
        if item.id == id {
            let entity = ToDoEntity::update(commad, id);

            item.name = entity.name;
            item.status = entity.status;
            item.deadline = entity.deadline;

            return Ok(item.clone());
        }
    }

    Err("Item not found")
}

pub fn delete_item(id: &Uuid) -> Option<ToDoEntity> {
    let mut items = ITEMS.lock().unwrap();
    if let Some(index) = items.iter().position(|x| x.id == *id) {
        Some(items.remove(index))
    } else {
        None
    }
}
