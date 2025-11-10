use chrono::Utc;

const TIME_FORMAT: &str = "%Y.%m.%d %H:%M:%S.%f";

pub fn now(message: &str) {
    println!("{}: {}", no(), message);
}

fn no() -> String {
    Utc::now().format(TIME_FORMAT).to_string()
}

#[cfg(test)]
mod tests {
    use crate::fractal_log::no;

    #[test]
    fn test_no() {
        let n = no();
        assert_ne!(n.len(), 0);
    }
}
