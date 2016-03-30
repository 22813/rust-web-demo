use std::process::Output;
use std::process::Command;
use std::io::Result;
use regex::Regex;
use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
pub fn http_post() {
    let client = Client::new();
    let mut res = client.post("http://www.baidu.com").body("foo=bar").send();
    match res {
        Ok(ref mut res)=>{
            let mut body = String::new();
            res.read_to_string(&mut body).unwrap();
            println!("Response: {}", body);
        },
        Err(e)=>{
            println!("error occurs:{:?}",e);
        }
    }
}

pub fn http_get() {
    let client = Client::new();
    let mut res = client.get("http://rust-lang.org/").header(Connection::close()).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Response: {}", body);
}

// invoke_system_command("sh",&["-c","echo hello"]);
pub fn invoke_system_command(command:&str,args:&[&str]){
    //let output:Result<Output>=Command::new("sh").arg("-c").arg("echo hello").output();
    let output:Result<Output>=Command::new(command).args(args).output();
    match output {
        Ok(output)=>{
            let status=output.status;
            let stdout:Vec<u8>=output.stdout;
            let stderr:Vec<u8>=output.stderr;
            let stdout:String=String::from_utf8(stdout).ok().unwrap();
            println!("stdout:{:?}",stdout);
            let stderr:String=String::from_utf8(stderr).ok().unwrap();
            println!("stderr:{:?}",stderr);
            println!("status:{:?}",status);
        },
        Err(e)=>println!("error:{:?}",e),
    }   
}

pub fn regex_test() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}",
                 cap.at(2).unwrap_or(""), cap.at(3).unwrap_or(""),
                 cap.at(1).unwrap_or(""));
    }
}

