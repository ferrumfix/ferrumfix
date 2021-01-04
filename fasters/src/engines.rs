use crate::transport::fixs;
use crate::openssl::ssl::{SslAcceptor, SslFiletype};
use std::net::{TcpListener, SocketAddr};
use std::path::Path;
use std::sync::Arc;

pub fn fixua_config(cert: &Path, key: &Path, address: SocketAddr) -> (TcpListener, Arc<SslAcceptor>) {
    let tcp_listener = TcpListener::bind(address).unwrap();
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
    use std::path::PathBuf;
    use std::net::TcpStream;

    fn exampledotcom_cert_key_pair() -> (PathBuf, PathBuf) {
        let mut src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        src_dir.push("src");
        let mut path_to_cert = src_dir.clone();
        let mut path_to_key = src_dir;
        path_to_cert.push("example.com.cert.pem");
        path_to_key.push("example.com.key.pem");
        (path_to_cert, path_to_key)
    }

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
        let (path_to_cert, path_to_key) = exampledotcom_cert_key_pair();
        fixua_config(
            path_to_cert.as_path(),
            path_to_key.as_path(),
            "0.0.0.0:8443".parse().unwrap(),
        );
    }
}
