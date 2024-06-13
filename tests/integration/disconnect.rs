use rs_openshowvar::OpenShowVar;

// WARNING: These tests use real connections to test the functionality of the library that implements the OpenShowVar protocol.
// Since they work with real data, running these tests may lead to unintended consequences in robot systems.
// Please run these tests only in development environments and with caution.
// Make sure to adjust your connection configurations and test data to fit your own environment.

// Tests the disconnection functionality by connecting to an OpenShowVar server,
// then disconnecting and verifying that the connection has been closed.
#[test]
fn test_disconnect() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().expect("Connection failed");
    // Disconnecting from OpenShowVar
    osv.disconnect();
    // Verifying that the connection has been successfully closed and is now set to None
    assert!(osv.conn.is_none());
}
