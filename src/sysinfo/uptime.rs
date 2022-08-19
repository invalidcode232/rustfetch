use std::fs;

const UPTIME_PATH: &str = "/proc/uptime";

pub fn get() -> String {
    let read_uptime = fs::read_to_string(UPTIME_PATH);

    if read_uptime.is_err() {
        println!("error: failed to read uptime");
        return "".to_string();
    }

    let uptime_contents = read_uptime.ok().unwrap();
    let uptime_str = uptime_contents.split(".").collect::<Vec<&str>>()[0];
    let uptime = uptime_str.parse::<i32>().unwrap();

    let d = uptime / 60 / 60 / 24;
    let h = uptime / 60 / 60 % 24;
    let m = uptime / 60 % 60;

    return format!("{}d {}h {}m", d, h, m);
}