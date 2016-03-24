use models::*;
use repository::task as repos;

pub fn list() -> Vec<Task> {
    repos::list()
}

pub fn get(id:i32) -> Option<Task> {
    repos::get(id)
}

pub fn delete(id:i32){
    repos::delete(id);
}
pub fn save(task:&Task){
    repos::save(task);
}
