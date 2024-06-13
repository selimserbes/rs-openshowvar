use rs_openshowvar::OpenShowVar;

// WARNING: These tests use real connections to test the functionality of the library that implements the OpenShowVar protocol.
// Since they work with real data, running these tests may lead to unintended consequences in robot systems.
// Please run these tests only in development environments and with caution.
// Make sure to adjust your connection configurations and test data to fit your own environment.

// Connects to the OpenShowVar server with a valid IP address and port,
// then checks if the connection is successful.
#[test]
fn test_connect_success() {
    // Creating an instance of OpenShowVar with a valid IP address and port
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);

    // The connection attempt should be successful
    assert!(osv.connect().is_ok());

    // The `conn` field should be `Some(_)` after successful connection
    assert!(osv.conn.is_some());
}

// Tries to connect to the OpenShowVar server with an invalid IP address,
// then verifies that the connection attempt fails.
#[test]
fn test_connect_failure() {
    // Creating an instance of OpenShowVar with an invalid IP address and port
    let mut osv = OpenShowVar::new("invalid_ip".to_string(), 7000);

    // The connection attempt should fail
    assert!(osv.connect().is_err());

    // The `conn` field should be `None` after failed connection attempt
    assert!(osv.conn.is_none());
}
