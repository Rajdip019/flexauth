pub struct Validation;

impl Validation {
    pub fn email(email: &str) -> bool {
        // Check if email is valid
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn password(password: &str) -> bool {
        // Check if password is valid
        let re = regex::Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
        re.is_match(password)
    }
}
