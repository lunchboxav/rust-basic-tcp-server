use std::io::{Write, Read, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

//extern crate httparse;

/*
//this is unused, placed here for archiving purpose
//if you wanna use this, stream are used without borrowing
fn read_request_head(stream: &TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let mut buff = Vec::new();
    let mut read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
    while read_bytes > 0 {
        read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
        if read_bytes == 2 && &buff[(buff.len()-2)..] == b"\r\n" {
            break;
        }
    }
    return buff;
}

fn handle_request(stream: TcpStream) {
    let request_bytes = read_request_head(&stream);
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let parsed_req = req.parse(&request_bytes);
    println!("{:?}", parsed_req);

    match req.path {
        Some(path) => {
            if path.starts_with("/hello") {
                let x = &path[7..];
                match x {
                    "" => send_response_hello(stream),
                    _ => send_response_hello_to(stream, String::from(x))
                }
            } else if path.starts_with("/numbers") {
                let y = &path[9..];
                let args: Vec<&str> = y.split("&").collect();
                let arg1_: Vec<&str> = args[0].split("=").collect();
                let arg1 = arg1_[1];
                let arg2_: Vec<&str> = args[1].split("=").collect();
                let arg2 = arg2_[1];

                send_response_multiplication(stream, arg1, arg2);
            } 
        },
        None => {
            send_response_error(stream);
        }
    }
}
*/

fn send_response_hello(mut stream: &TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    stream.write(response.as_bytes()).expect("Response failed");
}

fn send_response_hello_to(mut stream: &TcpStream, name: String) {
    let res_1 = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello ";
    let res_2 = name;
    let res_3 = "</body></html>\r\n";

    let response = format!("{}{}{}", res_1, res_2, res_3);
    stream.write(response.as_bytes()).expect("Response failed");
}

fn send_response_multiplication(mut stream: &TcpStream, op1_: &str, op2_: &str) {
    let op1: i32 = op1_.parse().unwrap(); 
    let op2: i32 = op2_.parse().unwrap();
    let result = op1*op2;

    let res_1 = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello ";
    let res_2 = result.to_string();
    let res_3 = "</body></html>\r\n";

    let response = format!("{}{}{}", res_1, res_2, res_3);
    stream.write(response.as_bytes()).expect("Response failed");
}

fn send_response_error(mut stream: &TcpStream) {
    let response = b"HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>500 - Server Error</body></html>\r\n";
    stream.write(response).expect("Write failed");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9386").unwrap();
    println!("server is listening on port 9386");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => { 
                thread::spawn( move || {
                    //let response = "Hello world\r\n";
                    handle_request(stream);
                })
            },

            Err(e) => {
                thread::spawn(move || {
                    println!("Unable to connect: {}",e)
                })
            },
        };
    }
}

fn handle_request(stream: TcpStream) {
    //let stream_clone = stream.try_clone().unwrap();
    let mut reader = BufReader::new(&stream);

    for (index, line) in reader.by_ref().lines().enumerate() {
        if index == 0 {
            if let Ok(ref l) = line {
                let request = &*l;
                let reqs: Vec<&str> = request.split(" ").collect();
                route_request(&stream, reqs[1]);
            }
        }

        if line.unwrap() == "" {
            break;
        }
    }

    //send_response_hello(reader.into_inner());
}

fn route_request(stream: &TcpStream, path: &str) {
    if path.starts_with("/numbers") {
        let y = &path[9..];
        let args: Vec<&str> = y.split("&").collect();
        let arg1_: Vec<&str> = args[0].split("=").collect();
        let arg1 = arg1_[1];
        let arg2_: Vec<&str> = args[1].split("=").collect();
        let arg2 = arg2_[1];

        send_response_multiplication(&stream, arg1, arg2);
    } else if path.starts_with("/hello") {
        let x = &path[7..];
        match x {
            "" => send_response_hello(&stream),
            _ => send_response_hello_to(&stream, String::from(x))
        }
    } else {
        send_response_error(&stream);
    }
}
