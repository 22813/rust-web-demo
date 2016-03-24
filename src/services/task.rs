use models::*;
use dao::task as task_dao;

pub fn list() -> Vec<Task> {
    task_dao::list()
}

pub fn get(id:i32) -> Option<Task> {
    task_dao::get(id)
}

pub fn delete(id:i32){
    task_dao::delete(id);
}
pub fn save(task:&Task){
    task_dao::save(task);
}
