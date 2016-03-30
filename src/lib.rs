extern crate postgres;
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
extern crate handlebars_iron as hbs;
extern crate term;
extern crate logger;
extern crate crypto;
extern crate hyper;
extern crate chrono;
extern crate iron_login;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate config;

pub mod controllers;
pub mod repository;
pub mod models;
pub mod utils;
pub mod services;
pub mod schedule;


pub fn run(){
    use iron::prelude::*;
    use std::path::Path;
    use std::net::*;
    use config::reader;
    use controllers;
    use schedule;
    schedule::init();
    let config = reader::from_file(Path::new("./web-root/config/web.conf")).unwrap();
    let port = config.lookup_integer32("web.listen.port").unwrap();

    let chain=controllers::get_chain();
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port as u16);
    println!("Listening on http://{}", host);
    Iron::new(chain).http(host).unwrap();
}
