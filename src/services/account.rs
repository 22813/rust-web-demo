use dao::account as account_dao;
use models::*;

pub fn get(name:Option<String>,password:Option<String>)->Option<Account>  {
   account_dao::get(name,password)
}
