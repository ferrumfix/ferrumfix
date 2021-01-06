use fasters::prelude::*;
use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:15000").expect("IO error!");
    let mut payload = vec![0u8; 8192];
    let encoder = encoders::Json::new(Dictionary::from_version(fasters::app::Version::Fix44), true);
    let mut offset = 0;
    for stream in listener.incoming() {
        let payload_size = stream.unwrap().read(&mut payload).unwrap();
        offset += payload_size;
        println!("{}", std::str::from_utf8(&payload[..offset]).unwrap());
        let msg = encoder.decode(&mut &payload[..offset]).unwrap();
        println!(
            "{}",
            std::str::from_utf8(&encoder.encode(msg).unwrap()[..]).unwrap()
        );
    }
}
