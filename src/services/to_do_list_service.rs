use std::sync::{Mutex, Arc};

use crate::models::{to_do_crate_command::ToDoCrateCommand, to_do_entity::ToDoEntity};

use lazy_static::lazy_static;


lazy_static! {
    static ref ITEMS: Mutex<Vec<ToDoEntity>> = Mutex::new(Vec::new());
}

//type MyVec = Arc<Mutex<Vec<ToDoEntity>>>;

// lazy_static! {
//     pub static ref ToDoList: Vec<ToDoEntity> = vec![]; 
// }

pub fn Add(item: &ToDoCrateCommand) -> bool {
    let mut items = ITEMS.lock().unwrap();

    //items.push(ToDoEntity::create(*item.name, *item.deadline));

    true
}
