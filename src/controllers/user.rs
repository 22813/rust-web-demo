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

pub struct UserController;

pub struct HitCounter;
impl Key for HitCounter{type Value=usize;}

impl UserController{
    pub fn new()->Self{
        UserController
    }
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

    pub fn list(req:&mut Request)->IronResult<Response>{
        let conn = req.get::<Read<AppDb>>().unwrap().get().unwrap();
        let users=User::list_all(conn);
        println!("{:?}",users);
        let mut data = BTreeMap::new();
        data.insert("users".to_string(), users.to_json());
        let encoded = json::encode(&data).unwrap();
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut(encoded);
        Ok(response)
    }
}
