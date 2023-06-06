extern crate core;

use badgestore::api::password::{generate_password, verify_password};

#[test]
pub fn test_password_works() {
    let password = generate_password().unwrap();
    assert!(verify_password(
        password.write_key.as_slice(),
        password.hash.as_slice()
    ))
}

#[test]
pub fn test_password_fails_if_wrong() {
    let password = generate_password().unwrap();
    assert!(!verify_password("bad".as_bytes(), password.hash.as_slice()))
}
