use iron::{AroundMiddleware,BeforeMiddleware,AfterMiddleware,Handler,typemap};
use iron::prelude::*;
use time;
use controllers::account;
use iron::{Url, status};
use iron::modifiers::Redirect;
pub struct MyMiddleware;

impl typemap::Key for MyMiddleware{type Value=u64;}

impl BeforeMiddleware for MyMiddleware{
    fn before(&self,req:&mut Request)->IronResult<()>{
        req.extensions.insert::<MyMiddleware>(time::precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for MyMiddleware{
    fn after(&self ,req: &mut Request,res:Response)->IronResult<Response>{
        if req.url.path.join("/").contains("task"){
            let delta=time::precise_time_ns()-*req.extensions.get::<MyMiddleware>().unwrap();
            println!("Request url:{}, took: {} ms",req.url.path.join("/"),(delta as f64)/1000000.0);
        }
        Ok(res)
    }
}


impl AroundMiddleware for MyMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        struct LoggerHandler<H: Handler> {  handler: H }
        impl<H: Handler> Handler for LoggerHandler<H> {
            fn handle(&self, req: &mut Request) -> IronResult<Response> {
                if account::check_login(req) || req.url.path.join("/").contains("account") {
                    let entry = time::precise_time_ns();
                    let res = self.handler.handle(req);
                    let time=time::precise_time_ns()-entry;
                    let path=req.url.path.join("/");
                    if path.contains("task") {
                        println!("Request: {:?}\nResponse: {:?}\nResponse-Time: {}", req, res, time);
                    }
                    return res;
                }
                let url = Url::parse(format!("{}://{}:{}/account/login/",req.url.scheme,req.url.host,req.url.port).as_str()).unwrap();
                Ok(Response::with((status::Found, Redirect(url.clone()))))

            }
        }
        Box::new(LoggerHandler {handler:handler }) as Box<Handler>
    }
}

