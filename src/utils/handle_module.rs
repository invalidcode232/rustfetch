use colored::Colorize;

pub fn format(out: &mut String, find: &str, replace: String) {
    if out.find(find).is_some() {
        *out = out.replace(find, &replace.blue().bold().to_string());
    }
}