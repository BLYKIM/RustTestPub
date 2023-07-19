use anyhow::Result;
use rustls::{Certificate, ClientConfig, RootCertStore};
use std::{fs::read, path::Path};

pub fn build_client_config<P: AsRef<Path>>(roots: &[P]) -> Result<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    match rustls_native_certs::load_native_certs() {
        Ok(certs) => {
            for cert in certs {
                root_store.add(&rustls::Certificate(cert.0))?;
            }
        }
        Err(_e) => {}
    };
    for root in roots {
        let certs = read_certificate_from_path(root)?;
        for cert in certs {
            root_store.add(&cert)?;
        }
    }

    let builder = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    Ok(builder)
}

pub fn read_certificate_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Certificate>> {
    let cert = read(&path)?;
    Ok(rustls_pemfile::certs(&mut &*cert)?
        .into_iter()
        .map(Certificate)
        .collect())
}
