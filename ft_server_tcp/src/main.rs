use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

fn help(name: &str) {
    println!("{} listening_ipaddr listening_port", name);
}

fn run_server(server_ip_port: &str) {
    let listener = TcpListener::bind(server_ip_port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("New client connection etablished!");
        handle_connection(stream);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let mut server_ip_port = String::with_capacity(128);
            server_ip_port.push_str(&args[1]);
            server_ip_port.push_str(":");
            server_ip_port.push_str(&args[2]);
            run_server(&server_ip_port);
        },
        _ => {
            help(&args[0])
        }
    }
    Ok(())
}
