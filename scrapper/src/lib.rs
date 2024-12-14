mod parse_input_request;
mod scrap_source;
mod server;
mod source;
mod view;
mod send_response;

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::net::{TcpStream};
    use std::thread;
    use std::time::Duration;
    use crate::tcp::init_tcp_test;
    #[test]
    fn it_not_running() {
        let response = "Connection refused (os error 111)";
        let stream = TcpStream::connect("127.0.0.1:4040");
        assert_eq!(stream.err().unwrap().to_string(), response)
    }
    #[test]
    fn it_running() {
        thread::spawn(|| {
            init_tcp_test();
        });
        thread::sleep(Duration::from_millis(1000));
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        let mut result = [0; 19];
        let stream = TcpStream::connect("127.0.0.1:4040");
        stream.unwrap().read(&mut result).expect("read failed");
        assert_eq!(std::str::from_utf8(&result).expect("valid utf8"), response)
    }
}
