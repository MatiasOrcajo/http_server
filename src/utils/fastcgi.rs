use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

use crate::models::Request;

const FCGI_BEGIN_REQUEST: u8 = 1;
const FCGI_PARAMS: u8 = 4;
const FCGI_STDIN: u8 = 5;

fn build_header(ty: u8, request_id: u16, content_length: u16, padding_length: u8) -> [u8; 8] {
    [
        1,
        ty,
        (request_id >> 8) as u8,
        (request_id & 0xFF) as u8,
        (content_length >> 8) as u8,
        (content_length & 0xFF) as u8,
        padding_length,
        0,
    ]
}

fn build_name_value_pair(key: &str, value: &str) -> Vec<u8> {
    let mut pair = Vec::new();
    pair.push(key.len() as u8);
    pair.push(value.len() as u8);
    pair.extend_from_slice(key.as_bytes());
    pair.extend_from_slice(value.as_bytes());
    pair
}


pub fn fastcgi(request: Request) -> std::io::Result<()> {

    let mut stream = UnixStream::connect("/run/php/php8.3-fpm.sock")?;

    // 1. Send FCGI_BEGIN_REQUEST
    let header = build_header(FCGI_BEGIN_REQUEST, 1, 8, 0);
    let body = [
        0, 1, // FCGI_RESPONDER
        0, 0, // Flags
        0, 0, 0, 0, // Reserved
    ];
    stream.write_all(&header)?;
    stream.write_all(&body)?;

    // 2. Send FCGI_PARAMS
    let params = [
        ("SCRIPT_FILENAME", "/tmp/test.php"),
        ("REQUEST_METHOD", &request.method),
        ("QUERY_STRING", ""),
        ("CONTENT_TYPE", &request.content_type),
        ("CONTENT_LENGTH", &request.content_length)
    ];

    // ("SERVER_SOFTWARE", "MyRustServer/1.0"),
    // ("SERVER_PROTOCOL", "HTTP/1.1"),
    // ("SERVER_NAME", "localhost"),
    // ("SERVER_PORT", "8080"),
    // ("REMOTE_ADDR", "127.0.0.1"),
    // ("REMOTE_PORT", "54321"),
    // ("SCRIPT_FILENAME", "/var/www/html/submit.php"),
    // ("SCRIPT_NAME", "/submit.php"),
    // ("DOCUMENT_ROOT", "/var/www/html"),
    // ("REQUEST_URI", "/submit.php?name=John"),
    // ("QUERY_STRING", "name=John"),
    // ("REQUEST_METHOD", "POST"),
    // ("CONTENT_TYPE", "application/x-www-form-urlencoded"),
    // ("CONTENT_LENGTH", "19"),

    for (key, value) in &params {
        let data = build_name_value_pair(key, value);
        let header = build_header(FCGI_PARAMS, 1, data.len() as u16, 0);
        stream.write_all(&header)?;
        stream.write_all(&data)?;
    }

    // Ends FCGI_PARAMS
    let header = build_header(FCGI_PARAMS, 1, 0, 0);
    stream.write_all(&header)?;

    // 3. Reads PHP-FPM response
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    println!("{}", String::from_utf8_lossy(&response));
    Ok(())
}
