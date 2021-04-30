use fefix::session::FixConnectionBuilder;
use fefix::AppVersion;
use fefix::{
    tagvalue::{Config, Decoder, Fv},
    Dictionary,
};
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpSocket;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const PORT: u16 = 0xF13;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp_socket = TcpSocket::new_v4()?;
    let socket_address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, PORT);
    let tcp_stream = tcp_socket.connect(socket_address.into()).await?;
    tcp_stream.set_nodelay(true)?;
    let fix_initiator = {
        let mut builder = FixConnectionBuilder::default();
        builder.set_begin_string("FIX-4.2");
        builder.set_target_comp_id("TW");
        builder.set_sender_comp_id("INCA");
        let fix_dictionary = Dictionary::from_version(AppVersion::Fix42);
        let fix_decoder = Decoder::new(fix_dictionary);
        let (reader, writer) = tokio::io::split(tcp_stream);
        builder
            .build()
            .initiate(reader.compat(), writer.compat_write(), fix_decoder)
            .await;
    };
    Ok(())
}
