use crate::domain::string_checks::generic_string_checks;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        if !generic_string_checks(&s) {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_graphene_long_name_is_valid() {
        let name = format!("{}{}", "a".repeat(255), "💻");
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_longer_than_256_graphene_long_name_is_rejected() {
        let name = format!("{}{}", "a".repeat(256), "💻");
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn invalid_chars_in_name_rejected() {
        for name in vec!['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_all_good() {
        let name = "Colman Humphrey".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
