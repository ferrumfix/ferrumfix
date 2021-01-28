use fasters::prelude::*;
use std::io;
use std::io::Read;
use std::net;

fn print_listening(addr: net::SocketAddr) {
    println!("Listening on port {} for JSON messages.", addr.port());
}

fn main() -> io::Result<()> {
    let fix_v44 = Dictionary::from_version(fasters::app::Version::Fix44);
    let mut encoder = encoders::Json::new(fix_v44, true);
    let listener = net::TcpListener::bind("0.0.0.0:0")?;
    print_listening(listener.local_addr()?);
    let mut payload = vec![0u8; 8192];
    let mut offset = 0;
    for stream in listener.incoming() {
        let payload_size = stream?.read(&mut payload)?;
        offset += payload_size;
        println!("{}", std::str::from_utf8(&payload[..offset]).unwrap());
        let msg = encoder.decode(&mut &payload[..offset]).unwrap();
        println!(
            "{}",
            std::str::from_utf8(&mut encoder.encode(msg).unwrap()[..]).unwrap()
        );
    }
    Ok(())
}
