use repository::account as repos;
use models::*;

pub fn get(name:Option<String>,password:Option<String>)->Option<Account>  {
   repos::get(name,password)
}
