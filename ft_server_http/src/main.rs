use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::thread;
use std::fs;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("http/helloworld.html").unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
