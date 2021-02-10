use crate::openssl::ssl::*;
use std::net::{SocketAddr, TcpListener};
use std::path::Path;
use std::sync::Arc;

const V1_DRAFT_RECOMMENDED_CIPHERSUITES: &[&str] = &[
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
    "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_DHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_DHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_DHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_DHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256",
    "TLS_DHE_RSA_WITH_AES_128_CBC_SHA256",
    "TLS_DHE_RSA_WITH_AES_256_CBC_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384",
];

const V1_DRAFT_RECOMMENDED_CIPHERSUITES_PSK_ONLY: &[&str] = &[
    "TLS_DHE_PSK_WITH_AES_128_GCM_SHA256",
    "TLS_DHE_PSK_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA",
    "TLS_DHE_PSK_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA",
    "TLS_DHE_PSK_WITH_AES_256_CBC_SHA",
    "TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256",
    "TLS_DHE_PSK_WITH_AES_128_CBC_SHA256",
    "TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384",
    "TLS_DHE_PSK_WITH_AES_256_CBC_SHA384",
];

/// A specific version of the FIXS protocol specification.
#[derive(Debug, Copy, Clone)]
pub enum Version {
    V1Draft,
}

impl Version {
    /// Returns an [`Iterator`] over the suggested ciphersuites for TLS,
    /// according to this version. The ciphersuites are specified in IANA format.
    /// 
    /// ```
    /// use fefix::transport::fixs::Version;
    /// 
    /// let version = Version::V1Draft;
    /// let mut ciphersuites_iana = version.recommended_cs_iana(false);
    /// assert!(ciphersuites_iana.any(|cs| cs == "TLS_DHE_RSA_WITH_AES_128_GCM_SHA256"));
    /// ```
    pub fn recommended_cs_iana(&self, psk: bool) -> impl Iterator<Item = &str> {
        match (self, psk) {
            (Version::V1Draft, false) => V1_DRAFT_RECOMMENDED_CIPHERSUITES.iter().chain(&[]),
            (Version::V1Draft, true) => V1_DRAFT_RECOMMENDED_CIPHERSUITES
                .iter()
                .chain(V1_DRAFT_RECOMMENDED_CIPHERSUITES_PSK_ONLY),
        }
        .copied()
    }

    /// Returns an [`Iterator`] over the suggested ciphersuites for TLS,
    /// according to this version. The ciphersuites are specified in OpenSSL's
    /// format.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use fefix::transport::fixs::Version;
    /// 
    /// let version = Version::V1Draft;
    /// let mut ciphersuites_openssl = version.recommended_cs_openssl(false);
    /// assert!(ciphersuites_openssl.any(|cs| cs == "DHE-RSA-AES128-GCM-SHA256"));
    /// ```
    /// 
    /// List all ciphersuites in a colon-separated format, like required by
    /// [`openssl-ciphers`](https://www.openssl.org/docs/manmaster/man1/openssl-ciphers.html).
    /// 
    /// ```
    /// use fefix::transport::fixs::Version;
    /// 
    /// let version = Version::V1Draft;
    /// let mut ciphersuites_openssl = version.recommended_cs_openssl(false);
    /// let cipherlist = ciphersuites_openssl.collect::<Vec<&str>>().join(":");
    /// println!("Supported ciphers: {}", cipherlist);
    /// ```
    pub fn recommended_cs_openssl(&self, psk: bool) -> impl Iterator<Item = &str> {
        self.recommended_cs_iana(psk)
            .map(|s| *ciphersuites::IANA_TO_OPENSSL.get(s).unwrap())
    }

    pub fn recommended_connector(&self) -> SslConnectorBuilder {
        self.private_recommended_connector()
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
            .set_cipher_list(
                self.recommended_cs_openssl(false)
                    .collect::<Vec<&str>>()
                    .join(":")
                    .as_str(),
            )
            .unwrap();
        context
    }

    pub fn recommended_acceptor(&self) -> SslAcceptorBuilder {
        self.private_recommended_acceptor()
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
            .set_cipher_list(
                self.recommended_cs_openssl(false)
                    .collect::<Vec<&str>>()
                    .join(":")
                    .as_str(),
            )
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

#[derive(Debug, Clone)]
pub struct FixuaConfig<'a> {
    pub cert: &'a Path,
    pub key: &'a Path,
    pub address: SocketAddr,
}

mod ciphersuites {
    use phf::phf_map;

    // See https://gist.github.com/Chion82/dafc1b209eb94b90f4bf090c6ae694e5 and
    // https://testssl.sh/openssl-iana.mapping.html.
    pub static IANA_TO_OPENSSL: phf::Map<&'static str, &'static str> = phf_map! {
        "TLS_RSA_WITH_NULL_MD5" => "NULL-MD5",
        "TLS_RSA_WITH_NULL_SHA" => "NULL-SHA",
        "TLS_RSA_EXPORT_WITH_RC4_40_MD5" => "EXP-RC4-MD5",
        "TLS_RSA_WITH_RC4_128_MD5" => "RC4-MD5",
        "TLS_RSA_WITH_RC4_128_SHA" => "RC4-SHA",
        "TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5" => "EXP-RC2-CBC-MD5",
        "TLS_RSA_WITH_IDEA_CBC_SHA" => "IDEA-CBC-SHA",
        "TLS_RSA_EXPORT_WITH_DES40_CBC_SHA" => "EXP-DES-CBC-SHA",
        "TLS_RSA_WITH_DES_CBC_SHA" => "DES-CBC-SHA",
        "TLS_RSA_WITH_3DES_EDE_CBC_SHA" => "DES-CBC3-SHA",
        "TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA" => "EXP-DH-DSS-DES-CBC-SHA",
        "TLS_DH_DSS_WITH_DES_CBC_SHA" => "DH-DSS-DES-CBC-SHA",
        "TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA" => "DH-DSS-DES-CBC3-SHA",
        "TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA" => "EXP-DH-RSA-DES-CBC-SHA",
        "TLS_DH_RSA_WITH_DES_CBC_SHA" => "DH-RSA-DES-CBC-SHA",
        "TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA" => "DH-RSA-DES-CBC3-SHA",
        "TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA" => "EXP-EDH-DSS-DES-CBC-SHA",
        "TLS_DHE_DSS_WITH_DES_CBC_SHA" => "EDH-DSS-DES-CBC-SHA",
        "TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA" => "EDH-DSS-DES-CBC3-SHA",
        "TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA" => "EXP-EDH-RSA-DES-CBC-SHA",
        "TLS_DHE_RSA_WITH_DES_CBC_SHA" => "EDH-RSA-DES-CBC-SHA",
        "TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA" => "EDH-RSA-DES-CBC3-SHA",
        "TLS_DH_anon_EXPORT_WITH_RC4_40_MD5" => "EXP-ADH-RC4-MD5",
        "TLS_DH_anon_WITH_RC4_128_MD5" => "ADH-RC4-MD5",
        "TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA" => "EXP-ADH-DES-CBC-SHA",
        "TLS_DH_anon_WITH_DES_CBC_SHA" => "ADH-DES-CBC-SHA",
        "TLS_DH_anon_WITH_3DES_EDE_CBC_SHA" => "ADH-DES-CBC3-SHA",
        "TLS_KRB5_WITH_DES_CBC_SHA" => "KRB5-DES-CBC-SHA",
        "TLS_KRB5_WITH_3DES_EDE_CBC_SHA" => "KRB5-DES-CBC3-SHA",
        "TLS_KRB5_WITH_RC4_128_SHA" => "KRB5-RC4-SHA",
        "TLS_KRB5_WITH_IDEA_CBC_SHA" => "KRB5-IDEA-CBC-SHA",
        "TLS_KRB5_WITH_DES_CBC_MD5" => "KRB5-DES-CBC-MD5",
        "TLS_KRB5_WITH_3DES_EDE_CBC_MD5" => "KRB5-DES-CBC3-MD5",
        "TLS_KRB5_WITH_RC4_128_MD5" => "KRB5-RC4-MD5",
        "TLS_KRB5_WITH_IDEA_CBC_MD5" => "KRB5-IDEA-CBC-MD5",
        "TLS_KRB5_EXPORT_WITH_DES_CBC_40_SHA" => "EXP-KRB5-DES-CBC-SHA",
        "TLS_KRB5_EXPORT_WITH_RC2_CBC_40_SHA" => "EXP-KRB5-RC2-CBC-SHA",
        "TLS_KRB5_EXPORT_WITH_RC4_40_SHA" => "EXP-KRB5-RC4-SHA",
        "TLS_KRB5_EXPORT_WITH_DES_CBC_40_MD5" => "EXP-KRB5-DES-CBC-MD5",
        "TLS_KRB5_EXPORT_WITH_RC2_CBC_40_MD5" => "EXP-KRB5-RC2-CBC-MD5",
        "TLS_KRB5_EXPORT_WITH_RC4_40_MD5" => "EXP-KRB5-RC4-MD5",
        "TLS_PSK_WITH_NULL_SHA" => "PSK-NULL-SHA",
        "TLS_DHE_PSK_WITH_NULL_SHA" => "DHE-PSK-NULL-SHA",
        "TLS_RSA_PSK_WITH_NULL_SHA" => "RSA-PSK-NULL-SHA",
        "TLS_RSA_WITH_AES_128_CBC_SHA" => "AES128-SHA",
        "TLS_DH_DSS_WITH_AES_128_CBC_SHA" => "DH-DSS-AES128-SHA",
        "TLS_DH_RSA_WITH_AES_128_CBC_SHA" => "DH-RSA-AES128-SHA",
        "TLS_DHE_DSS_WITH_AES_128_CBC_SHA" => "DHE-DSS-AES128-SHA",
        "TLS_DHE_RSA_WITH_AES_128_CBC_SHA" => "DHE-RSA-AES128-SHA",
        "TLS_DH_anon_WITH_AES_128_CBC_SHA" => "ADH-AES128-SHA",
        "TLS_RSA_WITH_AES_256_CBC_SHA" => "AES256-SHA",
        "TLS_DH_DSS_WITH_AES_256_CBC_SHA" => "DH-DSS-AES256-SHA",
        "TLS_DH_RSA_WITH_AES_256_CBC_SHA" => "DH-RSA-AES256-SHA",
        "TLS_DHE_DSS_WITH_AES_256_CBC_SHA" => "DHE-DSS-AES256-SHA",
        "TLS_DHE_RSA_WITH_AES_256_CBC_SHA" => "DHE-RSA-AES256-SHA",
        "TLS_DH_anon_WITH_AES_256_CBC_SHA" => "ADH-AES256-SHA",
        "TLS_RSA_WITH_NULL_SHA256" => "NULL-SHA256",
        "TLS_RSA_WITH_AES_128_CBC_SHA256" => "AES128-SHA256",
        "TLS_RSA_WITH_AES_256_CBC_SHA256" => "AES256-SHA256",
        "TLS_DH_DSS_WITH_AES_128_CBC_SHA256" => "DH-DSS-AES128-SHA256",
        "TLS_DH_RSA_WITH_AES_128_CBC_SHA256" => "DH-RSA-AES128-SHA256",
        "TLS_DHE_DSS_WITH_AES_128_CBC_SHA256" => "DHE-DSS-AES128-SHA256",
        "TLS_RSA_WITH_CAMELLIA_128_CBC_SHA" => "CAMELLIA128-SHA",
        "TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA" => "DH-DSS-CAMELLIA128-SHA",
        "TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA" => "DH-RSA-CAMELLIA128-SHA",
        "TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA" => "DHE-DSS-CAMELLIA128-SHA",
        "TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA" => "DHE-RSA-CAMELLIA128-SHA",
        "TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA" => "ADH-CAMELLIA128-SHA",
        "TLS_RSA_EXPORT1024_WITH_RC4_56_MD5" => "EXP1024-RC4-MD5",
        "TLS_RSA_EXPORT1024_WITH_RC2_CBC_56_MD5" => "EXP1024-RC2-CBC-MD5",
        "TLS_RSA_EXPORT1024_WITH_DES_CBC_SHA" => "EXP1024-DES-CBC-SHA",
        "TLS_DHE_DSS_EXPORT1024_WITH_DES_CBC_SHA" => "EXP1024-DHE-DSS-DES-CBC-SHA",
        "TLS_RSA_EXPORT1024_WITH_RC4_56_SHA" => "EXP1024-RC4-SHA",
        "TLS_DHE_DSS_EXPORT1024_WITH_RC4_56_SHA" => "EXP1024-DHE-DSS-RC4-SHA",
        "TLS_DHE_DSS_WITH_RC4_128_SHA" => "DHE-DSS-RC4-SHA",
        "TLS_DHE_RSA_WITH_AES_128_CBC_SHA256" => "DHE-RSA-AES128-SHA256",
        "TLS_DH_DSS_WITH_AES_256_CBC_SHA256" => "DH-DSS-AES256-SHA256",
        "TLS_DH_RSA_WITH_AES_256_CBC_SHA256" => "DH-RSA-AES256-SHA256",
        "TLS_DHE_DSS_WITH_AES_256_CBC_SHA256" => "DHE-DSS-AES256-SHA256",
        "TLS_DHE_RSA_WITH_AES_256_CBC_SHA256" => "DHE-RSA-AES256-SHA256",
        "TLS_DH_anon_WITH_AES_128_CBC_SHA256" => "ADH-AES128-SHA256",
        "TLS_DH_anon_WITH_AES_256_CBC_SHA256" => "ADH-AES256-SHA256",
        "TLS_GOSTR341094_WITH_28147_CNT_IMIT" => "GOST94-GOST89-GOST89",
        "TLS_GOSTR341001_WITH_28147_CNT_IMIT" => "GOST2001-GOST89-GOST89",
        "TLS_GOSTR341001_WITH_NULL_GOSTR3411" => "GOST94-NULL-GOST94",
        "TLS_GOSTR341094_WITH_NULL_GOSTR3411" => "GOST2001-GOST89-GOST89",
        "TLS_RSA_WITH_CAMELLIA_256_CBC_SHA" => "CAMELLIA256-SHA",
        "TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256" => "CAMELLIA256-SHA256",
        "TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA" => "DH-DSS-CAMELLIA256-SHA",
        "TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA" => "DH-RSA-CAMELLIA256-SHA",
        "TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA" => "DHE-DSS-CAMELLIA256-SHA",
        "TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA" => "DHE-RSA-CAMELLIA256-SHA",
        "TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA" => "ADH-CAMELLIA256-SHA",
        "TLS_PSK_WITH_RC4_128_SHA" => "PSK-RC4-SHA",
        "TLS_PSK_WITH_3DES_EDE_CBC_SHA" => "PSK-3DES-EDE-CBC-SHA",
        "TLS_PSK_WITH_AES_128_CBC_SHA" => "PSK-AES128-CBC-SHA",
        "TLS_PSK_WITH_AES_256_CBC_SHA" => "PSK-AES256-CBC-SHA",
        "TLS_RSA_WITH_SEED_CBC_SHA" => "SEED-SHA",
        "TLS_DH_DSS_WITH_SEED_CBC_SHA" => "DH-DSS-SEED-SHA",
        "TLS_DH_RSA_WITH_SEED_CBC_SHA" => "DH-RSA-SEED-SHA",
        "TLS_DHE_DSS_WITH_SEED_CBC_SHA" => "DHE-DSS-SEED-SHA",
        "TLS_DHE_RSA_WITH_SEED_CBC_SHA" => "DHE-RSA-SEED-SHA",
        "TLS_DH_anon_WITH_SEED_CBC_SHA" => "ADH-SEED-SHA",
        "TLS_RSA_WITH_AES_128_GCM_SHA256" => "AES128-GCM-SHA256",
        "TLS_RSA_WITH_AES_256_GCM_SHA384" => "AES256-GCM-SHA384",
        "TLS_DHE_RSA_WITH_AES_128_GCM_SHA256" => "DHE-RSA-AES128-GCM-SHA256",
        "TLS_DHE_RSA_WITH_AES_256_GCM_SHA384" => "DHE-RSA-AES256-GCM-SHA384",
        "TLS_DH_RSA_WITH_AES_128_GCM_SHA256" => "DH-RSA-AES128-GCM-SHA256",
        "TLS_DH_RSA_WITH_AES_256_GCM_SHA384" => "DH-RSA-AES256-GCM-SHA384",
        "TLS_DHE_DSS_WITH_AES_128_GCM_SHA256" => "DHE-DSS-AES128-GCM-SHA256",
        "TLS_DHE_DSS_WITH_AES_256_GCM_SHA384" => "DHE-DSS-AES256-GCM-SHA384",
        "TLS_DH_DSS_WITH_AES_128_GCM_SHA256" => "DH-DSS-AES128-GCM-SHA256",
        "TLS_DH_DSS_WITH_AES_256_GCM_SHA384" => "DH-DSS-AES256-GCM-SHA384",
        "TLS_DH_anon_WITH_AES_128_GCM_SHA256" => "ADH-AES128-GCM-SHA256",
        "TLS_DH_anon_WITH_AES_256_GCM_SHA384" => "ADH-AES256-GCM-SHA384",
        "TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256" => "CAMELLIA128-SHA256",
        "TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256" => "DH-DSS-CAMELLIA128-SHA256",
        "TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256" => "DH-RSA-CAMELLIA128-SHA256",
        "TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256" => "DHE-DSS-CAMELLIA128-SHA256",
        "TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256" => "DHE-RSA-CAMELLIA128-SHA256",
        "TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256" => "ADH-CAMELLIA128-SHA256",
        "TLS_EMPTY_RENEGOTIATION_INFO_SCSV" => "TLS_FALLBACK_SCSV",
        "TLS_AES_128_GCM_SHA256" => "TLS_AES_128_GCM_SHA256",
        "TLS_AES_256_GCM_SHA384" => "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256" => "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_AES_128_CCM_SHA256" => "TLS_AES_128_CCM_SHA256",
        "TLS_AES_128_CCM_8_SHA256" => "TLS_AES_128_CCM_8_SHA256",
        "TLS_ECDH_ECDSA_WITH_NULL_SHA" => "ECDH-ECDSA-NULL-SHA",
        "TLS_ECDH_ECDSA_WITH_RC4_128_SHA" => "ECDH-ECDSA-RC4-SHA",
        "TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA" => "ECDH-ECDSA-DES-CBC3-SHA",
        "TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA" => "ECDH-ECDSA-AES128-SHA",
        "TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA" => "ECDH-ECDSA-AES256-SHA",
        "TLS_ECDHE_ECDSA_WITH_NULL_SHA" => "ECDHE-ECDSA-NULL-SHA",
        "TLS_ECDHE_ECDSA_WITH_RC4_128_SHA" => "ECDHE-ECDSA-RC4-SHA",
        "TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA" => "ECDHE-ECDSA-DES-CBC3-SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA" => "ECDHE-ECDSA-AES128-SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA" => "ECDHE-ECDSA-AES256-SHA",
        "TLS_ECDH_RSA_WITH_NULL_SHA" => "ECDH-RSA-NULL-SHA",
        "TLS_ECDH_RSA_WITH_RC4_128_SHA" => "ECDH-RSA-RC4-SHA",
        "TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA" => "ECDH-RSA-DES-CBC3-SHA",
        "TLS_ECDH_RSA_WITH_AES_128_CBC_SHA" => "ECDH-RSA-AES128-SHA",
        "TLS_ECDH_RSA_WITH_AES_256_CBC_SHA" => "ECDH-RSA-AES256-SHA",
        "TLS_ECDHE_RSA_WITH_NULL_SHA" => "ECDHE-RSA-NULL-SHA",
        "TLS_ECDHE_RSA_WITH_RC4_128_SHA" => "ECDHE-RSA-RC4-SHA",
        "TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA" => "ECDHE-RSA-DES-CBC3-SHA",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA" => "ECDHE-RSA-AES128-SHA",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA" => "ECDHE-RSA-AES256-SHA",
        "TLS_ECDH_anon_WITH_NULL_SHA" => "AECDH-NULL-SHA",
        "TLS_ECDH_anon_WITH_RC4_128_SHA" => "AECDH-RC4-SHA",
        "TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA" => "AECDH-DES-CBC3-SHA",
        "TLS_ECDH_anon_WITH_AES_128_CBC_SHA" => "AECDH-AES128-SHA",
        "TLS_ECDH_anon_WITH_AES_256_CBC_SHA" => "AECDH-AES256-SHA",
        "TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA" => "SRP-3DES-EDE-CBC-SHA",
        "TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA" => "SRP-RSA-3DES-EDE-CBC-SHA",
        "TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA" => "SRP-DSS-3DES-EDE-CBC-SHA",
        "TLS_SRP_SHA_WITH_AES_128_CBC_SHA" => "SRP-AES-128-CBC-SHA",
        "TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA" => "SRP-RSA-AES-128-CBC-SHA",
        "TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA" => "SRP-DSS-AES-128-CBC-SHA",
        "TLS_SRP_SHA_WITH_AES_256_CBC_SHA" => "SRP-AES-256-CBC-SHA",
        "TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA" => "SRP-RSA-AES-256-CBC-SHA",
        "TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA" => "SRP-DSS-AES-256-CBC-SHA",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256" => "ECDHE-ECDSA-AES128-SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384" => "ECDHE-ECDSA-AES256-SHA384",
        "TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256" => "ECDH-ECDSA-AES128-SHA256",
        "TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384" => "ECDH-ECDSA-AES256-SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256" => "ECDHE-RSA-AES128-SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384" => "ECDHE-RSA-AES256-SHA384",
        "TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256" => "ECDH-RSA-AES128-SHA256",
        "TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384" => "ECDH-RSA-AES256-SHA384",
        "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256" => "ECDHE-ECDSA-AES128-GCM-SHA256",
        "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384" => "ECDHE-ECDSA-AES256-GCM-SHA384",
        "TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256" => "ECDH-ECDSA-AES128-GCM-SHA256",
        "TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384" => "ECDH-ECDSA-AES256-GCM-SHA384",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256" => "ECDHE-RSA-AES128-GCM-SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384" => "ECDHE-RSA-AES256-GCM-SHA384",
        "TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256" => "ECDH-RSA-AES128-GCM-SHA256",
        "TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384" => "ECDH-RSA-AES256-GCM-SHA384",
        "TLS_ECDHE_PSK_WITH_RC4_128_SHA" => "ECDHE-PSK-RC4-SHA",
        "TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA" => "ECDHE-PSK-3DES-EDE-CBC-SHA",
        "TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA" => "ECDHE-PSK-AES128-CBC-SHA",
        "TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA" => "ECDHE-PSK-AES256-CBC-SHA",
        "TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256" => "ECDHE-PSK-AES128-CBC-SHA256",
        "TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384" => "ECDHE-PSK-AES256-CBC-SHA384",
        "TLS_ECDHE_PSK_WITH_NULL_SHA" => "ECDHE-PSK-NULL-SHA",
        "TLS_ECDHE_PSK_WITH_NULL_SHA256" => "ECDHE-PSK-NULL-SHA256",
        "TLS_ECDHE_PSK_WITH_NULL_SHA384" => "ECDHE-PSK-NULL-SHA384",
        "TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256" => "ECDHE-ECDSA-CAMELLIA128-SHA256",
        "TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384" => "ECDHE-ECDSA-CAMELLIA256-SHA384",
        "TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256" => "ECDH-ECDSA-CAMELLIA128-SHA256",
        "TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384" => "ECDH-ECDSA-CAMELLIA256-SHA384",
        "TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256" => "ECDHE-RSA-CAMELLIA128-SHA256",
        "TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384" => "ECDHE-RSA-CAMELLIA256-SHA384",
        "TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256" => "ECDH-RSA-CAMELLIA128-SHA256",
        "TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384" => "ECDH-RSA-CAMELLIA256-SHA384",
        "TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256" => "PSK-CAMELLIA128-SHA256",
        "TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384" => "PSK-CAMELLIA256-SHA384",
        "TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256" => "DHE-PSK-CAMELLIA128-SHA256",
        "TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384" => "DHE-PSK-CAMELLIA256-SHA384",
        "TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256" => "RSA-PSK-CAMELLIA128-SHA256",
        "TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384" => "RSA-PSK-CAMELLIA256-SHA384",
        "TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256" => "ECDHE-PSK-CAMELLIA128-SHA256",
        "TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384" => "ECDHE-PSK-CAMELLIA256-SHA384",
        "TLS_RSA_WITH_AES_128_CCM" => "AES128-CCM",
        "TLS_RSA_WITH_AES_256_CCM" => "AES256-CCM",
        "TLS_DHE_RSA_WITH_AES_128_CCM" => "DHE-RSA-AES128-CCM",
        "TLS_DHE_RSA_WITH_AES_256_CCM" => "DHE-RSA-AES256-CCM",
        "TLS_RSA_WITH_AES_128_CCM_8" => "AES128-CCM8",
        "TLS_RSA_WITH_AES_256_CCM_8" => "AES256-CCM8",
        "TLS_DHE_RSA_WITH_AES_128_CCM_8" => "DHE-RSA-AES128-CCM8",
        "TLS_DHE_RSA_WITH_AES_256_CCM_8" => "DHE-RSA-AES256-CCM8",
        "TLS_PSK_WITH_AES_128_CCM" => "PSK-AES128-CCM",
        "TLS_PSK_WITH_AES_256_CCM" => "PSK-AES256-CCM",
        "TLS_DHE_PSK_WITH_AES_128_CCM" => "DHE-PSK-AES128-CCM",
        "TLS_DHE_PSK_WITH_AES_256_CCM" => "DHE-PSK-AES256-CCM",
        "TLS_PSK_WITH_AES_128_CCM_8" => "PSK-AES128-CCM8",
        "TLS_PSK_WITH_AES_256_CCM_8" => "PSK-AES256-CCM8",
        "TLS_PSK_DHE_WITH_AES_128_CCM_8" => "DHE-PSK-AES128-CCM8",
        "TLS_PSK_DHE_WITH_AES_256_CCM_8" => "DHE-PSK-AES256-CCM8",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CCM" => "ECDHE-ECDSA-AES128-CCM",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CCM" => "ECDHE-ECDSA-AES256-CCM",
        "TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8" => "ECDHE-ECDSA-AES128-CCM8",
        "TLS_ECDHE_ECDSA_WITH_AES_256_CCM_8" => "ECDHE-ECDSA-AES256-CCM8",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256_OLD" => "ECDHE-RSA-CHACHA20-POLY1305-OLD",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256_OLD" => "ECDHE-ECDSA-CHACHA20-POLY1305-OLD",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256" => "ECDHE-RSA-CHACHA20-POLY1305",
        "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256" => "ECDHE-ECDSA-CHACHA20-POLY1305",
        "TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256_OLD" => "DHE-RSA-CHACHA20-POLY1305-OLD",
        "TLS_GOSTR341094_RSA_WITH_28147_CNT_MD5" => "GOST-MD5",
        "TLS_RSA_WITH_28147_CNT_GOST94" => "GOST-GOST94",
        "SSL_CK_RC4_128_WITH_MD5" => "RC4-MD5",
        "SSL_CK_RC4_128_EXPORT40_WITH_MD5" => "EXP-RC4-MD5",
        "SSL_CK_RC2_128_CBC_WITH_MD5" => "RC2-CBC-MD5",
        "SSL_CK_RC2_128_CBC_EXPORT40_WITH_MD5" => "EXP-RC2-CBC-MD5",
        "SSL_CK_IDEA_128_CBC_WITH_MD5" => "IDEA-CBC-MD5",
        "SSL_CK_DES_64_CBC_WITH_MD5" => "DES-CBC-MD5",
        "SSL_CK_DES_64_CBC_WITH_SHA" => "DES-CBC-SHA",
        "SSL_CK_DES_192_EDE3_CBC_WITH_MD5" => "DES-CBC3-MD5",
        "SSL_CK_DES_192_EDE3_CBC_WITH_SHA" => "DES-CBC3-SHA",
        "SSL_CK_RC4_64_WITH_MD5" => "RC4-64-MD5",
        "SSL_CK_DES_64_CFB64_WITH_MD5_1" => "DES-CFB-M1",
        "SSL_CK_NULL" => "NULL"
    };
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
