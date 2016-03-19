use iron::prelude::*;
use iron_login::User;
use iron::modifiers::Redirect;
use iron::{Url, status};

use utils::{request,response};

#[derive(Debug)]
struct Account(String);
impl Account {
    fn new(user_id: &str) -> Account {
        Account(user_id.to_owned())
    }
}
impl User for Account {
    fn from_user_id(_: &mut Request, user_id: &str) -> Option<Account> {
        Some(Account(user_id.to_owned()))
    }
    fn get_user_id(&self) -> &str {
        &self.0
    }
}
pub fn logout(req: &mut Request) -> IronResult<Response> {
    let login = Account::get_login(req);
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}/account/login/",url.scheme,url.host,url.port).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))).set(login.log_out()))
}

pub fn check_login(req:&mut Request)->bool{
    let login = Account::get_login(req);
    let user=login.get_user();
    match user{
        None=>false,
        _=>true,
    } 
}

pub fn login(_: &mut Request) -> IronResult<Response> {
    response::template("account-login","".to_owned())
}

pub fn do_login(req: &mut Request) -> IronResult<Response> {
    let login = Account::get_login(req);
    let name=request::get_form_param(req,"name");
    //let password=request::get_form_param(req,"password");
    let ref url=req.url;
    if let Some(name)=name{ 
        //return Ok(Response::new() .set(::iron::status::Ok) .set(format!("User set to '{}'", name)).set(login.log_in(Account::new(&name))))
        let url = Url::parse(format!("{}://{}:{}/task/",url.scheme,url.host,url.port).as_str()).unwrap();
        let response=Response::with((status::Found, Redirect(url.clone()))).set(login.log_in(Account::new(&name)));
        return Ok(response);
    }
    response::redirect(req,"/account/login/")
}
