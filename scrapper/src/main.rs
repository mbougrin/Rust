use crate::server::tcp::init_tcp;
mod server;
mod view;
mod parse_input_request;
mod send_response;
mod scrap_source;
mod source;

fn main() {
    init_tcp();
}