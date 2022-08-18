use std::fs;

const UPTIME_PATH: &str = "/proc/uptime";

pub fn get() -> String {
    let contents = fs::read_to_string(UPTIME_PATH).unwrap();
    let uptime_str = contents.split(".").collect::<Vec<&str>>()[0];
    let uptime = uptime_str.parse::<i32>().unwrap();

    let d = uptime / 60 / 60 / 24;
    let h = uptime / 60 / 60 % 24;
    let m = uptime / 60 % 60;

    return format!("{}d {}h {}m", d, h, m);
}