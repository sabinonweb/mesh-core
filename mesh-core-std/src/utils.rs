use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair};
use std::collections::HashSet;
use time::{Duration, OffsetDateTime};

pub fn handle_packet(seen: &mut HashSet<u64>, seq: u64, payload: &[u8]) -> bool {
    if seen.contains(&seq) {
        log::info!("Duplicate packet {}", seq);
        false
    } else {
        seen.insert(seq);
        println!("Hashset {:?}", seen);
        log::info!("Received {}: {}", seq, String::from_utf8_lossy(payload));
        true
    }
}

pub fn generate_certificate_authority() -> (Certificate, rcgen::Issuer<'static, KeyPair>) {
    let mut params = CertificateParams::default();
    params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "MyTestRootCA");
    params.distinguished_name = dn;

    params
        .key_usages
        .push(rcgen::KeyUsagePurpose::DigitalSignature);
    params.key_usages.push(rcgen::KeyUsagePurpose::KeyCertSign);
    params.key_usages.push(rcgen::KeyUsagePurpose::CrlSign); // CrlSign is for yedi kunai lai revoke
                                                             // garnu paryo bahne yo list pathaune and
                                                             // if in crl list reject garne
    let one_year = Duration::days(365 * 5);
    params.not_before = OffsetDateTime::now_utc()
        .checked_sub(Duration::days(1))
        .unwrap(); // start yesterday
    params.not_after = OffsetDateTime::now_utc().checked_add(one_year).unwrap();

    let key_pair = KeyPair::generate().unwrap();

    let ca_cert = params.self_signed(&key_pair).unwrap();
    let issuer = rcgen::Issuer::new(params, key_pair);

    (ca_cert, issuer)
}

pub fn generate_node_certs(
    issuer: &rcgen::Issuer<'static, KeyPair>,
    node_name: &str,
) -> (Certificate, KeyPair) {
    // using inside it makes the node name SAN
    let mut params =
        CertificateParams::new(vec![node_name.into(), "localhost".to_string()]).unwrap();
    params
        .distinguished_name
        .push(DnType::CommonName, node_name);
    // it contains hash of CA's public key to verify who singed it
    params.use_authority_key_identifier_extension = true;

    let one_day = Duration::days(1);
    let five_years = Duration::days(365 * 5);
    params.not_before = OffsetDateTime::now_utc().checked_sub(one_day).unwrap();
    params.not_after = OffsetDateTime::now_utc().checked_add(five_years).unwrap();

    // allowed to sign data during TLS handshake
    params
        .key_usages
        .push(rcgen::KeyUsagePurpose::DigitalSignature);
    // valid for server authentication in TLS
    params
        .extended_key_usages
        .push(rcgen::ExtendedKeyUsagePurpose::ServerAuth);

    let key_pair = KeyPair::generate().unwrap();
    (params.signed_by(&key_pair, &issuer).unwrap(), key_pair)
}
