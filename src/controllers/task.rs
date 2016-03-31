use std::sync::{Once, ONCE_INIT};
use utils::crypto;
use services::task as service;
use models::*;
use super::prelude::*;

static START: Once = ONCE_INIT;
pub fn init_router(router:&mut Router){
    START.call_once(|| {
        router.get("/task/",|_: &mut Request|{
            let tasks=service::list();
            let mut data = BTreeMap::new();
            data.insert("tasks".to_string(), tasks.to_json());
            response::template("task/list",data)
        });

        router.get("/task/json/", |_:&mut Request|{
            let tasks=service::list();
            let mut data = BTreeMap::new();
            data.insert("tasks".to_string(), tasks.to_json());
            let data = json::encode(&data).unwrap();
            response::ok_json(&data)
        });

        router.get("/task/json/aes/",|_:&mut Request|{
            let tasks=service::list();
            let mut data = BTreeMap::new();
            data.insert("tasks".to_string(), tasks.to_json());
            let data = json::encode(&data).unwrap();
            let data=crypto::aes_encrypt_string(&data);
            let data=crypto::base64_encode_bytes(&data.ok().unwrap());
            let data=data.expect("");
            response::ok(&data)
        });

        router.get("/task/json/base64/",|_:&mut Request|{
            let tasks=service::list();
            let mut data = BTreeMap::new();
            data.insert("tasks".to_string(), tasks.to_json());
            let data = json::encode(&data).unwrap();
            let data=crypto::base64_encode_string(&data).expect("");
            response::ok(&data)
        });

        router.get("/task/new",|_:&mut Request|response::template("task/new",()));

        router.get("/task/:id",|req: &mut Request|{
            let id=req.get_path_param("id").unwrap_or("0".to_owned());
            let id=i32::from_str(&*id).unwrap_or(0);
            let mut response = Response::new();
            response.set_mut(status::Ok);
            if id>0{
                let task=service::get(id);
                if let Some(task)=task {
                    let mut data = BTreeMap::new();
                    data.insert("task".to_string(), task.to_json());
                    response.set_mut(Template::new("task/edit", data));
                }
            }
            Ok(response)
        });

        router.get("/task/delete/:id",|req: &mut Request| {
            let id=req.get_path_param("id").unwrap_or("0".to_owned());
            let id=i32::from_str(&*id).unwrap_or(0);
            if id>0{
                service::delete(id);
            }
            response::redirect(req,"/task/")
        });

        router.post("/task/",|req: &mut Request|{
            let name=req.get_form_param("name"); 
            let content=req.get_form_param("content");
            let status=req.get_form_param("status").unwrap_or("0".to_owned());
            let time:DateTime<Local>=Local::now();
            let id=req.get_form_param("id").unwrap_or("0".to_owned());
            let task=Task{
                id:             i32::from_str(&*id).unwrap_or(0),
                name:           name,
                content:        content,
                create_time:    Some(time),
                update_time:    Some(time),
                status:         i32::from_str(&*status).unwrap_or(0),
            };
            println!("saving task:{:?}",&task);
            service::save(&task);
            response::redirect(req,"/task/")
        });

        //curl --data-urlencode "data=NTDlhYMzMDDmnaEs5aSW6ZO+5Luj5Y+RLOmUmuaWh+acrA==" "http://localhost:8080/api"
        router.post("/task/json-post",|req: &mut Request| {
            if let Some(s)=req.get_form_param("data"){
                if let Some(data)=crypto::base64_decode_to_string(&s) {
                    if let Ok(data)=Json::from_str(&data) {
                        if let Some(obj)=data.as_object() {
                            //let id=get_json_i64(&obj,"id");
                            let manufactor=get_json_string(&obj,"manufactor");
                            let id=get_json_i64(&obj,"id");
                            println!("id:{}",id); 
                            println!("manufactor:{:?}",manufactor);
                        }
                    }
                }
            }
            Ok(Response::with(status::Ok))
        });
    });
}

fn get_json_string(obj:&BTreeMap<String,Json>,key:&str)->Option<String>{
    obj.get(key).map(|json|json.as_string()).unwrap_or_else(||None).map(|str|str.to_owned())
}

fn get_json_i64(obj:&BTreeMap<String,Json>,key:&str)->i64{
    obj.get(key).map(|json|json.as_i64()).unwrap_or_default().unwrap_or_default()
}

