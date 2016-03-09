use r2d2::PooledConnection;
use r2d2_postgres::PostgresConnectionManager;


use std::collections::BTreeMap;

use rustc_serialize::json::{ToJson, Json};

#[derive(Default, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub remember_token: String,
}

impl ToJson for User {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("id".to_string(), self.id.to_json());
        m.insert("name".to_string(), self.name.to_json());
        m.insert("email".to_string(), self.email.to_json());
        m.insert("remember_token".to_string(), self.remember_token.to_json());
        m.to_json()
    }
}

impl User {

    pub fn list_all(conn: PooledConnection<PostgresConnectionManager>) -> Vec<User> {
        let mut users: Vec<User> = vec![];
        for row in &conn.query("SELECT * from t_user", &[]).unwrap() {
            let mut user = User::default();
            user.id = row.get("id");
            user.name = row.get("name");
            user.email=row.get("email");
            user.remember_token=row.get("remember_token");
            users.push(user);
        }
        users
    }
/*
    pub fn find_by_id(id: i32, conn: &Connection) -> User {
        let stmt = conn.prepare("SELECT * FROM users WHERE id = (").unwrap();
        let mut user = User::default();
        for row in stmt.query(&[&id]).unwrap() {
            user.id = row.get(0);
        }
        user
    }
    */
}
