use std::{io::BufReader, net::TcpStream};

use crate::types::URLCreationDescription;

pub fn handle_post(http_request: &Vec<String>) -> String {
    //The post route will just help the user post once, not update

    let path = *&http_request[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .strip_prefix('/')
        .unwrap(); //Gets the key for map

    let mut map = crate::storage::load_hashmap();

    if "create_short_url" == path {
        let json_string = http_request.iter().rev().next().unwrap();

        let creation_description: URLCreationDescription =
            serde_json::from_str(&json_string).unwrap();

        let (short_url, status_description) = creation_description.create_url_status_description();

        if map.contains_key(&short_url) {
            //If already present, don't overwrite
            println!("There is a conflict");
            "HTTP/1.1 409 Conflict\r\n\r\n".to_string()
        } else {
            map.insert(short_url, status_description);

            crate::storage::flush_hashmap(&map);

            println!("Route was created!");
            "HTTP/1.1 201 Created\r\n\r\n".to_string()
        }
    } else {
        "HTTP/1.1 400 Bad Request\r\n\r\n".to_string()
    }
}
