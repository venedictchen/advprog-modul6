
use std::{
    fs,
    io::{prelude::*,BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request:Vec<_> = buf_reader
    .lines()
    .map(|result|result.unwrap())
    .take_while(|line|!line.is_empty()) 
    .collect();


    if http_request.is_empty(){
        return;
    }

    
    let request_line = http_request.get(0).unwrap();
    
    let response = generate_response(request_line);

    stream.write_all(response.as_bytes()).unwrap();
    
}


fn generate_response(request_line: &str) -> String {
    let get = "GET / HTTP/1.1"; 
    let (status_line, filename) = if request_line == get {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}",
    )
}

