extern crate url_shortener;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};

//These are for example purposes only
const LONG_URL: &str =
    "https://www.google.com/search?channel=fs&client=ubuntu-sn&q=meow+meow";
const RATE_LIMIT: Option<u64> = Some(500000000000);
const PERMISSION_RULE: Option<String> = None;

fn main() {
    let request = url_shortener::types::URLCreationDescription {
        long_url: LONG_URL.to_string(),
        rate_limit: RATE_LIMIT,
        permission_rule: PERMISSION_RULE,
    };

    let short_url = request.get_hash();

    println!("The request struct is {:?}", request);
    println!("The hash is {:?}", short_url);

    let mut map: HashMap<String, url_shortener::types::URLStatusDescription> = HashMap::new();

    let record = url_shortener::types::URLStatusDescription {
        long_url: LONG_URL.to_string(),
        rate_limit: RATE_LIMIT,
        permission_rule: PERMISSION_RULE,
    };

    map.insert(short_url.clone(), record);

    url_shortener::storage::flush_hashmap(&map);

    println!("{:?}", map[&short_url]);

    let read_map = url_shortener::storage::load_hashmap();
    println!("The read hashmap is {:?}",read_map);
}
