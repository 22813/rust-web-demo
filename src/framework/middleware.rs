use iron::{AroundMiddleware,BeforeMiddleware,AfterMiddleware,Handler,typemap};
use iron::prelude::*;
use time;
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
        let delta=time::precise_time_ns()-*req.extensions.get::<MyMiddleware>().unwrap();
        println!("Request url:{}, took: {} ms",req.url.path.join("/"),(delta as f64)/1000000.0);
        Ok(res)
    }
}


impl AroundMiddleware for MyMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        struct LoggerHandler<H: Handler> {  handler: H }
        impl<H: Handler> Handler for LoggerHandler<H> {
            fn handle(&self, req: &mut Request) -> IronResult<Response> {
                let entry = time::precise_time_ns();
                let res = self.handler.handle(req);
                let time=time::precise_time_ns()-entry;
                println!("Request: {:?}\nResponse: {:?}\nResponse-Time: {}", req, res, time);
                res
            }
        }
        Box::new(LoggerHandler {handler:handler }) as Box<Handler>
    }
}

