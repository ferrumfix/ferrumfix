//! Presets of standard TLS parameters as specified by *FIX-over-TLS*
//! ([FIXS](https://www.fixtrading.org/standards/fixs/)).

mod iana_to_openssl;

use iana_to_openssl::IANA_TO_OPENSSL;
#[cfg(feature = "utils-openssl")]
use openssl::ssl::*;

/// A release of the FIXS standard with its own recommended settings.
pub trait FixOverTlsVersion {
    /// Returns a [`Vec`] of the suggested ciphersuites for TLS,
    /// according to `self` version. The ciphersuites are specified in IANA format.
    ///
    /// ```
    /// use fefixs::{FixOverTlsVersion, FixOverTlsV1};
    ///
    /// let ciphersuites_iana = FixOverTlsV1.recommended_cs_iana(false);
    /// assert!(ciphersuites_iana.iter().any(|cs| cs == &"TLS_DHE_RSA_WITH_AES_128_GCM_SHA256"));
    /// ```
    fn recommended_cs_iana(&self, psk: bool) -> Vec<String>;

    /// Returns a [`Vec`] of the suggested ciphersuites for TLS,
    /// according to `self` version. The ciphersuites are specified in OpenSSL's
    /// format.
    ///
    /// # Examples:
    ///
    /// ```
    /// use fefixs::{FixOverTlsVersion, FixOverTlsV1};
    ///
    /// let ciphersuites_openssl = FixOverTlsV1.recommended_cs_openssl(false);
    /// assert!(ciphersuites_openssl.iter().any(|cs| cs == &"DHE-RSA-AES128-GCM-SHA256"));
    /// ```
    ///
    /// List all ciphersuites in a colon-separated format, like required by
    /// [`openssl-ciphers`](https://www.openssl.org/docs/manmaster/man1/openssl-ciphers.html).
    ///
    /// ```
    /// use fefixs::{FixOverTlsVersion, FixOverTlsV1};
    ///
    /// let ciphersuites_openssl = FixOverTlsV1.recommended_cs_openssl(false);
    /// let cipherlist = ciphersuites_openssl.join(":");
    /// println!("Supported ciphers: {}", cipherlist);
    /// ```
    fn recommended_cs_openssl(&self, psk: bool) -> Vec<String>;

    /// Creates an [`SslConnectorBuilder`] with fhe FIXS recommended settings.
    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    fn recommended_connector_builder(&self) -> SslConnectorBuilder;

    /// Creates an [`SslAcceptorBuilder`] with fhe FIXS recommended settings.
    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    fn recommended_acceptor_builder(&self) -> SslAcceptorBuilder;
}

#[derive(Debug, Copy, Clone)]
pub struct FixOverTlsV1;

impl FixOverTlsV1 {
    const RECOMMENDED_CIPHERSUITES: &'static [&'static str] = &[
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

    const RECOMMENDED_CIPHERSUITES_PSK_ONLY: &'static [&'static str] = &[
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
}

impl FixOverTlsVersion for FixOverTlsV1 {
    fn recommended_cs_iana(&self, psk: bool) -> Vec<String> {
        let recommented_ciphersuites = if psk {
            Self::RECOMMENDED_CIPHERSUITES_PSK_ONLY
        } else {
            Self::RECOMMENDED_CIPHERSUITES
        };

        recommented_ciphersuites
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    fn recommended_cs_openssl(&self, psk: bool) -> Vec<String> {
        self.recommended_cs_iana(psk)
            .iter()
            .map(|s| {
                IANA_TO_OPENSSL
                    .iter()
                    .find(|(iana, _openssl)| iana == s)
                    .map(|(_iana, openssl)| openssl.to_string())
                    .unwrap()
            })
            .collect()
    }

    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    fn recommended_connector_builder(&self) -> SslConnectorBuilder {
        let mut context = SslConnector::builder(SslMethod::tls()).unwrap();
        context
            .set_min_proto_version(Some(SslVersion::TLS1_2))
            .unwrap();
        context.set_options(SslOptions::NO_COMPRESSION);
        context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
        context.set_session_cache_mode(SslSessionCacheMode::SERVER);
        context
            .set_cipher_list(self.recommended_cs_openssl(false).join(":").as_str())
            .unwrap();
        context
    }

    #[cfg(feature = "utils-openssl")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-openssl")))]
    fn recommended_acceptor_builder(&self) -> SslAcceptorBuilder {
        let mut context = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
        context
            .set_min_proto_version(Some(SslVersion::TLS1_2))
            .unwrap();
        context.set_session_cache_mode(SslSessionCacheMode::SERVER);
        context.set_options(SslOptions::CIPHER_SERVER_PREFERENCE);
        context.set_options(SslOptions::NO_COMPRESSION);
        context.set_options(SslOptions::NO_SESSION_RESUMPTION_ON_RENEGOTIATION);
        context
            .set_cipher_list(self.recommended_cs_openssl(false).join(":").as_str())
            .unwrap();
        context
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "utils-openssl")]
    fn v1_acceptor_is_ok() {
        use super::*;

        FixOverTlsV1.recommended_acceptor_builder();
    }

    #[test]
    #[cfg(feature = "utils-openssl")]
    fn v1_connector_is_ok() {
        use super::*;

        FixOverTlsV1.recommended_connector_builder();
    }
}
