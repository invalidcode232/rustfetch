use std::{env::var};

pub fn get() -> Option<String> {
    let user = var("USER").ok();

    return user;
}