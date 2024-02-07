use std::{io::BufReader, net::TcpStream};

pub fn handle_get(http_request: &Vec<String>) -> String {
    let path = *&http_request[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .strip_prefix('/')
        .unwrap(); //Gets the key for map

    let map = crate::storage::load_hashmap();
    let long_url = map[path].long_url.clone();

    long_url
}
