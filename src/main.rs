use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //iniciar un servidor
    let address = "localhost:8000";
    let listener = TcpListener::bind(&address).unwrap();
    println!("Server listening on {}", address);
    //escuchar por conexiones
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");

        handle_connection(stream);
    }
}

//manejar las conexiones
fn handle_connection(mut stream: TcpStream) {
    //leer el request
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    //responder con un mensaje

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        //responder al cliente
        send_index(&mut stream);
    }else{
        send_404(&mut stream);
    }
}

fn build_response(content: String) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    )
}

fn send_index(stream: &mut TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();
    let response = build_response(contents);
    stream.write(response.as_bytes()).unwrap();
}

fn send_404(stream: &mut TcpStream) {
    let contents = fs::read_to_string("404.html").unwrap();
    let response = build_response(contents);
    stream.write(response.as_bytes()).unwrap();
}