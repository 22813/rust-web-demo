pub mod task;
pub mod account;
pub mod prelude {
    pub use std::collections::BTreeMap;
    pub use rustc_serialize::json::{ToJson, Json};
    pub use chrono::*;
    pub use iron::prelude::*;
    pub use iron::typemap::Key;
    pub use postgres::rows::*;

    use r2d2::{Config,Pool, PooledConnection};
    use r2d2_postgres::{PostgresConnectionManager,SslMode};
    pub fn get_conn()->PooledConnection<PostgresConnectionManager>{
        let conn = POOL.get().unwrap();
        conn
    }
    lazy_static! {
        static ref POOL:Pool<PostgresConnectionManager>  = connect_pool(); 
    }
    fn connect_pool()->Pool<PostgresConnectionManager>{
        let manager = PostgresConnectionManager::new("postgres://postgres:123456@localhost:5432/mydb", SslMode::None).unwrap();
        let config = Config::builder().pool_size(10).build();
        let pool=Pool::new(config, manager).unwrap();
        println!("Connected to postgres with pool: {:?}", pool);
        pool
    }
}
