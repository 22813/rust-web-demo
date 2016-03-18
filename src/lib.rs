extern crate postgres;
//extern crate rand;
extern crate rustc_serialize;

extern crate iron;
extern crate persistent;
extern crate router;
extern crate mount;
extern crate urlencoded;
extern crate staticfile;

extern crate r2d2;
extern crate r2d2_postgres;
extern crate time;
extern crate handlebars_iron;
extern crate term;
extern crate logger;
extern crate crypto;
extern crate hyper;
extern crate chrono;

use iron::prelude::*;

use router::Router;
use mount::Mount;
use staticfile::Static;

use std::net::*;
use std::path::Path;

use persistent::{Read};

use handlebars_iron::{HandlebarsEngine};

use r2d2::{Config,Pool};
use r2d2_postgres::{PostgresConnectionManager,SslMode};

use framework::{middleware,database};
use controllers::{user,task};

use logger::Logger;
use logger::format::Format;
use logger::format::FormatAttr::FunctionAttrs;
use term::Attr;

pub mod framework;
pub mod controllers;
pub mod models;
pub mod utils;

static FORMAT: &'static str = "@[red A]Uri: {uri}@, @[blue blink underline]Method: {method}@, @[yellow standout]Status: {status}@, @[brightgreen]Time: {response-time}@";


pub fn run(){
    let manager = PostgresConnectionManager::new("postgres://postgres:123456@localhost:5432/mydb", SslMode::None).unwrap();
    let config = Config::builder().pool_size(10).build();
    let pool=Pool::new(config, manager).unwrap();

    println!("Connected to postgres with pool: {:?}", pool);

    let mut router = Router::new();
    router.get("/user/env", user::env);
    router.get("/user/hits", user::hits);
    router.get("/user/list",user::list);
    router.get("/user/list-base64",user::list_base64);
    router.get("/user/list-aes",user::list_aes);


    router.get("/task/",task::list);
    router.get("/task/new",task::new);
    router.get("/task/:id",task::edit);
    router.get("/task/delete/:id",task::delete);
    router.post("/task/",task::save);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static", Static::new(Path::new("./src/static/")));

    let mut middleware = Chain::new(mount);
    //middleware.link(Write::<HitCounter>::both(0));
    middleware.link(Read::<database::AppDb>::both(pool));
    middleware.link_after(HandlebarsEngine::new("./src/templates", ".hbs"));

    middleware.link_before(middleware::MyMiddleware);
    middleware.link_after(middleware::MyMiddleware);
    middleware.link_around(middleware::MyMiddleware);

    fn attrs(req: &Request, _res: &Response) -> Vec<term::Attr> {
        match format!("{}", req.url).as_ref() {
            "/" => vec![Attr::Blink],
            _ => vec![]
        }
    }

    let format = Format::new(FORMAT, vec![], vec![FunctionAttrs(attrs)]);
    middleware.link(Logger::new(Some(format.unwrap())));
    
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("Listening on http://{}", host);
    Iron::new(middleware).http(host).unwrap();
}
