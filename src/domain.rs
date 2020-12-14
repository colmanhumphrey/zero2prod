use unicode_segmentation::UnicodeSegmentation;

pub fn generic_string_checks(s: &str) -> bool {
    let empty_or_whitespace = s.trim().is_empty();

    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_chars = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_chars = s.chars().filter(|g| forbidden_chars.contains(g)).count() > 0;

    !(empty_or_whitespace || is_too_long || contains_forbidden_chars)
}

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

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if !generic_string_checks(&s) {
            return Err(format!("{} is not a valid subscriber email.", s));
        }

        if !s.contains("@") {
            return Err(format!("{} is not a valid subscriber email.", s));
        }

        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}

impl NewSubscriber {
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use crate::domain::SubscriberEmail;
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

    #[test]
    fn email_without_atsym_fails() {
        let email = "colatgmail.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn valid_email_all_good() {
        let email = "col@gmail.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
