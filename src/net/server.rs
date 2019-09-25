use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let byte_read = stream.read(&mut buf)?;
        if byte_read == 0 {return Ok(());}
        stream.write(&buf[..byte_read])?;
    }
}

pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("Couldn't bind on port 8888");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {eprintln!("failed: {}", e);}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }

}