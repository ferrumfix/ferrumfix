//! *FIX-over-TLS* ([FIXS](https://www.fixtrading.org/standards/fixs/))
//! utilities.
//!
//! This module is enabled by the `fixs` feature flag. Most additional FIXS
//! utilities are only available with the `utils-openssl` enabled, which
//! adds a dependency to OpenSSL. The minimal OpenSSL version required is 1.1.1+,
//! but **this may change in the future** and all users are advised to update
//! OpenSSL to avoid any issues.

use lazy_static::lazy_static;
#[cfg(feature = "utils-openssl")]
use openssl::ssl::*;
use std::collections::HashMap;

lazy_static! {
    static ref IANA_TO_OPENSSL: HashMap<String, String> = {
        let json_file = include_str!("iana2openssl.json");
        serde_json::from_str(json_file)
            .expect("IANA-to-OpenSSL mapping JSON file is invalid. This is a bug.")
    };
}

/// Which version of FIX-over-TLS (FIXS) to use.
#[derive(Debug, Copy, Clone)]
pub enum Version {
    /// Draft of the FIXS version 1.0 standard.
    V1Draft,
}

impl Version {
    /// Returns a [`Vec`] of the suggested ciphersuites for TLS,
    /// according to `self` version. The ciphersuites are specified in IANA format.
    ///
    /// ```
    /// use fefix::fixs::Version;
    ///
    /// let version = Version::V1Draft;
    /// let ciphersuites_iana = version.recommended_cs_iana(false);
    /// assert!(ciphersuites_iana.iter().any(|cs| cs == &"TLS_DHE_RSA_WITH_AES_128_GCM_SHA256"));
    /// ```
    pub fn recommended_cs_iana(&self, psk: bool) -> Vec<String> {
        match (self, psk) {
            (Version::V1Draft, false) => V1_DRAFT_RECOMMENDED_CIPHERSUITES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            (Version::V1Draft, true) => V1_DRAFT_RECOMMENDED_CIPHERSUITES
                .iter()
                .chain(V1_DRAFT_RECOMMENDED_CIPHERSUITES_PSK_ONLY)
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Returns a [`Vec`] of the suggested ciphersuites for TLS,
    /// according to `self` version. The ciphersuites are specified in OpenSSL's
    /// format.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fefix::fixs::Version;
    ///
    /// let version = Version::V1Draft;
    /// let ciphersuites_openssl = version.recommended_cs_openssl(false);
    /// assert!(ciphersuites_openssl.iter().any(|cs| cs == &"DHE-RSA-AES128-GCM-SHA256"));
    /// ```
    ///
    /// List all ciphersuites in a colon-separated format, like required by
    /// [`openssl-ciphers`](https://www.openssl.org/docs/manmaster/man1/openssl-ciphers.html).
    ///
    /// ```
    /// use fefix::fixs::Version;
    ///
    /// let version = Version::V1Draft;
    /// let ciphersuites_openssl = version.recommended_cs_openssl(false);
    /// let cipherlist = ciphersuites_openssl.join(":");
    /// println!("Supported ciphers: {}", cipherlist);
    /// ```
    pub fn recommended_cs_openssl(&self, psk: bool) -> Vec<String> {
        self.recommended_cs_iana(psk)
            .iter()
            .map(|s| IANA_TO_OPENSSL.get(s.as_str()).unwrap().clone())
            .collect()
    }

    /// Creates an [`SslConnectorBuilder`] with fhe FIXS recommended settings.
    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    pub fn recommended_connector_builder(&self) -> SslConnectorBuilder {
        let mut context = SslConnector::builder(SslMethod::tls()).unwrap();
        match self {
            Version::V1Draft => {
                context
                    .set_min_proto_version(Some(SslVersion::TLS1_2))
                    .unwrap();
                context.set_options(SslOptions::NO_COMPRESSION);
                context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
            }
        };
        context.set_session_cache_mode(SslSessionCacheMode::SERVER);
        context
            .set_cipher_list(self.recommended_cs_openssl(false).join(":").as_str())
            .unwrap();
        context
    }

    /// Creates an [`SslAcceptorBuilder`] with fhe FIXS recommended settings.
    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    pub fn recommended_acceptor_builder(&self) -> SslAcceptorBuilder {
        let mut context = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
        match self {
            Version::V1Draft => {
                context
                    .set_min_proto_version(Some(SslVersion::TLS1_2))
                    .unwrap();
                context.set_session_cache_mode(SslSessionCacheMode::SERVER);
                context.set_options(SslOptions::CIPHER_SERVER_PREFERENCE);
                context.set_options(SslOptions::NO_COMPRESSION);
                context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
            }
        };
        context
            .set_cipher_list(self.recommended_cs_openssl(false).join(":").as_str())
            .unwrap();
        context
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg(feature = "utils-openssl")]
    fn v1draft_acceptor_is_ok() {
        Version::V1Draft.recommended_acceptor_builder();
    }

    #[test]
    #[cfg(feature = "utils-openssl")]
    fn v1draft_connector_is_ok() {
        Version::V1Draft.recommended_connector_builder();
    }
}
