use rs_openshowvar::OpenShowVar;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

// Helper function to start a mock server.
//
// This function contains the necessary code to start a TCP server for use in tests.
// It returns a TCP listener and a thread handle that manages the server.
// Instead of a real server, a simulated server is created for use in tests.
// Details of the server implementation are not covered here.
fn start_mock_server() -> (TcpListener, std::thread::JoinHandle<()>) {
    // Bind a TCP listener to a random port on localhost
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    // Spawn a thread to handle incoming connections
    let handle = thread::spawn({
        let listener = listener.try_clone().unwrap();
        move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).unwrap();
                // Process the request and send back the appropriate response
                let response = process_request(&buffer[..n]);
                stream.write_all(&response).unwrap();
            }
        }
    });
    (listener, handle)
}

// Processes a mock request and generates the appropriate response.
//
// This function is responsible for processing a mock request and generating
// the appropriate response. In this implementation, the request is simply echoed back.
fn process_request(request: &[u8]) -> Vec<u8> {
    // Here we should process the request and generate the appropriate response.
    // For simplicity, we will just echo back the request.
    let response = request.to_vec();

    // Modify the response to match the expected behavior in tests
    if request[4] == 1 {
        // For write requests, echo only the value part with the necessary header
        let var_name_len = ((request[5] as usize) << 8) | (request[6] as usize);
        let val_len =
            ((request[7 + var_name_len] as usize) << 8) | (request[7 + var_name_len + 1] as usize);
        let mut response = vec![
            0,
            0,
            0,
            (3 + val_len) as u8,
            1,
            (val_len >> 8) as u8,
            (val_len & 0xFF) as u8,
        ];
        response.extend_from_slice(&request[7 + var_name_len + 2..]);
        response
    } else {
        // For read requests, echo the entire request
        response
    }
}

// Tests the `connect` method of the `OpenShowVar` struct for successful connection.
#[test]
fn test_connect() {
    // Start a mock server
    let (listener, _handle) = start_mock_server();
    let addr = listener.local_addr().unwrap();

    // Create an `OpenShowVar` instance and connect to the mock server
    let mut osv = OpenShowVar::new(addr.ip().to_string(), addr.port());
    assert!(osv.connect().is_ok());
    // Check if connection is established
    assert!(osv.conn.is_some());
}

// Tests the `send` method of the `OpenShowVar` struct.
#[test]
fn test_send() {
    // Start a mock server
    let (listener, _handle) = start_mock_server();
    let addr = listener.local_addr().unwrap();

    // Create an `OpenShowVar` instance and connect to the mock server
    let mut osv = OpenShowVar::new(addr.ip().to_string(), addr.port());
    osv.connect().unwrap();

    // Test sending data to the mock server
    let response = osv.send("existing_var", "new_value");
    assert!(response.is_ok());
}

// Tests the `read` method of the `OpenShowVar` struct.
#[test]
fn test_read() {
    // Start a mock server
    let (listener, _handle) = start_mock_server();
    let addr = listener.local_addr().unwrap();

    // Create an `OpenShowVar` instance and connect to the mock server
    let mut osv = OpenShowVar::new(addr.ip().to_string(), addr.port());
    osv.connect().unwrap();

    // Test reading data from the mock server
    let response = osv.read("existing_var");
    assert!(response.is_ok());
    assert_eq!(response.unwrap(), "existing_var");
}

// Tests the `write` method of the `OpenShowVar` struct.
#[test]
fn test_write() {
    // Start a mock server
    let (listener, _handle) = start_mock_server();
    let addr = listener.local_addr().unwrap();

    // Create an `OpenShowVar` instance and connect to the mock server
    let mut osv = OpenShowVar::new(addr.ip().to_string(), addr.port());
    osv.connect().unwrap();

    // Test writing data to the mock server
    let response = osv.write("existing_var", "test_val");
    assert!(response.is_ok());
    assert_eq!(response.unwrap(), "test_val");
}

// Tests the `disconnect` method of the `OpenShowVar` struct.
#[test]
fn test_disconnect() {
    // Start a mock server
    let (listener, _handle) = start_mock_server();
    let addr = listener.local_addr().unwrap();

    // Create an `OpenShowVar` instance and connect to the mock server
    let mut osv = OpenShowVar::new(addr.ip().to_string(), addr.port());
    osv.connect().unwrap();
    // Disconnect from the mock server
    osv.disconnect();

    // Check if connection is closed
    assert!(osv.conn.is_none());
}
