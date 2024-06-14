use rs_openshowvar::OpenShowVar;
use std::thread;
use std::time::Duration;

// WARNING: These tests use real connections to test the functionality of the library that implements the OpenShowVar protocol.
// Since they work with real data, running these tests may lead to unintended consequences in robot systems.
// Please run these tests only in development environments and with caution.
// Make sure to adjust your connection configurations and test data to fit your own environment.

// Tests writing and reading operations with a real robot.
// This test connects to an OpenShowVar server running on a real robot,
// writes a value to a specified variable, waits for a short duration,
// then reads the same variable to ensure the value matches the written one.
#[test]
fn test_write_and_read_with_real_robot() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().unwrap();

    // Defining the value to be written and the variable name
    let value = "new_value";
    let variable_name = "existing_var";

    // Performing the variable writing operation
    let write_result = osv.write(variable_name, &value.to_string());
    assert!(write_result.is_ok(), "Variable writing failed");

    // Waiting for the value to propagate
    thread::sleep(Duration::from_secs(1));

    // Performing the variable reading operation
    let read_result = osv.read(variable_name);
    assert!(read_result.is_ok(), "Variable reading failed");
    assert_eq!(
        read_result.unwrap(),
        value.to_string(),
        "Read value is different than expected"
    );

    // Closing the OpenShowVar connection
    osv.disconnect();
}

// Tests error handling in OpenShowVar operations.
// This test checks error handling behavior for various scenarios such as
// connecting with an invalid IP address, reading from a non-existent variable,
// and disconnecting when there is no active connection.
#[test]
fn test_open_show_var_error_handling() {
    // Testing connection with an invalid IP address
    let mut osv = OpenShowVar::new("invalid_ip_address".to_string(), 7000);
    assert!(
        osv.connect().is_err(),
        "Successful connection with an invalid IP address"
    );

    // Testing reading a non-existent variable
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().unwrap();
    assert!(
        osv.read("non_existing_var").is_err(),
        "Successful reading of a non-existent variable"
    );

    // Testing disconnecting without an active connection
    osv.disconnect();
    assert!(
        osv.conn.is_none(),
        "Successful disconnection without an active connection"
    );
}

// Tests the performance of OpenShowVar operations.
// This test measures the performance of write and read operations by repeatedly writing
// a variable and immediately reading it back, ensuring that the written and read values match.
#[test]
fn test_open_show_var_performance() {
    // Establishing a connection to OpenShowVar
    let mut osv = OpenShowVar::new("192.168.1.10".to_string(), 7000);
    osv.connect().unwrap();

    // Writing and reading variable values in a loop
    for i in 0..100 {
        let variable_name = "existing_var";
        osv.write(&variable_name, &i.to_string()).unwrap();
        let read_value = osv.read(&variable_name).unwrap();
        assert_eq!(
            read_value,
            i.to_string(),
            "Written and read values do not match"
        );
    }

    // Closing the OpenShowVar connection
    osv.disconnect();
}
