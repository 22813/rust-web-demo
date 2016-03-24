use controllers::prelude::*;
use services::account as service;

pub fn init_router(router:&mut Router){
    router.post("/account/login/",do_login);
    router.get("/account/login/",login);
    router.get("/account/logout/",logout);
}

pub fn logout(req: &mut Request) -> IronResult<Response> {
    let login = MyUser::get_login(req);
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}/account/login/",url.scheme,url.host,url.port).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))).set(login.log_out()))
}

pub fn check_login(req:&mut Request)->bool{
    let login = MyUser::get_login(req);
    let user=login.get_user();
    match user{
        None=>false,
        _=>true,
    } 
}

pub fn login(_: &mut Request) -> IronResult<Response> {
    let mut data = BTreeMap::new();
    data.insert("error".to_string(),"".to_owned());
    response::template("account-login",data)
}

pub fn do_login(req: &mut Request) -> IronResult<Response> {
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
    fn get_user_id(&self) -> &str {
        &self.0
    }
}
