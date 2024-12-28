mod http;

use http::get_http;

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:160").unwrap();

    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap(); 

    }
}
