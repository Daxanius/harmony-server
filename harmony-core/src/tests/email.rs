#[cfg(test)]
use crate::core::validation::validated_types::Email;

#[test]
fn test_valid_emails() {
    let email = Email::new("test@example.com").unwrap();
    assert_eq!(email.value(), "test@example.com");

    let email = Email::new("user.name@domain.co").unwrap();
    assert_eq!(email.value(), "user.name@domain.co");

    let email = Email::new("user_name@domain.com").unwrap();
    assert_eq!(email.value(), "user_name@domain.com");

    let email = Email::new("user-name@sub.domain.com").unwrap();
    assert_eq!(email.value(), "user-name@sub.domain.com");

    let email = Email::new("user123@domain.org").unwrap();
    assert_eq!(email.value(), "user123@domain.org");
}

#[test]
fn test_invalid_emails() {
    let email = Email::new("plainaddress");
    assert!(email.is_err());

    let email = Email::new("user@domain@domain.com");
    assert!(email.is_err());

    let email = Email::new("user@domain");
    assert!(email.is_err());

    let email = Email::new("user@.com");
    assert!(email.is_err());

    let email = Email::new("@domain.com");
    assert!(email.is_err());

    let email = Email::new("user@ domain.com");
    assert!(email.is_err());

    let email = Email::new("user@domain.c ");
    assert!(email.is_err());
}
