use std::net::TcpListener;

fn main() {
    // Create a listener that listens for connections. Later on handle the error when connecting to
    // server.
    let listener = 
        TcpListener::bind("127.0.0.1:7878").unwrap(); // Unwrap so the program will panic if there's error variant
    
    // Loop for connections and print if we have a connection estabilished
    for stream in listener.incoming() { //incoming returns an iterator being recived on listener.
        let stream = stream.unwrap(); 

        println!("Connection estabilished!");
    }
}
