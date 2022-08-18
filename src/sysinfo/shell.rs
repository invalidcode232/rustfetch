use std::{env::var};

pub fn get() -> String {
    let path = var("SHELL").ok().unwrap();

    return path.split("/").last().unwrap().to_string();
}