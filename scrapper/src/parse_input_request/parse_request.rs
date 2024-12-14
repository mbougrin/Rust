use std::collections::HashMap;
use std::net::TcpStream;
use std::str::Split;
use crate::parse_input_request::parse_search::parse_search;
use crate::send_response::send_request_response::send_response;
pub struct ResponseRequestInfo<'a> {
    pub req_type:&'a str,
    pub req_search:&'a str,
    pub req_http:&'a str,
    pub req_param:HashMap<String,String>,
}

fn parse_req_type(line: &str) -> &str {
    let mut split = line.split(" ");
    split.next().unwrap()
}

fn parse_req_html(line: &str) -> &str {
    let split = line.split(" ");
    let collection: Vec<&str> = split.collect();
    collection.get(collection.len() - 1).unwrap()
}

fn parse_param_request(split_buffer:Split<&str>) -> HashMap<String,String> {
    let mut mymapp = HashMap::new();
    for it in split_buffer {
        if it.contains(": ") {
            let mut key_value = it.split(": ");
            mymapp.insert(key_value.next().unwrap().to_string(),
                          key_value.last().unwrap().to_string());
        }
    }
    mymapp
}

fn print_struct(req_stc: &ResponseRequestInfo) {
    println!("req_param:{:?}", req_stc.req_param);
    println!("req_type:{}", req_stc.req_type);
    println!("req_search:{}", req_stc.req_search);
    println!("req_http:{}", req_stc.req_http);
}

pub fn parse_request_response(buffer: String, stream: TcpStream) {
    // let old_value: &'static &str;
    let mut split_buffer:Split<&str> = buffer.trim().split("\r\n");
    let first_line = split_buffer.next().unwrap();
    let mut req_stc:ResponseRequestInfo;
    if first_line.contains("source") {
        req_stc = ResponseRequestInfo{
            req_type: parse_req_type(first_line),
            req_search: "old_value",
            req_http: parse_req_html(first_line),
            req_param: parse_param_request(split_buffer),
        };
    } else {
        req_stc = ResponseRequestInfo{
            req_type: parse_req_type(first_line),
            req_search: parse_search(first_line),
            req_http: parse_req_html(first_line),
            req_param: parse_param_request(split_buffer),
        };
        // old_value = &req_stc.req_search.clone();
    }
    print_struct(&req_stc);
    send_response(req_stc, stream);
}