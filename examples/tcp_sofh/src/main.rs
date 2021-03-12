use fefix::sofh::Decoder;

use std::io;
use std::net;
use std::str;

fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("127.0.0.1:8080")?;
    println!(
        "Listening on port {} for SOFH-enclosed messages.",
        listener.local_addr()?.port()
    );

    let reader = listener.accept()?.0;
    let decoder = Decoder::from_buffer(Vec::new());
    let mut frames = decoder.read_frames(reader);
    while let Ok(frame) = frames.next() {
        let frame = frame.unwrap();
        let message = str::from_utf8(frame.message()).unwrap();
        println!("Received message '{}'", message);
    }
    Ok(())
}
