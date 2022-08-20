use std::env::var;

pub fn get() -> String {
    let user = var("USER").ok();

    if user.is_none() {
        println!("error: failed to fetch username");
        return "".to_string();
    }

    user.unwrap()
}

