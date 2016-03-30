use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};

pub fn init() {
    let locker = Arc::new(Mutex::new(false));
    thread::spawn(move || {
        loop {
            let _=locker.lock().unwrap();
            work();
            thread::sleep(Duration::from_secs(10));        
        }
    });
}

fn work(){
    //println!("schedule working .....");
}

