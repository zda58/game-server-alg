use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use local_ip_address::local_ip;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let local_ip = local_ip().unwrap();
    println!("Listening on address: {}:{}", local_ip, addr.port());
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&stream);
    let mut writer = stream.try_clone()?;

    loop {
        let mut buffer = String::new();
        reader.read_line(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        println!("Received: {}", buffer.trim());

        writer.write_all(buffer.as_bytes())?;
        writer.flush()?;
    }

    Ok(())
}