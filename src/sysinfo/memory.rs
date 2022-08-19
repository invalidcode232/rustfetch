use std::fs;

const MEMORY_PATH: &str = "/proc/meminfo";

pub fn get() -> String {
    let read_memory = fs::read_to_string(MEMORY_PATH);

    if read_memory.is_err() {
        println!("error: failed to read memory");
        return "".to_string();
    }

    let memory_contents = read_memory.unwrap();
    let memory_data = memory_contents.split("\n").collect::<Vec<&str>>();
    let free_memory_line = memory_data[2].to_string();
    let free_memory = free_memory_line.replace("MemAvailable:", "").replace("kB", "");

    let free_memory_int: i32 = free_memory.trim().parse().unwrap();

    return (free_memory_int / 1024).to_string() + "MiB";
}