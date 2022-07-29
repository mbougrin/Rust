use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

fn print_time() {
    println!("{}", SystemTime::now()
             .duration_since(UNIX_EPOCH)
             .unwrap().as_millis());

}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_millis();
        let mut s = since_the_epoch.to_string();
        s.push_str("\n");
        stream.write(s.as_bytes()).unwrap();
        stream.flush().unwrap();
        let ms = Duration::from_millis(1000);
        thread::sleep(ms);
    }
}

fn help(name: &String) {
    println!("{name} listening_ipaddr listening_port");
}

fn run_server(server_ip_port: &String) {
    let listener = TcpListener::bind(server_ip_port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream)
        });
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            print_time();
            let mut server_ip_port = String::with_capacity(128);
            server_ip_port.push_str(&args[1]);
            server_ip_port.push_str(":");
            server_ip_port.push_str(&args[2]);
            run_server(&server_ip_port);
        },
        _ => {
            help(&args[0]);
        }
    }
    Ok(())
}
