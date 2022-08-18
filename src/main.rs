mod sysinfo;

fn main() {
    // this is just a sample output,
    // will be replaced with the contents of an actual config file.
    let mut output = "
        user: `user`
        uptime: `uptime`
        shell: `shell`
    "
    .to_string();

    output = output.replace("`user`", &sysinfo::user::get());
    output = output.replace("`uptime`", &sysinfo::uptime::get());
    output = output.replace("`shell`", &sysinfo::shell::get());

    println!("{}", output);
}
