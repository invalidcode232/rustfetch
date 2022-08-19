pub fn format(out: &mut String, find: &str, replace: String) {
    if out.find(find).is_some() {
        let clr_replace = format!("\x1b[1m\x1b[94m{}\x1b[0m", replace);
        *out = out.replace(find, &clr_replace);
        // *out = out.replace(find, format);
    }
}