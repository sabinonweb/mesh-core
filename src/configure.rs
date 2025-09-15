use quinn::{
    rustls,
    rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    ClientConfig, Endpoint, ServerConfig,
};
use std::{net::SocketAddr, sync::Arc};

pub fn make_client_endpoint(
    server_certificates: &[&[u8]],
    addr: SocketAddr,
) -> Result<Endpoint, Box<dyn std::error::Error + Send + Sync>> {
    let client_config = configure_client(server_certificates)?;
    let mut endpoint = Endpoint::client(addr)?;
    endpoint.set_default_client_config(client_config);

    Ok(endpoint)
}

pub fn make_server_endpoint(
    addr: SocketAddr,
) -> Result<(Endpoint, CertificateDer<'static>), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    let (server_config, certificates) = configure_server()?;
    let endpoint = Endpoint::server(server_config, addr)?;

    Ok((endpoint, certificates))
}

fn configure_client(
    server_certificates: &[&[u8]],
) -> Result<ClientConfig, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // rustls stores trusted certificates in RootCert
    let mut certificates = rustls::RootCertStore::empty();
    for cert in server_certificates {
        certificates.add(CertificateDer::from(*cert))?;
    }

    Ok(ClientConfig::with_root_certificates(Arc::new(
        certificates,
    ))?)
}

fn configure_server() -> Result<
    (ServerConfig, CertificateDer<'static>),
    Box<dyn std::error::Error + Send + Sync + 'static>,
> {
    // for now we are using self signed certicates without any Cetificate Authority
    let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let certificate_der = CertificateDer::from(certificate.cert);
    let private_key = PrivatePkcs8KeyDer::from(certificate.signing_key.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![certificate_der.clone()], private_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, certificate_der))
}
