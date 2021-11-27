mod common;
mod client;
use chrono;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let temp_client = client::Client::new("Sample-1");
    match temp_client {
        Ok(temp) => {
            loop {
                let temp = Some(temp.clone());
                let dt: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
                temp.unwrap().send_data(dt.to_string().as_str());
                thread::sleep(time::Duration::from_secs(10));
            }
        },
        Err(err) => println!("Cannot Instantiate Client: {}", err),
    }    
}
