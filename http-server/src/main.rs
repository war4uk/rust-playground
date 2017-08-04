use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8181").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }


    println!("Hello, world!");
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        response_with_content("HTTP/1.1 200 OK \r\n\r\n", "index.html", &mut stream);
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        response_with_content("HTTP/1.1 200 OK \r\n\r\n", "index.html", &mut stream);
    } else {
        response_with_content("HTTP/1.1 404 NOT FOUND \r\n\r\n", "404.html", &mut stream);
    }
}

fn response_with_content(first_line: &str, file_name: &str, stream: &mut TcpStream) {
    let mut file = File::open(file_name).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let response = format!("{}{}", first_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}