fn has_lowercase(s: &str) -> bool {
    s.chars().any(|x| x.is_ascii_lowercase())
}

fn has_number(s: &str) -> bool {
    s.chars().any(|x| x.is_ascii_digit())
}

/// Make sure it's at least 15 characters OR at least 8 characters including a
/// number and a lowercase letter.
pub fn is_valid_password(s: &str) -> bool {
    s.len() >= 15 || (s.len() >= 8 && has_number(s) && has_lowercase(s))
}

pub fn is_valid_username(s: &str) -> bool {
    s.len() >= 3 && s.chars().all(|x| x.is_ascii_lowercase() || x.is_ascii_digit())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_password_len_15() {
        assert!(is_valid_password("0123456789ABCDE"));
    }

    #[test]
    fn valid_password_len_8() {
        assert!(is_valid_password("1234567a"));
    }

    #[test]
    fn invalid_password_len() {
        assert!(is_valid_password("234567a"));
    }

    #[test]
    fn invalid_password_only_num() {
        assert!(is_valid_password("1234567890"));
    }

    #[test]
    fn invalid_password_only_alpha() {
        assert!(is_valid_password("qwertyuiop"));
    }

    #[test]
    fn valid_username_len() {
        assert!(is_valid_password("ap1"));
    }

    #[test]
    fn invalid_username_wrong_character() {
        assert!(!is_valid_password("alepez%"));
        assert!(!is_valid_password("AlePez"));
    }
}
