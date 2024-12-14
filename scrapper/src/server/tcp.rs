use std::net::TcpListener;
use std::thread;
use crate::server::connection_handler::{handler_connection, handler_connection_test};

pub fn init_tcp_test() {
    let listener = TcpListener::bind("127.0.0.1:4040").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
            handler_connection_test(stream);
    }
}
pub fn init_tcp() {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handler_connection(stream);
        });
    }
}