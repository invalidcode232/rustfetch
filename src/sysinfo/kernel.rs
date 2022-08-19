use std::fs;

const KERNEL_PATH: &str = "/proc/version";

pub fn get() -> String {
    let read_file = fs::read_to_string(KERNEL_PATH);
    if read_file.is_err() {
        println!("error: failed to get kernel info");
        return "".to_string();
    }

    let contents = read_file.unwrap();
    let kernel = contents.split(" ").collect::<Vec<&str>>()[2];

    return kernel.to_string();
}
