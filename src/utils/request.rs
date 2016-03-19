use iron::prelude::*;
use router::Router;
use urlencoded::{UrlEncodedBody,UrlEncodedQuery};
use std::collections::HashMap;
use urlencoded::UrlDecodingError;

pub trait RequestExt {
    fn get_form_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String>;
    fn get_query_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String>;
    fn get_path_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String>;
}


fn get_from_hash(r:Result<&HashMap<String,Vec<String>>, UrlDecodingError>, key:&str)->Option<String>{
    if let Ok(ref hashmap)=r{
        if let Some(vec)=hashmap.get(key){
            if vec.len()>0{
                let value=&vec[0];
                if value.len()>0{
                    let mut result=String::new();
                    result.push_str(value.as_str());
                    return Some(result);
                }
            }
        }
    }
    None
}

impl<'a, 'b> RequestExt for Request<'a, 'b> {
    fn get_form_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String> {
        get_from_hash(self.get_ref::<UrlEncodedBody>(),name)
    }
    fn get_query_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String> {
        get_from_hash(self.get_ref::<UrlEncodedQuery>(),name)
    }
    fn get_path_param<'c, 'd>(&'c mut self, name: &'d str) -> Option<String>{
        let ref value = self.extensions.get::<Router>().unwrap().find(name).unwrap_or("");
        if value.len()>0 {
            return Some(String::from(*value));
        }
        None
    }
}


