pub mod task;
pub mod account;
pub mod prelude {
    pub use std::str::FromStr;
    pub use std::collections::BTreeMap;
    pub use rustc_serialize::json;
    pub use rustc_serialize::json::{Json,ToJson};
    pub use hbs::{Template, HandlebarsEngine, DirectorySource, MemorySource};
    pub use chrono::*;

    pub use iron::prelude::*;
    pub use iron_login::User;
    pub use iron::{Url, status};
    pub use iron::modifiers::Redirect;

    pub use utils::crypto;
    pub use utils::{response};
    pub use utils::request::*;
    pub use router::{Router,NoRoute};
    
    use iron_login;
    use std::path::Path;
    use iron::{AfterMiddleware,AroundMiddleware,Handler,typemap};
    use logger::Logger;
    use mount::Mount;
    use staticfile::Static;

    pub fn get_chain()->Chain{
        let mut router = Router::new();
        fn index(_: &mut Request) -> IronResult<Response> {
            response::template("index",())
        }
        router.get("/",index);
        super::task::init_router(&mut router);
        super::account::init_router(&mut router);

        let mut mount = Mount::new();
        mount.mount("/", router);
        mount.mount("/static", Static::new(Path::new("./src/static/")));

        let mut chain = Chain::new(mount);

        let mut hbse = HandlebarsEngine::new();
        hbse.add(Box::new(DirectorySource::new("./src/templates/", ".hbs")));
        if let Err(r) = hbse.reload() {
            //panic!("{}", r.description());
            panic!("{:?}", r);
        }
        chain.link_after(hbse);
        chain.link_after(ErrorReporter);
        chain.link_after(Custom404);

        chain.link_around(LoginChecker);
        chain.link_around(iron_login::LoginManager::new(b"My Secret Key"[..].to_owned()));
        chain.link(Logger::new(None));
        chain
    }

    struct LoginChecker;
    impl typemap::Key for LoginChecker{type Value=u64;}
    impl AroundMiddleware for LoginChecker {
        fn around(self, handler: Box<Handler>) -> Box<Handler> {
            struct LoggerHandler<H: Handler> {  handler: H }
            impl<H: Handler> Handler for LoggerHandler<H> {
                fn handle(&self, req: &mut Request) -> IronResult<Response> {
                    if super::account::check_login(req) || req.url.path.join("/").contains("account") {
                        let res = self.handler.handle(req);
                        return res;
                    }
                    let url = Url::parse(format!("{}://{}:{}/account/login/",req.url.scheme,req.url.host,req.url.port).as_str()).unwrap();
                    Ok(Response::with((status::Found, Redirect(url.clone()))))
                }
            }
            Box::new(LoggerHandler {handler:handler }) as Box<Handler>
        }
    }
    struct ErrorReporter;
    impl AfterMiddleware for ErrorReporter {
        fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
            //println!("{}", err.description());
            println!("{:?}", err);
            Err(err)
        }
    }
    struct Custom404;
    impl AfterMiddleware for Custom404 {
        fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
            println!("Hitting custom 404 middleware");
            if let Some(_) = err.error.downcast::<NoRoute>() {
                Ok(Response::with((status::NotFound, "Custom 404 response")))
            } else {
                Err(err)
            }
        }
    }
}
