extern crate url_shortener;

fn main() {
    let long_url =
        "https://www.google.com/search?channel=fs&client=ubuntu-sn&q=meow+meow".to_string();
    let rate_limit = Some(500000000000);
    let permission_rule = None;

    let request = url_shortener::types::URLCreationDescription {
        long_url,
        rate_limit,
        permission_rule,
    };

    println!("The request struct is {:?}", request);
    println!("The hash is {:?}", request.get_hash());
}
