use std::{env::var};
use crate::sysinfo::sysmodule;

pub fn get() -> sysmodule::Module<String> {
    let user = var("USER").ok();

    return sysmodule::Module {
        label: "user",
        value: user,
    };
}