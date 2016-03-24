pub mod task;
pub mod account;
pub mod prelude {
    pub use std::collections::BTreeMap;
    pub use rustc_serialize::json::{ToJson, Json};
    pub use chrono::*;
    pub use iron::prelude::*;
    pub use iron::typemap::Key;

    pub use postgres::rows::*;
    pub use r2d2::{Config,Pool, PooledConnection};
    pub use r2d2_postgres::{PostgresConnectionManager,SslMode};
    pub type PostgresPool = Pool<PostgresConnectionManager>;
    pub type PostgresPooledConnection = PooledConnection<PostgresConnectionManager>;
    pub type Conn=PostgresPooledConnection;

    lazy_static! {
        pub static ref POOL:PostgresPool  = connect_pool(); 
    }
    fn connect_pool()->PostgresPool{
        let manager = PostgresConnectionManager::new("postgres://postgres:123456@localhost:5432/mydb", SslMode::None).unwrap();
        let config = Config::builder().pool_size(10).build();
        let pool=Pool::new(config, manager).unwrap();
        println!("Connected to postgres with pool: {:?}", pool);
        pool
    }
    pub fn get_conn()->Conn{
        let conn = POOL.get().unwrap();
        conn
    }
}
