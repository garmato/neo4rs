use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub enum ConnectionTLSConfig {
    None,
    ClientCACertificate(ClientCertificate),
    NoSSLValidation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientCertificate {
    pub(crate) cert_file: PathBuf,
}

impl ClientCertificate {
    pub fn new(path: impl AsRef<Path>) -> Self {
        ClientCertificate {
            cert_file: path.as_ref().to_path_buf(),
        }
    }
}
