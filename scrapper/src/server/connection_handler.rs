use std::io;
use std::io::{BufRead, Write};
use std::net::TcpStream;
use crate::parse_input_request::parse_request::parse_request_response;

pub fn handler_connection_test(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

fn print_request(buffer: &String) {
    println!("request: {}", buffer);
}
pub fn handler_connection(mut stream: TcpStream) {
    // SomeTime Not Response With This
    // let mut buffer = String::new();
    // stream.read_to_string(&mut buffer).unwrap();
    // parse_request_response(buffer, stream);

    let mut reader = io::BufReader::new(&mut stream);
    let received: Vec<u8> = reader.fill_buf().expect("error convert vec").to_vec();
    reader.consume(received.len());
    let recv = String::from_utf8(received).unwrap();
    println!("\n\nRequest:{}\n\n", recv);
    parse_request_response(recv, stream);

    // with buffer
    // let mut buffer = [0;256];
    // stream.read(&mut buffer).unwrap();
    // let buff_str = String::from_utf8_lossy(&buffer).to_string();
    // parse_request_response(buff_str, stream);
}