use std::fs;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::task::Task;

fn handle(mut stream: TcpStream) {
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).unwrap();
    let request = String::from_utf8_lossy(&buf[..]).to_string();
    if request.is_empty() {
        return;
    }
    let pos = request.find('{');
    match pos {
        None => {
            println!("Bad request: {}", request);
        }
        Some(pos) => {
            process(&request[pos..]);
        }
    }
}

fn process(request: &str) {
    println!("{}", request);
    let task: Task = serde_json::from_slice(request.as_bytes()).unwrap();
    let mut input_str : Vec<String> = Vec::new();
    let mut output_str : Vec<String> = Vec::new();
    for test in task.tests {
        input_str.push(test.input);
        output_str.push(test.output);
    }
    fs::write("./input.txt", input_str.join("")).unwrap();
    fs::write("./valid.txt", output_str.join("")).unwrap();
}

pub fn listen() {
    let listener = TcpListener::bind("127.0.0.1:4244").unwrap();
    println!("Listening for connections on port {}", 4244);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle(stream);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}