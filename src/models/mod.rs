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

