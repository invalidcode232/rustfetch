use std::fs;

const OS_PATH: &str = "/etc/os-release";

pub fn get() -> String {
    let read_os = fs::read_to_string(OS_PATH);

    if read_os.is_err() {
        println!("error: failed to fetch distro");
        return "".to_string();
    }

    let os_content = read_os.unwrap();

    let os_data = os_content.split("\n").collect::<Vec<&str>>();
    let pretty_name = os_data[0];

    let distro_name = pretty_name.split("=").collect::<Vec<&str>>()[1];

    distro_name.to_string().replace("\"", "")
}