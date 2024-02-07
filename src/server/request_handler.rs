use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::server::{route_creation, route_redirection};

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("{:?}", http_request);

    let req_type = &http_request[0];

    if req_type.starts_with("GET") {
        let long_url = route_redirection::handle_get(&http_request);

        let status_line = "HTTP/1.1 301 Moved Permanently";
        let response = format!("{status_line}\r\nLocation: {long_url}\r\n\r\n");

        stream.write_all(response.as_bytes()).unwrap();
    } else if req_type.starts_with("POST") {
        route_creation::handle_post(&http_request)
    } else {
    }
}
