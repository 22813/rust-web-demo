use iron::prelude::*;

use std::cell::RefCell;
//use framework::database;
use router::Router;
use urlencoded::{UrlEncodedBody,UrlEncodedQuery};

pub fn get_form_param(req:&mut Request,name:&str)->Option<String>{
    let query=RefCell::new(req.get_ref::<UrlEncodedBody>()).into_inner();
    //let query=RefCell::new(req.get_ref::<UrlEncodedQuery>()).into_inner();
    if let Ok(ref hashmap)=query{
        if let Some(vec)=hashmap.get(name){
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
pub fn get_query_param(req:&mut Request,name:&str)->Option<String>{
    let query=RefCell::new(req.get_ref::<UrlEncodedQuery>()).into_inner();
    if let Ok(ref hashmap)=query{
        if let Some(vec)=hashmap.get(name){
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

pub fn get_path_param(req:&mut Request,name:&str)->Option<String>{
    let ref value = req.extensions.get::<Router>().unwrap().find(name).unwrap_or("");
    if value.len()>0 {
        return Some(String::from(*value));
    }
    None
}



