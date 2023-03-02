use chrono::Utc;

const TIME_FORMAT: &str = "%Y.%m.%d %H:%M:%S.%f";

pub fn now(message: &str) {
    println!("{}: {}", no(), message);
}

const X: usize = 249;
const Y: usize = 248;

pub fn only(x: usize, y: usize, message: &str) {
    if x == X {
        if y == Y {
            println!("* {}: {}", no(), message);
        }
    }
}

fn no() -> String {
    Utc::now().format(TIME_FORMAT).to_string()
}

