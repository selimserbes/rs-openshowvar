use rs_openshowvar::OpenShowVar;

// WARNING: These tests use real connections to test the functionality of the library that implements the OpenShowVar protocol.
// Since they work with real data, running these tests may lead to unintended consequences in robot systems.
// Please run these tests only in development environments and with caution.
// Make sure to adjust your connection configurations and test data to fit your own environment.

// Test reading a variable successfully.
#[test]
fn test_read_success() {
    // Creating an instance of OpenShowVar with a valid IP address and port
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);

    // Attempting to connect to OpenShowVar server
    osv.connect().expect("Connection failed");

    // Successfully read a variable
    assert!(osv.read("existing_var").is_ok());
}

// Test attempting to read a variable with an empty name.
#[test]
fn test_read_empty_variable_name() {
    // Creating an instance of OpenShowVar with a valid IP address and port
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);

    // Attempting to connect to OpenShowVar server
    osv.connect().expect("Connection failed");

    // An error should be returned when attempting to read a variable with an empty name
    assert!(osv.read("").is_err());
}

// Test attempting to read a non-existent variable.
#[test]
fn test_read_variable_not_found() {
    // Creating an instance of OpenShowVar with a valid IP address and port
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);

    // Attempting to connect to OpenShowVar server
    osv.connect().expect("Connection failed");

    // An error should be returned when attempting to read a variable that does not exist
    assert!(osv.read("non_existing_var").is_err());
}
