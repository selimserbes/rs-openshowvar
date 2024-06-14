# rs_openshowvar

`rs_openshowvar` is a Rust library for interacting with Kuka robots over TCP/IP using the OpenShowVar protocol.

## Overview

`rs_openshowvar` is a Rust library that facilitates connecting to robot systems via TCP/IP to perform read and write operations. Targeting the KukaVarProxy server, it allows access to Kuka robots using the OpenShowVar protocol. This library is designed for use in robot control and monitoring applications, providing reliable communication over TCP/IP and extensive functionality for managing robot variables.

## KukaVarProxy Message Format

The communication with KukaVarProxy follows this message format:

- msg ID in HEX (2 bytes)
- msg length in HEX (2 bytes)
- read (0) or write (1) indicator (1 byte)
- variable name length in HEX (2 bytes)
- variable name in ASCII (# bytes)
- variable value length in HEX (2 bytes)
- variable value in ASCII (# bytes)

## Installation

Add `rs_openshowvar` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
rs_openshowvar = "0.1.5"
```

## Usage

### KukaVarProxy

This library is designed to connect to Kuka robots via the KukaVarProxy server. KukaVarProxy is server software used to access Kuka robot system variables over TCP/IP.

1. To install and configure KukaVarProxy, please follow the instructions provided in the [KukaVarProxy GitHub repository](https://github.com/ImtsSrl/KUKAVARPROXY).

2. **Ensure that `rs_openshowvar` is configured to use port `7000` to connect to the KukaVarProxy server.** KukaVarProxy listens on this port to communicate with Kuka robots.

3. Create your own program using the `rs_openshowvar` library to connect to your robot. Below is an example:

```rust
use rs_openshowvar::OpenShowVar;

pub fn main() {
    // Create an instance of OpenShowVar with the robot's IP address and port
    let mut robot = OpenShowVar::new("192.168.1.10".to_string(), 7000);

    // Connect to the robot
    match robot.connect() {
        Ok(_) => println!("Connected to the robot"),
        Err(e) => {
            // Display the connection error and terminate the process
            println!("Connection error: {}", e);
            return;
        }
    }

    // Specify the variable name and value
    let variable_name = "existing_var";
    let value = "new_value";

    // Write to the variable
    match robot.write(variable_name, value) {
        Ok(_) => println!("Variable written successfully"),
        Err(e) => println!("Error writing variable: {}", e),
    }

    // Read from the variable
    match robot.read(variable_name) {
        Ok(read_value) => println!("Read value: {}", read_value),
        Err(e) => println!("Error reading variable: {}", e),
    }
    // Disconnect from the robot
    robot.disconnect();
}
```

## Documentation

For detailed API documentation and usage examples, visit the [Documentation](https://docs.rs/rs_openshowvar).

## Contributing

Contributions are welcome! If you'd like to contribute to `rs_openshowvar`, please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
