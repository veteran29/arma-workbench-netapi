use std::io::{self, prelude::*};
use std::net::TcpStream;

#[macro_use]
extern crate structure;

fn main() {
    match workbench_func("IsWorkbenchRunning") {
        Ok(r) => println!("IsWorkbenchRunning: {}", r),
        Err(e) => println!("IsWorkbenchRunning: Error: {}", e),
    }

    println!("");
    match workbench_func("FileSystemApi") {
        Ok(r) => println!("FileSystemApi: {}", r),
        Err(e) => println!("FileSystemApi: Error: {}", e),
    }

    println!("\nDone");
}

trait WorkbenchStream {
    fn w_read_int(&mut self) -> io::Result<i32>;
    fn w_write_int(&mut self, int: u32) -> io::Result<()>;
    fn w_write_string(&mut self, string: String) -> io::Result<()>;

    fn w_read_string(&mut self) -> io::Result<String>;
}

impl WorkbenchStream for TcpStream {
    fn w_write_int(&mut self, int: u32) -> io::Result<()> {
        structure!("<I").pack_into(self, int)
    }

    fn w_write_string(&mut self, string: String) -> io::Result<()> {
        self.w_write_int(string.len().try_into().unwrap())?;
        self.write_all(string.as_bytes())
    }

    fn w_read_int(&mut self) -> io::Result<i32> {
        let mut output: [u8; 4] = [0; 4];
        self.read_exact(&mut output)?;

        let (int,): (i32,) = structure!("<i").unpack(output)?;
        Ok(int)
    }

    fn w_read_string(&mut self) -> io::Result<String> {
        let mut output: String = "".to_string();
        let mut length: usize = self.w_read_int()?.try_into().unwrap();

        while length > 0 {
            let mut chunk: Vec<u8> = vec![0; length];
            self.read_exact(&mut chunk)?;
            let string = std::str::from_utf8(&chunk).expect("Valid UTF-8");

            if string.len() == 0 {
                break;
            }

            length -= chunk.len();
            output += string;
        }
        Ok(output)
    }
}

fn workbench_func(function: &str) -> Result<String, io::Error> {
    // Socket
    const ADDRESS: &str = "127.0.0.1";
    const PORT: &str = "5775";

    // Workbench specific
    const PROTOCOL_VERSION: u32 = 1;
    // const USER_AGENT: &str = "NetApi Client";
    const USER_AGENT: &str = "Blender 3.0";
    const CONTENT_TYPE: &str = "JsonRPC";

    let mut stream = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))?;

    let request = format!("{{\"APIFunc\": \"{}\"}}", function);

    println!("Sending metadata");
    stream.w_write_int(PROTOCOL_VERSION)?;
    stream.w_write_string(USER_AGENT.to_string())?;
    stream.w_write_string(CONTENT_TYPE.to_string())?;

    println!("Sending request");
    stream.w_write_string(request)?;

    println!("Reading response");

    let error = stream.w_read_string()?;
    if error != "Ok" {
        return Err(io::Error::new(io::ErrorKind::Other, error))
    }
    
    let response = stream.w_read_string()?;
    println!("Response: {}", response);
    println!();

    Ok(response)
}
