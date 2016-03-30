use super::prelude::*;
use models::Task;

const DELETE_SQL    :&'static str="delete from task where id=$1";
const LIST_SQL      :&'static str="SELECT * from task order by id desc";
const GET_SQL       :&'static str="SELECT * from task where id=$1";
const UPDATE_SQL    :&'static str="update task set name=$1,content=$2,update_time=$3,status=$4 where id=$5";
const CREATE_SQL    :&'static str="insert into task(create_time,name,content,update_time,status) values($1,$2,$3,$4,$5)";

impl Row2Model for Task {
    fn convert(row:Row)->Task{
        let mut task = Task::default();
        task.id             = row.get("id");
        task.name           = row.get("name");
        task.content        =row.get("content");
        task.create_time    =row.get("create_time");
        task.update_time    =row.get("update_time");
        task.status         =row.get("status");
        task
    }
}
pub fn list() -> Vec<Task> {
    super::find_list(LIST_SQL,&[])
}

pub fn get(id:i32) -> Option<Task> {
    super::find_one(GET_SQL, &[&id])
}
pub fn delete(id:i32){
    super::execute(DELETE_SQL,&[&id]);
}
pub fn save(task:&Task){
    let params:&[&ToSql]=&[&task.create_time,&task.name,&task.content,&task.update_time,&task.status,&task.id];
    if task.id>0 {
        super::execute(UPDATE_SQL,&params[1..]);
    }else{
        super::execute(CREATE_SQL,&params[..5]);
    }   
}
