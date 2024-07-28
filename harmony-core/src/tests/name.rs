#[cfg(test)]
use crate::core::validation::validated_types::Name;

#[test]
fn test_valid_names() {
    let name = Name::new("John Doe").unwrap();
    assert_eq!(name.value(), "John Doe");

    let name = Name::new("Alice123").unwrap();
    assert_eq!(name.value(), "Alice123");

    let name = Name::new("Bob_25").unwrap();
    assert_eq!(name.value(), "Bob_25");

    let name = Name::new("Name With Spaces").unwrap();
    assert_eq!(name.value(), "Name With Spaces");

    let name = Name::new("A12").unwrap();
    assert_eq!(name.value(), "A12");

    let name = Name::new("User_1").unwrap();
    assert_eq!(name.value(), "User_1");
}

#[test]
fn test_invalid_names() {
    let name = Name::new(" JSmith");
    assert!(name.is_err());

    let name = Name::new("JSmith ");
    assert!(name.is_err());

    let name = Name::new(" JSmith ");
    assert!(name.is_err());

    let name = Name::new("JSmith123456789012345"); // More than 16 characters
    assert!(name.is_err());

    let name = Name::new("J$mith");
    assert!(name.is_err());

    let name = Name::new("John\nDoe");
    assert!(name.is_err());

    let name = Name::new("John\tDoe");
    assert!(name.is_err());

    let name = Name::new("  ");
    assert!(name.is_err());

    let name = Name::new("JM");
    assert!(name.is_err());
}
