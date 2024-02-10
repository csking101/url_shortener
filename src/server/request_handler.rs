use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::server::{route_creation, route_deletion, route_redirection};

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
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let req_type = &http_request[0];

    if req_type.starts_with("GET") {
        let response = route_redirection::handle_get(&http_request);
        stream.write_all(response.as_bytes()).unwrap();
    } else if req_type.starts_with("POST") {
        let mut content_length: usize = 0;
        for line in &http_request {
            if line.starts_with("Content-Length") {
                content_length = line.split_whitespace().collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap();
            }
        }

        let mut payload_buffer = vec![0; content_length];
        buf_reader.read_exact(&mut payload_buffer).unwrap();
        let payload = String::from_utf8(payload_buffer).unwrap();

        let response = route_creation::handle_post(&http_request, payload);
        stream.write_all(response.as_bytes()).unwrap();
    } else if req_type.starts_with("DELETE") {
        let response = route_deletion::handle_delete(&http_request);
        stream.write_all(response.as_bytes()).unwrap();
    }
}
