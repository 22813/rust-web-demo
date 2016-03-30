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
    Ok(response)
}
pub fn json_response(data:&str)->IronResult<Response>{
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(data);
    //let mut headers = Headers::new();
    //headers.set( ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
    //response.headers=headers; 
    response.headers.set(ContentType(Mime(TopLevel::Application,SubLevel::Json,vec![(Attr::Charset,Value::Utf8)])));
    //response.headers.set(Cookie(vec![CookiePair::new("foo".to_owned(),"bar".to_owned())]));
    Ok(response)
}
