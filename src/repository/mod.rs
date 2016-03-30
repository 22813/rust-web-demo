pub mod task;
pub mod account;

pub mod prelude {
    pub use postgres::rows::*;
    pub use postgres::types::ToSql;

    pub trait Row2Model{
        fn convert(row:Row)->Self;
    }
}
use std::path::Path;
use r2d2::{Config,Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager,SslMode};
use self::prelude::*;
use config::reader;



fn get_conn()->PooledConnection<PostgresConnectionManager>{
    let conn = POOL.get().unwrap();
    conn
}
lazy_static! {
    static ref POOL:Pool<PostgresConnectionManager>  = connect_pool(); 
}
fn connect_pool()->Pool<PostgresConnectionManager>{
    let config = reader::from_file(Path::new("./web-root/config/web.conf")).unwrap();
    let host = config.lookup_str("database.host").unwrap();
    let port = config.lookup_str("database.port").unwrap();
    let user_name = config.lookup_str("database.user_name").unwrap();
    let password = config.lookup_str("database.password").unwrap();
    let db_name = config.lookup_str("database.db_name").unwrap();

    let connect_str=format!("postgres://{}:{}@{}:{}/{}",user_name,password,host,port,db_name);
    let manager = PostgresConnectionManager::new(connect_str.as_str(), SslMode::None).unwrap();

    let config = Config::builder().pool_size(10).build();
    let pool=Pool::new(config, manager).unwrap();
    println!("Connected to postgres with pool: {:?}", pool);
    pool
}

pub fn find_one<T>(query: &str, params: &[&ToSql])->Option<T> where T:Row2Model{
    let conn=get_conn();
    for row in &conn.query(query, params).unwrap() {
        return Some(T::convert(row));
    }
    None
}

pub fn find_list<T>(query: &str, params: &[&ToSql])->Vec<T> where T:Row2Model{
    let mut result: Vec<T> = vec![];
    let conn=get_conn();
    for row in &conn.query(query, params).unwrap() {
        result.push(T::convert(row));
    }
    result
}

// return: the number of rows modified
pub fn execute(query: &str, params: &[&ToSql])->u64{
    let conn=get_conn();
    conn.execute(query,params).unwrap()
}
