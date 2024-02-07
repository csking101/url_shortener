use std::{io::BufReader, net::TcpStream};

pub fn handle_get(http_request: &Vec<String>) -> String {
    let path = *&http_request[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .strip_prefix('/')
        .unwrap(); //Gets the key for map

    let map = crate::storage::load_hashmap();

    if (map.contains_key(path)) {
        let long_url = map[path].long_url.clone();
        let status_line = "HTTP/1.1 301 Moved Permanently";
        format!("{status_line}\r\nLocation: {long_url}\r\n\r\n")
    } else {
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    }

    // response
}
