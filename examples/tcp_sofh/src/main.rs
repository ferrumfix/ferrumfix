use fefix::sofh::CodecBuffered;

use std::io;
use std::net;

fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("0.0.0.0:0")?;
    print_listening(listener.local_addr()?);
    let reader = listener.accept()?.0;
    let parser = CodecBuffered::new();
    let mut frames = parser.read_frames(reader);
    while let Ok(frame) = frames.next() {
        let frame = frame.as_ref().unwrap();
        let payload = std::str::from_utf8(frame.payload()).unwrap();
        println!("{}", payload);
    }
    Ok(())
}

fn print_listening(addr: net::SocketAddr) {
    println!(
        "Listening on port {} for SOFH-enclosed messages.",
        addr.port()
    );
}
