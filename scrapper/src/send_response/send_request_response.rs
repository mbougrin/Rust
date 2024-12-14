use std::io::Write;
use std::net::TcpStream;
use crate::parse_input_request::parse_request::ResponseRequestInfo;
use crate::view::html::{html_file, html_file_search};

fn write_response_request_fail(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 400 Bad Request";
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        0,
        ""
    );
    stream.write_all(response.as_bytes()).unwrap();
}
fn write_response_request_default(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = html_file();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}
fn write_response_request(mut stream: TcpStream, req_stc: ResponseRequestInfo) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = html_file_search(req_stc.req_search);
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
}

pub fn send_response(req_stc: ResponseRequestInfo, stream: TcpStream) {
    if req_stc.req_type == "GET" {
        if req_stc.req_search == "" {
            write_response_request_default(stream);
        } else {
            write_response_request(stream, req_stc);
        }
    } else {
        write_response_request_fail(stream);
    }
}