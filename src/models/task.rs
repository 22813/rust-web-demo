use framework::database::PostgresPooledConnection;
use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use postgres::rows::*;
use chrono::*;


#[derive(Default, Debug)]
pub struct Task {
   pub id: i32,
   pub name: Option<String>,
   pub content: Option<String>,
   pub create_time:Option<DateTime<Local>>,
   pub update_time:Option<DateTime<Local>>,
   pub status: i32,//0:new,1:ongoing,2:finished,3:canceld
}


impl ToJson for Task {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("id".to_string(), self.id.to_json());
        m.insert("name".to_string(), self.name.to_json());
        m.insert("content".to_string(), self.content.to_json());
        if let Some(dt)=self.create_time{
            m.insert("create_time".to_string(),dt.format("%Y-%m-%d %H:%M:%S").to_string().to_json());
        }
        if let Some(dt)=self.update_time{
            m.insert("update_time".to_string(),dt.format("%Y-%m-%d %H:%M:%S").to_string().to_json());
        }
        m.insert("status".to_string(), self.status.to_json());
        m.to_json()
    }
}
impl Task {
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
    pub fn list(conn: PostgresPooledConnection) -> Vec<Task> {
        let mut tasks: Vec<Task> = vec![];
        for row in &conn.query("SELECT * from task order by id desc", &[]).unwrap() {
            tasks.push(Self::new(row));
        }
        tasks
    }
    
    pub fn get(conn: PostgresPooledConnection,id:i32) -> Option<Task> {
        for row in &conn.query("SELECT * from task where id=$1", &[&id]).unwrap() {
            return Some(Self::new(row));
        }
        None 
    }
    
    pub fn delete(conn:PostgresPooledConnection,id:i32){
        conn.execute("delete from task where id=$1",&[&id]).unwrap();
    }
    pub fn save(conn:PostgresPooledConnection,task:&Task){
        if task.id>0 {
            conn.execute("update task set name=$1,content=$2,update_time=$3,status=$4 where id=$5",&[&task.name,&task.content,&task.update_time,&task.status,&task.id]).unwrap();
        }else{
            conn.execute("insert into task(name,content,create_time,update_time,status) values($1,$2,$3,$4,$5)",&[&task.name,&task.content,&task.create_time,&task.update_time,&task.status]).unwrap();
        }   
    }
}
