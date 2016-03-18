use iron::prelude::*;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};
use iron::typemap::Key;
use persistent::{Read};
//
// // Types
//
pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

pub struct AppDb;

impl Key for AppDb { type Value = PostgresPool; }

pub fn get_conn(req:&mut Request)->PostgresPooledConnection{
    let conn = req.get::<Read<AppDb>>().unwrap().get().unwrap();
    conn
}
