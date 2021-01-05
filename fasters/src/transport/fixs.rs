use crate::openssl::ssl::*;
use std::net::{SocketAddr, TcpListener};
use std::path::Path;
use std::sync::Arc;

pub const RECOMMENDED_CIPHERSUITES: &[&str] = &[
    // The original FIXS specification uses IANA cipher names. You can convert
    // to OpenSSL names using a mapping table online.

    // TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256
    "ECDHE-ECDSA-AES128-GCM-SHA256",
    // TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
    "ECDHE-ECDSA-AES256-GCM-SHA384",
    // TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA
    "ECDHE-ECDSA-AES128-SHA256",
    // TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA
    "ECDHE-ECDSA-AES256-SHA384",
    // TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256
    "ECDHE-ECDSA-AES128-SHA256",
    // TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384
    "ECDHE-ECDSA-AES256-SHA384",
    // TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
    "ECDHE-RSA-AES128-GCM-SHA256",
    // TLS_DHE_RSA_WITH_AES_128_GCM_SHA256
    "DHE-RSA-AES128-GCM-SHA256",
    // TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384
    "ECDHE-RSA-AES256-GCM-SHA384",
    // TLS_DHE_RSA_WITH_AES_256_GCM_SHA384
    "DHE-RSA-AES256-GCM-SHA384",
    // TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA
    "ECDHE-RSA-AES128-SHA",
    // TLS_DHE_RSA_WITH_AES_128_CBC_SHA
    "DHE-RSA-AES128-SHA",
    // TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA
    "ECDHE-RSA-AES256-SHA",
    // TLS_DHE_RSA_WITH_AES_256_CBC_SHA
    "DHE-RSA-AES256-SHA",
    // TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256
    "ECDHE-RSA-AES128-SHA256",
    // TLS_DHE_RSA_WITH_AES_128_CBC_SHA256
    "DHE-RSA-AES128-SHA256",
    // TLS_DHE_RSA_WITH_AES_256_CBC_SHA256
    "DHE-RSA-AES256-SHA256",
    // TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384
    "ECDHE-RSA-AES256-SHA384",
];

pub const RECOMMENDED_CIPHERSUITES_PSK_ONLY: &[&str] = &[
    // TLS_DHE_PSK_WITH_AES_128_GCM_SHA256
    "DHE-PSK-AES128-GCM-SHA256",
    // TLS_DHE_PSK_WITH_AES_256_GCM_SHA384
    "DHE-PSK-AES256-GCM-SHA384",
    // TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA
    "ECDHE-PSK-AES128-CBC-SHA",
    // TLS_DHE_PSK_WITH_AES_128_CBC_SHA
    "DHE-PSK-AES128-CBC-SHA",
    // TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA
    "ECDHE-PSK-AES256-CBC-SHA",
    // TLS_DHE_PSK_WITH_AES_256_CBC_SHA
    "DHE-PSK-AES256-CBC-SHA",
    // TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256
    "ECDHE-PSK-AES128-CBC-SHA256",
    // TLS_DHE_PSK_WITH_AES_128_CBC_SHA256
    "DHE-PSK-AES128-CBC-SHA256",
    // TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384
    "ECDHE-PSK-AES256-CBC-SHA384",
    // TLS_DHE_PSK_WITH_AES_256_CBC_SHA384
    "DHE-PSK-AES256-CBC-SHA384",
];

pub enum Version {
    V1Draft,
}

impl Version {
    pub fn openssl_ciphers(&self) -> impl Iterator<Item = &str> {
        match self {
            Version::V1Draft => RECOMMENDED_CIPHERSUITES.iter().map(|s| *s),
        }
    }

    pub fn recommended_connector(&self) -> SslConnectorBuilder {
        self.private_recommended_connector()
    }

    pub fn recommended_acceptor(&self) -> SslAcceptorBuilder {
        self.private_recommended_acceptor()
    }

    fn private_recommended_connector(&self) -> SslConnectorBuilder {
        let mut context = SslConnector::builder(SslMethod::tls()).unwrap();
        match self {
            Version::V1Draft => {
                context
                    .set_min_proto_version(Some(SslVersion::TLS1_1))
                    .unwrap();
                context
                    .set_max_proto_version(Some(SslVersion::TLS1_2))
                    .unwrap();
                context.set_options(SslOptions::NO_COMPRESSION);
                context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
                context.set_options(SslOptions::NO_TLSV1_3);
            }
        };
        context.set_session_cache_mode(SslSessionCacheMode::SERVER);
        context
            .set_cipher_list(RECOMMENDED_CIPHERSUITES.join(":").as_str())
            .unwrap();
        context
    }

    fn private_recommended_acceptor(&self) -> SslAcceptorBuilder {
        let mut context = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
        match self {
            Version::V1Draft => {
                context
                    .set_min_proto_version(Some(SslVersion::TLS1_1))
                    .unwrap();
                context
                    .set_max_proto_version(Some(SslVersion::TLS1_2))
                    .unwrap();
                context.set_session_cache_mode(SslSessionCacheMode::SERVER);
                context.set_options(SslOptions::CIPHER_SERVER_PREFERENCE);
                context.set_options(SslOptions::NO_COMPRESSION);
                context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
                context.set_options(SslOptions::NO_TLSV1_3);
            }
        };
        context
            .set_cipher_list(RECOMMENDED_CIPHERSUITES.join(":").as_str())
            .unwrap();
        context
    }

    pub fn fixua_acceptor(&self, config: FixuaConfig) -> (TcpListener, Arc<SslAcceptor>) {
        let tcp_listener = TcpListener::bind(config.address).unwrap();
        let mut acceptor = Version::V1Draft.recommended_acceptor();
        acceptor
            .set_private_key_file(config.key, SslFiletype::PEM)
            .unwrap();
        acceptor
            .set_certificate_file(config.cert, SslFiletype::PEM)
            .unwrap();
        let acceptor = Arc::new(acceptor.build());
        (tcp_listener, acceptor)
    }
}

pub struct FixuaConfig<'a> {
    pub cert: &'a Path,
    pub key: &'a Path,
    pub address: SocketAddr,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn v1draft_acceptor_is_ok() {
        Version::V1Draft.recommended_acceptor();
    }

    #[test]
    fn v1draft_connector_is_ok() {
        Version::V1Draft.recommended_connector();
    }
}
