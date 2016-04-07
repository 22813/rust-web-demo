use postgres::rows::*;
use models::Account;

impl super::Row2Model for Account{
    fn convert(row:Row)->Account{
        let mut account = Account::default();
        account.id      = row.get("id");
        account.name    = row.get("name");
        account.password=row.get("password");
        account
    }
}

pub fn get(name:Option<String>,password:Option<String>)->Option<Account>  {
    super::find_one("SELECT * from account where name=$1 and password=$2", &[&name,&password])
}
