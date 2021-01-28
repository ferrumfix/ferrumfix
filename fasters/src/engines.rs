use crate::app::slr;
use crate::dictionary::Dictionary;
use crate::encoders::Encoding;
use crate::session;
use crate::session::EventOutbound;
use std::marker::PhantomData;
use std::net::TcpListener;
use uuid::Uuid;

struct Acceptor<M, E: Encoding<M>> {
    id: Uuid,
    dictionary: Dictionary,
    encoder: E,
    phantom: PhantomData<M>,
}

impl<E: Encoding<slr::Message>> Acceptor<slr::Message, E> {
    pub fn new(dict: Dictionary, encoder: E) -> Self {
        Acceptor {
            id: Uuid::new_v4(),
            dictionary: dict,
            encoder,
            phantom: PhantomData,
        }
    }

    pub async fn listen(mut self, listener: TcpListener) {
        self.handle_connection(listener.incoming().map(|stream| stream.unwrap()))
            .await;
    }

    async fn handle_connection<T: std::io::Read + std::io::Write>(
        &mut self,
        listener: impl Iterator<Item = T>,
    ) {
        let mut payload = Vec::with_capacity(8192);
        let mut offset = 0;
        let config = session::Configuration::new();
        let mut session_layer = session::Acceptor::new(config);
        for mut socket in listener {
            let payload_size = socket.read(&mut payload).unwrap();
            offset += payload_size;
            let msg = self.encoder.decode(&mut &payload[offset..]).ok().unwrap();
            let event = session::EventInbound::IncomingMessage(msg);
            for pending_event in session_layer.notify(event) {
                match pending_event {
                    EventOutbound::Message(msg) => {
                        let payload: Vec<u8> = self.encoder.encode(msg).ok().unwrap();
                        socket.write(&payload).unwrap();
                    }
                    EventOutbound::Terminate => {
                        break;
                    }
                }
            }
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
