#[cfg(test)]
use crate::core::validation::validated_types::Password;

#[test]
fn test_valid_passwords() {
    let password = Password::new("P@ssw0rd").unwrap();
    assert_eq!(password.value(), "P@ssw0rd");

    let password = Password::new("Valid123!").unwrap();
    assert_eq!(password.value(), "Valid123!");

    let password = Password::new("C0mpl3x#Password").unwrap();
    assert_eq!(password.value(), "C0mpl3x#Password");
}

#[test]
fn test_invalid_passwords() {
    let password = Password::new("short");
    assert!(password.is_err());

    let password = Password::new("nocomplexity");
    assert!(password.is_err());

    let password = Password::new("12345678");
    assert!(password.is_err());

    let password = Password::new("Password ");
    assert!(password.is_err());

    let password = Password::new("Password\t");
    assert!(password.is_err());

    let password = Password::new("Password\n");
    assert!(password.is_err());

    let password = Password::new("Password\n123");
    assert!(password.is_err());

    let password = Password::new("      ");
    assert!(password.is_err());

    let password = Password::new(" Passw!rd\n123 ");
    assert!(password.is_err());
}
