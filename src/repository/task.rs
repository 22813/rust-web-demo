use repository::prelude::*;
use models::*;

const DELETE_SQL:&'static str="delete from task where id=$1";
const LIST_SQL:&'static str="SELECT * from task order by id desc";
const GET_SQL:&'static str="SELECT * from task where id=$1";
const UPDATE_SQL:&'static str="update task set name=$1,content=$2,update_time=$3,status=$4 where id=$5";
const CREATE_SQL:&'static str="insert into task(name,content,create_time,update_time,status) values($1,$2,$3,$4,$5)";

fn new(row:Row)->Task{
    let mut task = Task::default();
    task.id = row.get("id");
    task.name = row.get("name");
    task.content=row.get("content");
    task.create_time=row.get("create_time");
    task.update_time=row.get("update_time");
    task.status=row.get("status");
    task
}
pub fn list() -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];
    let conn=get_conn();
    for row in &conn.query(LIST_SQL, &[]).unwrap() {
        tasks.push(new(row));
    }
    tasks
}

pub fn get(id:i32) -> Option<Task> {
    let conn=get_conn();
    for row in &conn.query(GET_SQL, &[&id]).unwrap() {
        return Some(new(row));
    } None 
}

pub fn delete(id:i32){
    let conn=get_conn();
    conn.execute(DELETE_SQL,&[&id]).unwrap();
}
pub fn save(task:&Task){
    let conn=get_conn();
    if task.id>0 {
        conn.execute(UPDATE_SQL,&[&task.name,&task.content,&task.update_time,&task.status,&task.id]).unwrap();
    }else{
        conn.execute(CREATE_SQL,&[&task.name,&task.content,&task.create_time,&task.update_time,&task.status]).unwrap(); }   
}
