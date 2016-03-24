pub mod task;
pub mod account;
pub mod prelude {
    pub use std::str::FromStr;
    pub use std::collections::BTreeMap;
    pub use rustc_serialize::json;
    pub use rustc_serialize::json::{Json,ToJson};
    pub use handlebars_iron::{Template};
    pub use chrono::*;

    pub use iron::prelude::*;
    pub use iron_login::User;
    pub use iron::{Url, status};
    pub use iron::modifiers::Redirect;

    pub use utils::crypto;
    pub use utils::{response};
    pub use utils::request::*;
    pub use router::Router;
    
    use iron_login;
    use std::path::Path;
    use iron::{AroundMiddleware,Handler,typemap};
    use handlebars_iron::{HandlebarsEngine};
    use controllers;
    use logger::Logger;
    use logger::format::Format;
    use logger::format::FormatAttr::FunctionAttrs;
    use term;
    use mount::Mount;
    use staticfile::Static;
    static FORMAT: &'static str = "@[red A]Uri: {uri}@, @[blue blink underline]Method: {method}@, @[yellow standout]Status: {status}@, @[brightgreen]Time: {response-time}@";


    pub fn get_chain()->Chain{
        let mut router = Router::new();
        controllers::task::init_router(&mut router);
        controllers::account::init_router(&mut router);

        let mut mount = Mount::new();
        mount.mount("/", router);
        mount.mount("/static", Static::new(Path::new("./src/static/")));

        let mut chain = Chain::new(mount);
        chain.link_after(HandlebarsEngine::new("./src/templates", ".hbs"));
        chain.link_around(LoginChecker);
        chain.link_around(iron_login::LoginManager::new(b"My Secret Key"[..].to_owned()));

        fn attrs(req: &Request, _res: &Response) -> Vec<term::Attr> {
            match format!("{}", req.url).as_ref() {
                "/" => vec![term::Attr::Blink],
                _ => vec![]
            }
        }

        let format = Format::new(FORMAT, vec![], vec![FunctionAttrs(attrs)]);
        chain.link(Logger::new(Some(format.unwrap())));
        chain
    }

    pub struct LoginChecker;

    impl typemap::Key for LoginChecker{type Value=u64;}
    impl AroundMiddleware for LoginChecker {
        fn around(self, handler: Box<Handler>) -> Box<Handler> {
            struct LoggerHandler<H: Handler> {  handler: H }
            impl<H: Handler> Handler for LoggerHandler<H> {
                fn handle(&self, req: &mut Request) -> IronResult<Response> {
                    if controllers::account::check_login(req) || req.url.path.join("/").contains("account") {
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
}
