use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    // Create a listener that listens for connections. Later on handle the error when connecting to
    // server.
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap(); // Unwrap so the program will panic if there's error variant
    
    // Loop for connections and print if we have a connection estabilished
    for _stream in listener.incoming() { //incoming() returns an iterator being recived on listener.
        let _stream = _stream.unwrap(); 

        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) { // stream is mutable because .read() takes &mut self as an argument
    let mut buffer = [0; 1024]; // Might want to change later on

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get) {
             ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
}
