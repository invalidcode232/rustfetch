mod sysinfo;

fn main() {
    // this is just a sample output, 
    // will be replaced with the contents of an actual config file.
    let mut output = "
        user: `user`
        uptime: `uptime`
    ".to_string();

    output = output.replace("`user`", &sysinfo::user::get().unwrap());
    output = output.replace("`uptime`", &sysinfo::uptime::get());

    println!("{}", output);
}
