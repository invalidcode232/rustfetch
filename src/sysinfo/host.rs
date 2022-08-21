use std::fs;

const HOSTNAME_PATH: &str = "/etc/hostname";
pub fn get() -> String {
    let hostname_read = fs::read_to_string(HOSTNAME_PATH);

    if hostname_read.is_err() {
        println!("error: failed to retrieve hostname");
        return "".to_string();
    }

    return hostname_read.unwrap();
}
