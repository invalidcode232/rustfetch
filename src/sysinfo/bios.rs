use std::fs;

const BIOS_RELEASE: &str = "/sys/devices/virtual/dmi/id/bios_release";
pub fn get() -> String {
    let bios_read = fs::read_to_string(BIOS_RELEASE);

    if bios_read.is_err() {
        println!("error: failed to read bios version");
        return "".to_string();
    }

    return bios_read.unwrap();
}
