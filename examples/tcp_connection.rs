use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
extern crate url_shortener;
fn main() {
    url_shortener::server::run_server();
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 301 Moved Permanently";
    let response = format!("{status_line}\r\nLocation: https://www.google.com\r\n\r\n");
    // let contents = "<!DOCTYPE html>
    // <html lang=\"en\">
    //   <head>
    //     <meta charset=\"utf-8\">
    //     <title>Hello!</title>
    //   </head>
    //   <body>
    //     <h1>Hello!</h1>
    //     <p>Hi from Rust</p>
    //   </body>
    // </html>
    // ";
    // let length = contents.len();

    // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
