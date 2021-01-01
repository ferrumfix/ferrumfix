use crate::transport::fixs;
use crate::openssl::ssl::{SslAcceptor, SslFiletype};
use std::net::{TcpListener, SocketAddr};
use std::path::Path;
use std::sync::Arc;

pub fn fixua_config(cert: &Path, key: &Path, port: u16) -> (TcpListener, Arc<SslAcceptor>) {
    let tcp_listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    let mut acceptor = fixs::Version::V1Draft.recommended_acceptor();
    acceptor
        .set_private_key_file(key, SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_file(cert, SslFiletype::PEM)
        .unwrap();
    let acceptor = Arc::new(acceptor.build());
    (tcp_listener, acceptor)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transport::fixs;
    use std::io::Write;
    use std::path::Path;
    use std::net::TcpStream;

    #[test]
    fn fixua_hello() {
        std::thread::spawn(|| {
            let mut connector = fixs::Version::V1Draft.recommended_connector()
                .build()
                .configure()
                .unwrap()
                .verify_hostname(false);
            connector.set_verify(openssl::ssl::SslVerifyMode::NONE);
            connector.set_use_server_name_indication(false);

            let stream = TcpStream::connect("127.0.0.1:8443").unwrap();
            let mut stream = connector
                .connect("example.com", stream)
                .unwrap();

            stream.write_all(b"Hello, world!").unwrap();
        });
        fixua_config(
            Path::new("/home/neysofu/Projects/fasters/fasters/example.com.cert.pem"),
            Path::new("/home/neysofu/Projects/fasters/fasters/example.com.key.pem"),
            8443,
        );
    }
}
