use std::{io::BufReader, net::TcpStream};

pub fn handle_get(http_request: &Vec<String>) -> String {
    let path = *&http_request[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .strip_prefix('/')
        .unwrap(); //Gets the key for map

    let mut map = crate::storage::load_hashmap();

    if map.contains_key(path) {
        let long_url = map[path].long_url.clone();

        match map[path].rate_limit {
            Some(rate_limit) => {
                if (rate_limit > 0) {
                    let status_line = "HTTP/1.1 302 Found";

                    let new_url_status_description = crate::types::URLStatusDescription {
                        rate_limit: Some(rate_limit - 1),
                        long_url: map[path].long_url.clone(),
                        permission_rule: map[path].permission_rule.clone(),
                    };

                    map.insert(path.to_string(), new_url_status_description);

                    crate::storage::flush_hashmap(&map);
                    println!("Valid GET request!");
                    format!("{status_line}\r\nLocation: {long_url}\r\n\r\n")
                } else {
                    println!("Valid GET but Rate limited");
                    "HTTP/1.1 429 Too Many Requests".to_string()
                }
            }
            None => {
                let status_line = "HTTP/1.1 302 Found";
                println!("Valid GET request!");
                format!("{status_line}\r\nLocation: {long_url}\r\n\r\n")
            }
        }
    } else {
        println!("Invalid GET request!");
        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
    }
}
