use repository::prelude::*;
use models::*;

fn new(row:Row)->Account{
    let mut account = Account::default();
    account.id = row.get("id");
    account.name = row.get("name");
    account.password=row.get("password");
    account
}
pub fn get(name:Option<String>,password:Option<String>)->Option<Account>  {
    let conn=get_conn();
    for row in &conn.query("SELECT * from account where name=$1 and password=$2", &[&name,&password]).unwrap() {
        return Some(new(row));
    }
    None
}
