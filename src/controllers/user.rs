use std::env;
use std::collections::BTreeMap;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson};

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use persistent::{Write,Read};

use models::user::User;
use framework::database::AppDb;
use utils::crypto;
use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

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
