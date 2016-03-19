use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Url, status};
use rustc_serialize::json::{ToJson};
use handlebars_iron::{Template};

pub fn redirect(req:&Request,path:&str)->IronResult<Response>{
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}{}",url.scheme,url.host,url.port,path).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}   
pub fn template<T: ToJson>(name: &str, value: T) ->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(Template::new(name,value));
    response.set_mut(status::Ok);
    Ok(response)
}
