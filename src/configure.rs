use quinn::{
    rustls,
    rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer},
    ClientConfig, Endpoint, ServerConfig,
};
use rcgen::{Issuer, KeyPair};
use std::{net::SocketAddr, sync::Arc};

use crate::{utils::generate_node_certs, MeshError, StaticMeshError};

pub fn make_endpoint(
    addr: SocketAddr,
    trusted_peers: &[CertificateDer<'static>],
    node_name: &str,
    issuer: &Issuer<'static, KeyPair>,
) -> Result<Endpoint, MeshError> {
    let (server_config, certificates) = configure_server(node_name, &issuer)?;
    let mut endpoint = Endpoint::server(server_config, addr)?;

    let mut trusted_peers: Vec<&[u8]> = trusted_peers.iter().map(|c| c.as_ref()).collect();
    trusted_peers.push(&certificates);
    let client_config = configure_client(&trusted_peers)?;
    endpoint.set_default_client_config(client_config);

    Ok(endpoint)
}

pub fn make_client_endpoint(
    server_certificates: &[&[u8]],
    addr: SocketAddr,
) -> Result<Endpoint, MeshError> {
    let client_config = configure_client(server_certificates)?;
    let mut endpoint = Endpoint::client(addr)?;
    endpoint.set_default_client_config(client_config);

    Ok(endpoint)
}

pub fn make_server_endpoint(
    addr: SocketAddr,
    node_name: &str,
    issuer: Issuer<'static, KeyPair>,
) -> Result<(Endpoint, CertificateDer<'static>), StaticMeshError> {
    let (server_config, certificates) = configure_server(node_name, &issuer)?;
    let endpoint = Endpoint::server(server_config, addr)?;

    Ok((endpoint, certificates))
}

fn configure_client(server_certificates: &[&[u8]]) -> Result<ClientConfig, StaticMeshError> {
    // rustls stores trusted certificates in RootCert
    let mut certificates = rustls::RootCertStore::empty();
    for cert in server_certificates {
        certificates.add(CertificateDer::from(*cert))?;
    }

    Ok(ClientConfig::with_root_certificates(Arc::new(
        certificates,
    ))?)
}

fn configure_server(
    node_name: &str,
    issuer: &Issuer<'static, KeyPair>,
) -> Result<(ServerConfig, CertificateDer<'static>), StaticMeshError> {
    // for now we are using self signed certicates without any Cetificate Authority
    let (node_cert, key_pair) = generate_node_certs(&issuer, node_name);
    let certificate_der = node_cert.clone().der().clone().into_owned();
    let private_key = PrivatePkcs8KeyDer::from(key_pair.serialize_der());

    let mut server_config =
        ServerConfig::with_single_cert(vec![certificate_der.clone()], private_key.into())?;
    let transport_config = Arc::get_mut(&mut server_config.transport).unwrap();
    transport_config.max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, certificate_der))
}
