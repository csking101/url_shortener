static ALPHANUMERIC: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const RADIX:u64 = 62;

// This struct is used for making the request
#[derive(Debug)]
pub struct URLCreationDescription {
    pub long_url: String,
    pub rate_limit: Option<u64>,
    pub permission_rule: Option<String>,
}

// This struct is used for storing in the DB (this is the value of the global hashmap's key)
#[derive(Debug)]
pub struct URLStatusDescription {
    pub long_url: String,
    pub rate_limit: Option<u64>,
    pub permission_rule: Option<String>,
}


fn get_shortened_url (mut hash_result:u64)-> String{
    let mut short_url = String::new();

    loop {
        if hash_result == 0{
            break;
        }

        let idx = (hash_result % RADIX) as usize;

        short_url.push(ALPHANUMERIC.chars().nth(idx).unwrap());
        hash_result /= RADIX;
    }

    short_url
}

impl URLCreationDescription {
    pub fn get_hash(&self) -> String{
        let mut hash_result:u64 = 0;

        // Creates the numeric representation of the struct
        match &self.permission_rule {
            Some(s) => {
                //FIXME: Add support for permission rules
                println!("The permission rule is {}",s);
            },
            None => {
                hash_result += self.long_url.chars().map(|c| c as u64).sum::<u64>();
                hash_result *= self.rate_limit.unwrap();
            }
        }

        get_shortened_url(hash_result)
    }


}