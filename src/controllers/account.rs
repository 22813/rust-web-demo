use super::prelude::*;
use iron_login::User;
use services::account as service;
use std::sync::{Once, ONCE_INIT};

static START: Once = ONCE_INIT;
pub fn init_router(router:&mut Router){
    START.call_once(|| {
        router.get("/account/login/",|_:&mut Request|response::template("account/login",()));

        router.post("/account/login/",|req: &mut Request|{
            let login = MyUser::get_login(req);
            let name=req.get_form_param("name");
            let password=req.get_form_param("password");
            if let Some(account)=service::get(name,password) {
                let mut response = Response::new().set(login.log_in(MyUser::new(&format!("{}",&account.id))));
                response.set_mut(Template::new("index","".to_owned()));
                response.set_mut(status::Ok);
                return Ok(response)
            }
            let mut data = BTreeMap::new();
            data.insert("error".to_string(),true);
            response::template("account-login",data)
        });

        router.get("/account/logout/",|req: &mut Request|{
            let login = MyUser::get_login(req);
            let ref url=req.url;
            let url = Url::parse(format!("{}://{}:{}/account/login/",url.scheme,url.host,url.port).as_str()).unwrap();
            Ok(Response::with((status::Found, Redirect(url.clone()))).set(login.log_out()))
        });
    });
}
pub fn check_login(req:&mut Request)->bool{
    let login = MyUser::get_login(req);
    let user=login.get_user();
    match user{
        None=>false,
        _=>true,
    } 
}

#[derive(Debug)]
struct MyUser(String);
impl MyUser {
    fn new(user_id: &str) -> MyUser {
        MyUser(user_id.to_owned())
    }
}
impl User for MyUser {
    fn from_user_id(_: &mut Request, user_id: &str) -> Option<MyUser> {
        Some(MyUser(user_id.to_owned()))
    }
    fn get_user_id(&self) -> String {
        let mut s=String::new();
        s.push_str(&self.0);
        s
    }
}
