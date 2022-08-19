use std::{env::var};

pub fn get() -> String {
    let shell = var("SHELL").ok();

    if shell.is_none() {
        println!("error: failed to fetch shell");
        return "".to_string();
    }

    shell.unwrap().split("/").last().unwrap().to_string()
}