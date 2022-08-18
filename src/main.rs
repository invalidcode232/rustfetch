mod sysinfo;

fn main() {
    let user =  sysinfo::user::get();

    if user.value.is_some() {
        println!("{}: {}", user.label, user.value.unwrap());
    }
}
