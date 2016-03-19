use std::collections::BTreeMap;
use std::str::FromStr;
use rustc_serialize::json::{ToJson};

use iron::prelude::*;
use handlebars_iron::{Template};

use framework::database;
//use urlencoded::{UrlEncodedBody,UrlEncodedQuery};
use chrono::*;
use models::task::Task;
use iron::{status};

use utils::{response};
use utils::request::*;
pub fn list(req: &mut Request) -> IronResult<Response> {
    let conn=database::get_conn(req);
    let tasks=Task::list(conn);
    let mut data = BTreeMap::new();
    data.insert("tasks".to_string(), tasks.to_json());
    response::template("task-list",data)
}

pub fn new(_: &mut Request) -> IronResult<Response> {
    response::template("task-new","".to_owned())
}
pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id=req.get_path_param("id").unwrap_or("0".to_owned());
    let id=i32::from_str(&*id).unwrap_or(0);
    if id>0{
        Task::delete(database::get_conn(req),id);
    }
    response::redirect(req,"/task/")
}
pub fn edit(req: &mut Request) -> IronResult<Response> {
    let id=req.get_path_param("id").unwrap_or("0".to_owned());
    let id=i32::from_str(&*id).unwrap_or(0);
    let mut response = Response::new();
    response.set_mut(status::Ok);
    if id>0{
        let task=Task::get(database::get_conn(req),id);
        if let Some(task)=task {
            let mut data = BTreeMap::new();
            data.insert("task".to_string(), task.to_json());
            response.set_mut(Template::new("task-edit", data));
        }
    }
    Ok(response)
}
pub fn save(req: &mut Request) -> IronResult<Response> {
    let name=req.get_form_param("name"); 
    let content=req.get_form_param("content");
    let status=req.get_form_param("status").unwrap_or("0".to_owned());
    let time:DateTime<Local>=Local::now();
    let id=req.get_form_param("id").unwrap_or("0".to_owned());
    let task=Task{
        id:             i32::from_str(&*id).unwrap_or(0),
        name:           name,
        content:        content,
        create_time:    Some(time),
        update_time:    Some(time),
        status:         i32::from_str(&*status).unwrap_or(0),
    };
    println!("saving task:{:?}",&task);
    Task::save(database::get_conn(req),&task);
    response::redirect(req,"/task/")
}




/*
pub struct HitCounter;
impl Key for HitCounter{type Value=usize;}
pub fn hits(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<HitCounter>>().unwrap();
    let mut count = mutex.lock().unwrap();
    *count += 1;
    Ok(Response::with((status::Ok, format!("Hits: {}", *count))))
} 
pub fn list_aes(req:&mut Request)->IronResult<Response>{
    let data =get_json_data(req);
    let data=crypto::aes_encrypt_string(&data);
    let data=crypto::base64_encode_bytes(&data.ok().unwrap());
    let data=data.expect("");
    compose_response(&data)
}
pub fn list_base64(req:&mut Request)->IronResult<Response>{
    let data =get_json_data( req);
    let data=crypto::base64_encode_string(&data).expect("");
    compose_response(&data)
}
*/
