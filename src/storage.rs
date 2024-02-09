use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn read_file(file_path: &String) -> String {
    // Create a path to the desired file
    let path = Path::new(file_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {} //print!("{} contains:\n{}", display, s),
    }

    s
}

fn write_file(file_path: &String, content: &String) {
    let path = Path::new(file_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {} //println!("successfully wrote to {}", display),
    }
}

pub fn serialize_hashmap(map: &HashMap<String, crate::types::URLStatusDescription>) -> String {
    let serialized = serde_json::to_string(&map).unwrap();

    // println!("The serialized hashmap is {}", serialized);

    serialized
}

pub fn deserialize_to_hashmap(
    serialized_str: String,
) -> Result<HashMap<String, crate::types::URLStatusDescription>, serde_json::Error> {
    let deserialized: Result<_, serde_json::Error> = serde_json::from_str(&serialized_str);

    // println!("The deserialized hashmap is {:?}", deserialized);

    deserialized
}

pub fn flush_hashmap(map: &HashMap<String, crate::types::URLStatusDescription>) {
    let file_path: String = "temp.txt".to_string();
    let serialized = serialize_hashmap(map);

    write_file(&file_path, &serialized);

    // println!("Flushed hashmap");
}

pub fn load_hashmap() -> HashMap<String, crate::types::URLStatusDescription> {
    let file_path: String = "temp.txt".to_string();
    let serialized_str = read_file(&file_path);
    let map = deserialize_to_hashmap(serialized_str);

    match map {
        Ok(map) => map,
        Err(err) => {
            let mut map: HashMap<String, crate::types::URLStatusDescription> = HashMap::new();
            map
        }
    }
}
