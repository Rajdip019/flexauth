pub struct Validation;

impl Validation {
    pub fn email(email: &str) -> bool {
        // Check if email is valid
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn password(password: &str) -> bool {
        let mut has_alpha = false;
        let mut has_digit = false;

        for c in password.chars() {
            if c.is_ascii_alphabetic() {
                has_alpha = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            }
        }

        has_alpha && has_digit && password.len() >= 8
    }
}
