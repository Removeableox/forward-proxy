use std::{
    net::{TcpListener, TcpStream},
    io::Write,
    fs,
};

fn handle_connection(mut stream: TcpStream) {
    let status = "HTTP/1.1 200 OK";
    let file = fs::read_to_string(format!("src/html/index.html")).unwrap();
    let size = file.len();
    let response = format!("{status}\r\nContent-Length: {size}\r\n\r\n{file}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}
