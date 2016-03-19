use std::collections::BTreeMap;
use std::cell::RefCell;
use std::str::FromStr;
use rustc_serialize::json::{ToJson};

use iron::prelude::*;
use router::Router;
use handlebars_iron::{Template};

use framework::database;
use urlencoded::{UrlEncodedBody};
//use urlencoded::{UrlEncodedBody,UrlEncodedQuery};
use chrono::*;
use models::task::Task;
use iron::modifiers::Redirect;
use iron::{Url, status};

pub fn list(req: &mut Request) -> IronResult<Response> {
    let conn=database::get_conn(req);
    let tasks=Task::list(conn);
    let mut data = BTreeMap::new();
    data.insert("tasks".to_string(), tasks.to_json());
    // let data="".to_owned();
    let mut response = Response::new();
    response.set_mut(Template::new("task-list", data));
    response.set_mut(status::Ok);
    Ok(response)
}

pub fn new(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(Template::new("task-new","".to_owned()));
    response.set_mut(status::Ok);
    Ok(response)
}
pub fn delete(req: &mut Request) -> IronResult<Response> {
    let id=get_path_param(req,"id").unwrap_or("0".to_owned());
    let id=i32::from_str(&*id).unwrap_or(0);
    if id>0{
        Task::delete(database::get_conn(req),id);
    }
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}/task/",url.scheme,url.host,url.port).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}
pub fn edit(req: &mut Request) -> IronResult<Response> {
    let id=get_path_param(req,"id").unwrap_or("0".to_owned());
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
    let name=get_form_param(req,"name"); 
    let content=get_form_param(req,"content");
    let status=get_form_param(req,"status").unwrap_or("0".to_owned());
    let time:DateTime<Local>=Local::now();
    let id=get_form_param(req,"id").unwrap_or("0".to_owned());
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
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}/task/",url.scheme,url.host,url.port).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}


fn get_form_param(req:&mut Request,name:&str)->Option<String>{
    let query=RefCell::new(req.get_ref::<UrlEncodedBody>()).into_inner();
    //let query=RefCell::new(req.get_ref::<UrlEncodedQuery>()).into_inner();
    if let Ok(ref hashmap)=query{
        if let Some(vec)=hashmap.get(name){
            if vec.len()>0{
                let value=&vec[0];
                if value.len()>0{
                    let mut result=String::new();
                    result.push_str(value.as_str());
                    return Some(result);
                }
            }
        }
    }
    None
}

fn get_path_param(req:&mut Request,name:&str)->Option<String>{
    let ref value = req.extensions.get::<Router>().unwrap().find(name).unwrap_or("");
    if value.len()>0 {
        return Some(String::from(*value));
    }
    None
}



















/*
pub struct HitCounter;
impl Key for HitCounter{type Value=usize;}

fn compose_response(data:&str)->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(data);
    //let mut headers = Headers::new();
    //headers.set( ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
    //response.headers=headers; 
    response.headers.set(ContentType(Mime(TopLevel::Application,SubLevel::Json,vec![(Attr::Charset,Value::Utf8)])));
    //response.headers.set(Cookie(vec![CookiePair::new("foo".to_owned(),"bar".to_owned())]));
    Ok(response)
}

pub fn env(_: &mut Request) -> IronResult<Response> {
    let powered_by:String = match env::var("POWERED_BY") {
        Ok(val) => val,
        Err(_) => "Iron".to_string()
    };    
    let message = format!("Powered by: {}, pretty cool aye", powered_by);
    Ok(Response::with((status::Ok, message)))
}



pub fn hits(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<HitCounter>>().unwrap();
    let mut count = mutex.lock().unwrap();
    *count += 1;
    Ok(Response::with((status::Ok, format!("Hits: {}", *count))))
} 
fn get_json_data(req:&mut Request)->String{
    let conn = req.get::<Read<AppDb>>().unwrap().get().unwrap();
    let users=User::list_all(conn);
    println!("users:{:?}",users);
    println!("users.to_json():{:?}",users.to_json());
    let mut data = BTreeMap::new();
    data.insert("users".to_string(), users.to_json());
    let data = json::encode(&data).unwrap();
    data 
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
pub fn list(req:&mut Request)->IronResult<Response>{
    let data =get_json_data( req);
    compose_response(&data)
}
*/
