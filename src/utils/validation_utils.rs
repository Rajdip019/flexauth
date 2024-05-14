pub struct Validation;

impl Validation {
    pub fn email(email: &str) -> bool {
        // Check if email is valid
        let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn password(password: &str) -> bool {
        // Minimum length requirement
        let min_length = 8;
        if password.len() < min_length {
            return false;
        }

        // Check for at least one lowercase letter
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        if !has_lowercase {
            return false;
        }

        // Check for at least one uppercase letter
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        if !has_uppercase {
            return false;
        }

        // Check for at least one number
        let has_number = password.chars().any(|c| c.is_numeric());
        if !has_number {
            return false;
        }

        // Check for at least one special character
        let has_special = password.chars().any(|c| c.is_ascii_punctuation());
        if !has_special {
            return false;
        }

        // No whitespace allowed
        if password.contains(' ') {
            return false;
        }

        // Password is valid
        return true;
    }
}
