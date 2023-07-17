use std::env;
use std::io::{self, Write};
use std::net::TcpStream;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <ip>", args[0]);
        process::exit(1);
    }

    let ip = args[1].clone();

    for port in 1..=1024 {
        let ip_for_thread = ip.clone();
        thread::spawn(move || {
            match TcpStream::connect(format!("{}:{}", ip_for_thread, port)) {
                Ok(_) => {
                    println!("Port {} is open", port);
                    io::stdout().flush().unwrap();
                }
                Err(_) => {}
            }
        });
        thread::sleep(Duration::from_millis(10));
    }
}
