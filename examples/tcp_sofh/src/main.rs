use fasters::codec::sofh::Codec;

use std::io;
use std::net;

fn print_listening(addr: net::SocketAddr) {
    println!(
        "Listening on port {} for SOFH-enclosed messages.",
        addr.port()
    );
}

fn main() -> io::Result<()> {
    let listener = net::TcpListener::bind("0.0.0.0:0")?;
    print_listening(listener.local_addr()?);
    let _stream = listener.accept()?.0;
    let _parser = Codec::new();
    //let mut frames = parser.read_frames(stream);
    //while let Some(frame_result) = frames.next() {
    //    let frame = frame_result.as_ref().unwrap();
    //    let payload = std::str::from_utf8(frame.payload()).unwrap();
    //    println!("{}", payload);
    //}
    Ok(())
}
