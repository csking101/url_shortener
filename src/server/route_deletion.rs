pub fn handle_delete(http_request: &Vec<String>) -> String {
    let path = *&http_request[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .strip_prefix('/')
        .unwrap(); //Gets the key for map

    let mut map = crate::storage::load_hashmap();

    match map.remove(path) {
        Some(_) => {
            println!("Route was deleted");
            crate::storage::flush_hashmap(&map);
            "HTTP/1.1 200 OK\r\n\r\n".to_string()
        }
        None => {
            println!("Route to delete non-existent");
            "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
        }
    }
}
