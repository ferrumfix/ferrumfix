use fasters::app::Version;
use fasters::encoders::json::{self, TransPrettyPrint};
use fasters::encoders::{Encoder, Decoder};
use fasters::Dictionary;
use std::io;
use std::io::Read;
use std::net;

fn print_listening(addr: net::SocketAddr) {
    println!("Listening on port {} for JSON messages.", addr.port());
}

fn main() -> io::Result<()> {
    let fix_v44 = Dictionary::from_version(Version::Fix44);
    let mut codec = (json::Json::new(fix_v44), TransPrettyPrint);
    let listener = net::TcpListener::bind("0.0.0.0:0")?;
    print_listening(listener.local_addr()?);
    let mut payload = vec![0u8; 8192];
    let mut offset = 0;
    for stream in listener.incoming() {
        let payload_size = stream?.read(&mut payload)?;
        offset += payload_size;
        println!("{}", std::str::from_utf8(&payload[..offset]).unwrap());
        let msg = codec.decode(&mut &payload[..offset]).unwrap();
        let mut buffer = Vec::<u8>::new();
        codec.encode(&mut buffer, &msg).unwrap();
        println!(
            "{}",
            std::str::from_utf8(buffer.as_slice()).unwrap()
        );
    }
    Ok(())
}
