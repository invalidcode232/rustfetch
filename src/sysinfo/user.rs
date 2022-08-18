use std::{env::var};

pub fn get() -> String {
    return var("USER").ok().unwrap();
}