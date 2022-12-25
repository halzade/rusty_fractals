use colored::Colorize;

pub fn log(s: &str) {
    println!("{} {} !", "message: ".green(), s.blue().bold());
}

#[test]
fn test_log() {
    log("hi!");
}