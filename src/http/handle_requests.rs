#[allow(unused_variables)]

use std::{io::{Read, Write}, net::TcpListener};
use crate::models::Request;

pub fn handle_requests() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; 2048];

        match stream.read(&mut temp_buffer) {
            Ok(bytes_read) => {
                    
                buffer.extend_from_slice(&temp_buffer[..bytes_read]);

                let request = Request::new(String::from_utf8_lossy(&buffer).to_string());

                let response = format!(
                    "HTTP/1.1 200 OK\r\n\r\n {:?}", request.unwrap().method.unwrap());

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();

                

            }
            Err(e) => {
                eprintln!("Error reading the connection: {}", e);
            }
        }

        println!("Connection established")
    }
}