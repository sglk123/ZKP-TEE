use std::os::unix::net::UnixListener;
use std::io::{Read, Write};

fn main() {
    // start vsock
    let listener = UnixListener::bind("/dev/vsock/3.5005").unwrap();
    println!("Enclave listening on vsock port 5005");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

                let response = b"Hello, from the Enclave!";
                stream.write_all(response).unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
