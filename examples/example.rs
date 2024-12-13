use rs_openshowvar::OpenShowVar;

// This example code demonstrates how to connect to a robot,
// write to a variable, and read from a variable using the OpenShowVar library.
//
// The OpenShowVar library is used to interact with robot control systems.
// In this example, we show how to write and read variables on a robot.
//
// Before using the library, make sure to add the `openshowvar` crate to your project.
// For more information, refer to the library documentation.
//
// Connection Details:
//
// The IP address and port number used in this example should be adjusted according to your robot's network settings.
// Ensure that the IP address and port number are correct.
//
// Error Handling:
//
// In this example, errors that occur during connection or variable write/read operations are handled with appropriate messages.
// Carefully examine error messages to understand and resolve issues.
//
// Tips and Recommendations:
//
// - While developing your code, you can add additional checks to verify the robot's status after write and read operations.
// - Ensure the IP address and port number are correct before attempting to connect to the robot.

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

    // Check if the connection is active using the is_connected method
    if robot.is_connected() {
        println!("Connection is active.");
    } else {
        println!("Connection is not active.");
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
