use std::io::{Read, Write};
use std::net::TcpStream;

/// The `OpenShowVar` structure is used to connect to a robot control system and read/write variable values over a TCP connection.
pub struct OpenShowVar {
    /// IP address for the TCP connection.
    tcp_ip: String,
    /// Port number for the TCP connection.
    tcp_port: u16,
    /// TCP connection.
    pub conn: Option<TcpStream>,
}

impl OpenShowVar {
    /// Creates a new instance of `OpenShowVar`.
    ///
    /// # Arguments
    ///
    /// * `tcp_ip` - IP address of the TCP server to connect to.
    /// * `tcp_port` - Port number of the TCP server to connect to.
    ///
    /// # Returns
    ///
    /// Returns a new instance of OpenShowVar.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// ```
    pub fn new(tcp_ip: String, tcp_port: u16) -> OpenShowVar {
        OpenShowVar {
            tcp_ip,
            tcp_port,
            conn: None,
        }
    }

    /// Connects to the TCP server.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the TCP connection is successful.
    /// Returns `std::io::Error` if the TCP connection fails.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// match osv.connect() {
    ///     Ok(_) => println!("Connection successful"),
    ///     Err(e) => println!("Connection error: {}", e),
    /// }
    /// ```
    pub fn connect(&mut self) -> std::io::Result<()> {
        // Create address by combining IP address and port number
        let addr = format!("{}:{}", self.tcp_ip, self.tcp_port);
        // Establish TCP connection
        self.conn = Some(TcpStream::connect(addr)?);
        Ok(())
    }

    /// Sends a variable value.
    ///
    /// # Arguments
    ///
    /// * `var_name` - Name of the variable to send.
    /// * `val` - Value of the variable to send.
    ///
    /// # Returns
    ///
    /// Returns a `std::io::Result` depending on the sent data.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// match osv.send("variable_name", "value") {
    ///     Ok(data) => println!("Data sent: {:?}", data),
    ///     Err(e) => println!("Sending error: {}", e),
    /// }
    /// ```
    pub fn send(&mut self, var_name: &str, val: &str) -> std::io::Result<Vec<u8>> {
        // Calculate lengths of variable name and value
        let mut msg = Vec::new();
        let mut temp = Vec::new();

        // If value exists, add its length and value
        if !val.is_empty() {
            let val_len = val.len();
            msg.push(((val_len & 0xff00) >> 8) as u8);
            msg.push((val_len & 0x00ff) as u8);
            msg.extend_from_slice(val.as_bytes());
        }

        // If variable name exists, add its length and name
        if !var_name.is_empty() {
            let var_name_len = var_name.len();
            temp.push(((var_name_len & 0xff00) >> 8) as u8);
            temp.push((var_name_len & 0x00ff) as u8);
            temp.extend_from_slice(var_name.as_bytes());
        }

        // If value exists, indicate its presence by adding a byte. Indicates whether it's Read or Write
        if !val.is_empty() {
            temp.insert(0, 1);
        } else {
            temp.insert(0, 0);
        }

        msg = [&temp[..], &msg[..]].concat();

        // Calculate message length and create header
        let msg_len = msg.len() as u16;
        let mut header = vec![0u8; 4];
        header[0] = ((0 & 0xff00) >> 8) as u8;
        header[1] = (0 & 0x00ff) as u8;
        header[2] = ((msg_len & 0xff00) >> 8) as u8;
        header[3] = (msg_len & 0x00ff) as u8;

        // Create request by combining header and message
        let request = [&header[..], &msg[..]].concat();

        // If connection exists, send the request
        if let Some(ref mut conn) = self.conn {
            conn.write_all(&request)?;
            let mut response = vec![0u8; 1024];
            let n = conn.read(&mut response)?;
            response.truncate(n);

            // Filter visible characters and process the response
            let visible_chars: Vec<u8> = response
                .iter()
                .cloned()
                .filter(|&byte| byte >= 32 && byte <= 126)
                .collect();

            let response_str = String::from_utf8_lossy(&visible_chars).to_string();
            // Check for error conditions
            if response_str.trim().is_empty() || response[response.len() - 1] == 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Variable not found",
                ));
            }
            Ok(response)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected",
            ))
        }
    }

    /// Reads the specified variable.
    ///
    /// # Arguments
    ///
    /// * `var_name` - Name of the variable to read.
    ///
    /// # Returns
    ///
    /// Returns the read variable value inside `std::io::Result<String>`.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// match osv.read("variable_name") {
    ///     Ok(val) => println!("Read value: {}", val),
    ///     Err(e) => println!("Reading error: {}", e),
    /// }
    /// ```
    pub fn read(&mut self, var_name: &str) -> std::io::Result<String> {
        // Return error if variable name to read is empty
        if var_name.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty variable name",
            ));
        }

        // Read variable value
        let response = self.send(var_name, "")?;

        // Check if the response is valid
        if response.len() < 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid response length",
            ));
        }

        // Get value length
        let val_len = ((response[5] as u16) << 8) | (response[6] as u16);

        // Return error if response length doesn't match value length
        if response.len() < (7 + val_len as usize) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Response length does not match value length",
            ));
        }

        // Return the variable value as a string
        let var_value = String::from_utf8_lossy(&response[7..(7 + val_len as usize)]).to_string();
        Ok(var_value)
    }

    /// Writes a value to the specified variable.
    ///
    /// # Arguments
    ///
    /// * `var_name` - Name of the variable to write.
    /// * `val` - Value to write to the variable.
    ///
    /// # Returns
    ///
    /// Returns the written variable value inside `std::io::Result<String>`.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// match osv.write("variable_name", "value") {
    ///     Ok(val) => println!("Written value: {}", val),
    ///     Err(e) => println!("Writing error: {}", e),
    /// }
    /// ```
    pub fn write(&mut self, var_name: &str, val: &str) -> std::io::Result<String> {
        // Return error if variable name to write is empty
        if var_name.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty variable name",
            ));
        }

        // Return error if value to write is empty
        if val.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty value",
            ));
        }

        // Write the variable value
        let response = self.send(var_name, val)?;

        // Check if the response is valid
        if response.len() < 7 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid response length",
            ));
        }

        // Get value length
        let val_len = ((response[5] as u16) << 8) | (response[6] as u16);

        // Return error if response length doesn't match value length
        if response.len() < (7 + val_len as usize) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Response length does not match value length",
            ));
        }

        // Return the variable value as a string
        let var_value = String::from_utf8_lossy(&response[7..(7 + val_len as usize)]).to_string();
        Ok(var_value)
    }

    /// Terminates the TCP connection.
    ///
    /// # Example
    ///
    /// ```
    /// use rs_openshowvar::OpenShowVar;
    /// let mut osv = OpenShowVar::new("127.0.0.1".to_string(), 7000);
    /// osv.disconnect();
    /// ```
    pub fn disconnect(&mut self) {
        // Close the connection if it exists
        if let Some(ref mut conn) = self.conn {
            let _ = conn.shutdown(std::net::Shutdown::Both);
            self.conn = None;
        }
    }
}
