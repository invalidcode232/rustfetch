use std::{fs::OpenOptions, io::Read};

mod sysinfo;
mod utils;

fn main() {
    let user = sysinfo::user::get();
    let mut config_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(format!("/home/{}/.config/rustfetch", &user))
        .unwrap();

    let mut config = String::new();
    let res = config_file.read_to_string(&mut config);
    if res.is_err() {
        panic!("error: failed to read config file");
    }

    utils::handle_module::format(&mut config, "`user`", sysinfo::user::get());
    utils::handle_module::format(&mut config, "`uptime`", sysinfo::uptime::get());
    utils::handle_module::format(&mut config, "`shell`", sysinfo::shell::get());
    utils::handle_module::format(&mut config, "`kernel`", sysinfo::kernel::get());
    utils::handle_module::format(&mut config, "`free`", sysinfo::memory::get());
    utils::handle_module::format(&mut config, "`distro`", sysinfo::distro::get());
    // utils::handle_module::format(&mut config, "`host`", sysinfo::host::get());
    utils::handle_module::format(&mut config, "`bios`", sysinfo::bios::get());

    println!("{}", config);
}
