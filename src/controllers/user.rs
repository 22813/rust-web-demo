use std::env;
use std::collections::BTreeMap;

use rustc_serialize::json;
use rustc_serialize::json::{ToJson};

use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::{Write,Read};
use handlebars_iron::{Template};

use models::user::User;
use framework::database::AppDb;
use utils::crypto;
use urlencoded::UrlEncodedQuery;
pub struct UserController;

pub struct HitCounter;
impl Key for HitCounter{type Value=usize;}

impl UserController{
    pub fn env(_: &mut Request) -> IronResult<Response> {
        let powered_by:String = match env::var("POWERED_BY") {
            Ok(val) => val,
            Err(_) => "Iron".to_string()
        };    
        let message = format!("Powered by: {}, pretty cool aye", powered_by);
        Ok(Response::with((status::Ok, message)))
    }

    pub fn handlebars(req: &mut Request) -> IronResult<Response> {
        let conn = req.get::<Read<AppDb>>().unwrap().get().unwrap();
        let users=User::list_all(conn);
        println!("{:?}",users);
        let mut data = BTreeMap::new();
        data.insert("users".to_string(), users.to_json());
        let mut response = Response::new();
        response.set_mut(Template::new("index", data));
        response.set_mut(status::Ok);
        Ok(response)
    }

    pub fn urlencoded(req: &mut Request) -> IronResult<Response> {
        match req.get_ref::<UrlEncodedQuery>() {
            Ok(ref hashmap) =>{
                println!("Parsed GET request query string:\n {:?}", hashmap);
                return Ok(Response::with((status::Ok, format!("params: {:?}", hashmap)))) ;
            },
            Err(ref e) => println!("{:?}", e)
        };
        Ok(Response::with((status::Ok, "hello" )))

    }
    pub fn posts(req: &mut Request) -> IronResult<Response> {
        let ref post_id = req.extensions.get::<Router>().unwrap().find("post_id").unwrap_or("none");
        Ok(Response::with((status::Ok, "PostId: {}", *post_id)))
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
        println!("{:?}",users);
        let mut data = BTreeMap::new();
        data.insert("users".to_string(), users.to_json());
        let data = json::encode(&data).unwrap();
        data 
    }
    fn get_response(data:&str)->IronResult<Response>{
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut(data);
        Ok(response)
    }
    pub fn list_aes(req:&mut Request)->IronResult<Response>{
        let data =Self::get_json_data(req);
        let data=crypto::aes_encrypt_string(&data);
        let data=crypto::base64_encode_bytes(&data.ok().unwrap());
        let data=data.expect("");
        Self::get_response(&data)
    }
    pub fn list_base64(req:&mut Request)->IronResult<Response>{
        let data =Self::get_json_data( req);
        let data=crypto::base64_encode_string(&data).expect("");
        Self::get_response(&data)
    }
    pub fn list(req:&mut Request)->IronResult<Response>{
        let data =Self::get_json_data( req);
        Self::get_response(&data)
    }
}
