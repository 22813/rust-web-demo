use models::prelude::*;
#[derive(Default, Debug)]
pub struct Account {
   pub id: i32,
   pub name: Option<String>,
   pub password: Option<String>,
}

impl Account {
    fn new(row:Row)->Self{
        let mut account = Account::default();
        account.id = row.get("id");
        account.name = row.get("name");
        account.password=row.get("password");
        account
    }
    pub fn get(name:Option<String>,password:Option<String>)->Option<Account>  {
        let conn=get_conn();
        for row in &conn.query("SELECT * from account where name=$1 and password=$2", &[&name,&password]).unwrap() {
            return Some(Self::new(row));
        }
        None
    }
}
