use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use chrono::*;
#[derive(Default, Debug)]
pub struct Account {
    pub id          : i32,
    pub name        : Option<String>,
    pub password    : Option<String>,
}

#[derive(Default, Debug)]
pub struct Task {
    pub id              : i32,
    pub name            : Option<String>,
    pub content         : Option<String>,
    pub create_time     : Option<DateTime<Local>>,
    pub update_time     : Option<DateTime<Local>>,
    pub status          : i32,//0:new,1:ongoing,2:finished,3:canceld
}

impl ToJson for Task {
    fn to_json(&self) -> Json {
        let mut m: BTreeMap<String, Json> = BTreeMap::new();
        m.insert("id".to_string(), self.id.to_json());
        m.insert("name".to_string(), self.name.to_json());
        m.insert("content".to_string(), self.content.to_json());
        if let Some(dt)=self.create_time{
            m.insert("create_time".to_string(),dt.format("%Y-%m-%d %H:%M:%S").to_string().to_json());
        }
        if let Some(dt)=self.update_time{
            m.insert("update_time".to_string(),dt.format("%Y-%m-%d %H:%M:%S").to_string().to_json());
        }
        m.insert("status".to_string(), self.status.to_json());
        m.to_json()
    }
}
