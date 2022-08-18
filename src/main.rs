mod sysinfo;

fn main() {
    let mut output = "
        user: `user`
        uptime: `uptime`
    ".to_string();

    output = output.replace("`user`", &sysinfo::user::get().unwrap());
    output = output.replace("`uptime`", &sysinfo::uptime::get());

    println!("{}", output);
}
