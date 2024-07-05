use private::Sealed;
use reqwest::{tls, Certificate};

mod private {
    pub trait Sealed {}
    impl Sealed for reqwest::Client {}
}

/// Sochat-specific extension for reqwest::Client
pub trait ClientExt: Sealed + Sized {
    fn sochat(certs: &[Certificate]) -> reqwest::Result<Self>;
}

impl ClientExt for reqwest::Client {
    fn sochat(certs: &[Certificate]) -> reqwest::Result<Self> {
        let mut builder = Self::builder();

        for c in certs {
            builder = builder.add_root_certificate(c.clone());
        }

        builder
            .http2_prior_knowledge()                // only the best
            .min_tls_version(tls::Version::TLS_1_3)
            .user_agent("SoChatClient/0.0")
            .build()
    }
}
