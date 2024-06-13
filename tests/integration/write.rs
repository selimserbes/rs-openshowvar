use rs_openshowvar::OpenShowVar;

// WARNING: These tests use real connections to test the functionality of the library that implements the OpenShowVar protocol.
// Since they work with real data, running these tests may lead to unintended consequences in robot systems.
// Please run these tests only in development environments and with caution.
// Make sure to adjust your connection configurations and test data to fit your own environment.

// Test writing a value to a variable successfully.
#[test]
fn test_write_success() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.1.168.10".to_string(), 7000);
    osv.connect().expect("Connection failed");

    // Write a new value
    assert!(osv.write("existing_var", "new_value").is_ok());
}

// Test attempting to write to a variable with an empty name.
#[test]
fn test_write_empty_variable_name() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().expect("Connection failed");

    // An error should be returned when attempting to write to a variable with an empty name
    assert!(osv.write("", "new_value").is_err());
}

// Test attempting to write an empty value to a variable.
#[test]
fn test_write_empty_value() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().expect("Connection failed");

    // An error should be returned when attempting to write an empty value
    assert!(osv.write("existing_var", "").is_err());
}

// Test attempting to write a value to a non-existent variable.
#[test]
fn test_write_variable_not_found() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().expect("Connection failed");

    // An error should be returned when attempting to write a value to a variable that does not exist
    assert!(osv.write("non_existing_var", "new_value").is_err());
}
