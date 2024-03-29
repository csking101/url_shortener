use crate::types::{URLCreationDescription, URLStatusDescription};

static ALPHANUMERIC: &'static str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const RADIX: u64 = 62;

fn get_shortened_url(mut hash_result: u64) -> String {
    let mut short_url = String::new();

    loop {
        if hash_result == 0 {
            break;
        }

        let idx = (hash_result % RADIX) as usize;

        short_url.push(ALPHANUMERIC.chars().nth(idx).unwrap());
        hash_result /= RADIX;
    }

    short_url
}

impl URLCreationDescription {
    pub fn get_hash(&self) -> String {
        let mut hash_result: u64 = 0;

        // Creates the numeric representation of the struct
        match &self.permission_rule {
            Some(s) => {
                //FIXME: Add support for permission rules
                println!("The permission rule is {}", s);
            }
            None => {
                hash_result += self.long_url.chars().map(|c| c as u64).sum::<u64>();

                match self.rate_limit {
                    Some(rate_limit) => {
                        hash_result += rate_limit;
                    }
                    None => {}
                }
            }
        }

        get_shortened_url(hash_result)
    }

    pub fn create_url_status_description(&self) -> (String, URLStatusDescription) {
        let short_url = self.get_hash();
        let status_struct = URLStatusDescription {
            long_url: self.long_url.clone(),
            rate_limit: self.rate_limit,
            permission_rule: self.permission_rule.clone(),
        };

        (short_url, status_struct)
    }
}
