
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};
use iron::typemap::Key;
//
// // Types
//
pub type PostgresPool = Pool<PostgresConnectionManager>;
pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;

pub struct AppDb;

impl Key for AppDb { type Value = PostgresPool; }

