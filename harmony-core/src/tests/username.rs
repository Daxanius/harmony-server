#[cfg(test)]
use crate::core::validation::validated_types::Username;

#[test]
fn test_valid_usernames() {
    let username = Username::new("user123").unwrap();
    assert_eq!(username.value(), "user123");

    let username = Username::new("user_name").unwrap();
    assert_eq!(username.value(), "user_name");

    let username = Username::new("User1234").unwrap();
    assert_eq!(username.value(), "User1234");

    let username = Username::new("u1_").unwrap();
    assert_eq!(username.value(), "u1_");

    let username: Username = Username::new("valid_user123").unwrap();
    assert_eq!(username.value(), "valid_user123");
}

#[test]
fn test_invalid_usernames() {
    let username = Username::new("us");
    assert!(username.is_err());

    let username = Username::new("user@name");
    assert!(username.is_err());

    let username = Username::new("user-name");
    assert!(username.is_err());

    let username = Username::new("user name");
    assert!(username.is_err());

    let username = Username::new("user!name");
    assert!(username.is_err());

    let username = Username::new("longusername12345");
    assert!(username.is_err());

    let username = Username::new("");
    assert!(username.is_err());

    let username = Username::new(" invalid_user123");
    assert!(username.is_err());
}
