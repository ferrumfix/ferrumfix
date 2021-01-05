use crate::dictionary::Dictionary;
use crate::openssl::ssl::{SslAcceptor, SslFiletype};
use crate::presentation::Encoding;
use crate::session;
use crate::transport::fixs;
use std::net::{SocketAddr, TcpListener};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

struct Acceptor<E: Encoding> {
    id: Uuid,
    dictionary: Dictionary,
    encoder: E,
}

impl<E: Encoding> Acceptor<E> {
    pub fn new(dict: Dictionary, encoder: E) -> Self {
        Acceptor {
            id: Uuid::new_v4(),
            dictionary: dict,
            encoder,
        }
    }

    pub async fn listen(self, listener: TcpListener) {
        self.handle_connection(listener.incoming().map(|stream| stream.unwrap())).await;
    }

    async fn handle_connection<T: std::io::Read>(&self, channels: impl Iterator<Item = T>) {
        let mut payload = Vec::with_capacity(8192);
        let mut offset = 0;
        let mut session_layer = session::ProcessorBuilder::new("FOO", "BAR").build();
        for mut channel in channels {
            let payload_size = channel.read(&mut payload).unwrap();
            offset += payload_size;
            let msg = self.encoder.decode(&mut &payload[offset..]).unwrap();
            session_layer.process_incoming(msg).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::transport::fixs;
    use std::io::Write;
    use std::net::TcpStream;
    use std::path::PathBuf;

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
            let mut connector = fixs::Version::V1Draft
                .recommended_connector()
                .build()
                .configure()
                .unwrap()
                .verify_hostname(false);
            connector.set_verify(openssl::ssl::SslVerifyMode::NONE);
            connector.set_use_server_name_indication(false);

            let stream = TcpStream::connect("127.0.0.1:8443").unwrap();
            let mut stream = connector.connect("example.com", stream).unwrap();

            stream.write_all(b"Hello, world!").unwrap();
        });
        let (path_to_cert, path_to_key) = exampledotcom_cert_key_pair();
        fixs::Version::V1Draft.fixua_acceptor(fixs::FixuaConfig {
            cert: path_to_cert.as_path(),
            key: path_to_key.as_path(),
            address: "0.0.0.0:8443".parse().unwrap(),
        });
    }
}
