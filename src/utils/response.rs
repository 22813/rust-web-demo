use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Url, status};
use rustc_serialize::json::{ToJson};
use hbs::{Template};
use hyper::header::{ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

pub fn redirect(req:&Request,path:&str)->IronResult<Response>{
    let ref url=req.url;
    let url = Url::parse(format!("{}://{}:{}{}",url.scheme,url.host,url.port,path).as_str()).unwrap();
    Ok(Response::with((status::Found, Redirect(url.clone()))))
}   
pub fn template<T: ToJson>(name: &str, value: T) ->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(Template::new(name,value));
    response.set_mut(status::Ok);
    response.headers.set(ContentType(Mime(TopLevel::Text,SubLevel::Html,vec![(Attr::Charset,Value::Utf8)])));
    Ok(response)
}
pub fn ok_json(data:&str)->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(status::Ok).set_mut(data);
    response.headers.set(ContentType(Mime(TopLevel::Application,SubLevel::Json,vec![(Attr::Charset,Value::Utf8)])));
    Ok(response)
}
pub fn ok(data:&str)->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(status::Ok).set_mut(data);
    response.headers.set(ContentType(Mime(TopLevel::Text,SubLevel::Plain,vec![(Attr::Charset,Value::Utf8)])));
    Ok(response)
}
