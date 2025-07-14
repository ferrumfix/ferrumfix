use futures_util::stream::StreamExt;
use std::io;
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let reader = listener.accept().await?.0;
    let mut decoder = rustysofh::TokioCodec::default().framed(reader);
    while let Some(frame) = decoder.next().await
        && let Ok(frame) = frame
    {
        let payload_clone = &frame.payload().to_vec()[..];
        let payload_utf8 = String::from_utf8_lossy(payload_clone);
        println!("Received message '{payload_utf8}'");
    }
    Ok(())
}

#[cfg(test)]
#[test]
fn run() {
    // FIXME: hangs.
    //    main().unwrap();
}
