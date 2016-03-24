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
extern crate iron_login;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod controllers;
pub mod repository;
pub mod models;
pub mod utils;
pub mod services;


pub fn run(){
    use controllers;
    use iron::prelude::*;
    use std::net::*;
    let chain=controllers::prelude::get_chain();
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("Listening on http://{}", host);
    Iron::new(chain).http(host).unwrap();
}
