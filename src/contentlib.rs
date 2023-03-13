pub fn select() {
    println!("called config::select");
}

pub fn print() {
    println!("called config::print");
}

pub fn sayversion() -> String {
    "v0.1.0-dev".to_string()
}
