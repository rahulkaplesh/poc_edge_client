mod common;
mod client;
use chrono;
use std::{process, sync::{Arc, atomic::{AtomicUsize, Ordering}}, thread, time};
use clap::{App, Arg};
use ctrlc;

fn main() {
    let running = Arc::new(AtomicUsize::new(0));
    let r = running.clone();
    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            println!("Exiting...");
        } else {
            process::exit(0);
        }
    }).expect("Error setting Ctrl-C handler");
    let matches = App::new("Edge Client")
        .version("1.0")
        .author("Rahul Kaplesh <rahulkaplesh@gmail.com")
        .about("an edge client")
        .arg( 
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("String")
                .help("Takes an address")
                .takes_value(true),
        ).arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("String")
                .help("Takes an name")
                .takes_value(true),
        ).get_matches();
    if let Some(client_name) = matches.value_of("name") {
        if let Some(server_address) = matches.value_of("address") {
            let temp_client = client::Client::new(client_name, server_address);
            match temp_client {
                Ok(temp) => {
                    loop {
                        if running.load(Ordering::SeqCst) > 0 {
                            break;
                        }
                        let temp = Some(temp.clone());
                        let dt: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
                        temp.unwrap().send_data(dt.to_string().as_str());
                        thread::sleep(time::Duration::from_secs(10));
                    }
                },
                Err(err) => println!("Cannot Instantiate Client: {}", err),
            }
        } else {
            println!("Enter the server address you want to connect to!!");
        }
    } else {
        println!("Enter the name of the client!!");
    }
        
}
