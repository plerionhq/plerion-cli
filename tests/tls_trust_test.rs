use rustls::client::{danger::ServerCertVerifier, WebPkiServerVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::RootCertStore;
use std::{io::BufReader, sync::Arc, time::Duration};

fn certificates(pem: &[u8]) -> Vec<CertificateDer<'static>> {
    rustls_pemfile::certs(&mut BufReader::new(pem))
        .collect::<Result<_, _>>()
        .unwrap()
}

fn verifier(roots: RootCertStore) -> Arc<WebPkiServerVerifier> {
    WebPkiServerVerifier::builder_with_provider(
        Arc::new(roots),
        Arc::new(rustls::crypto::ring::default_provider()),
    )
    .build()
    .unwrap()
}

#[test]
fn compromised_chains_are_rejected_even_with_their_roots() {
    let mut roots = RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let default_verifier = verifier(roots.clone());
    for name in [
        "aaa-certificate-services",
        "emsign-ecc-root-ca-g3",
        "emsign-root-cs-ca-g3",
        "emsign-root-tls-ca-g3",
    ] {
        let pem = std::fs::read(format!("tests/fixtures/tls/{name}.pem")).unwrap();
        let cert = certificates(&pem).remove(0);
        let mut anchor = RootCertStore::empty();
        anchor.add(cert.clone()).unwrap();
        assert!(
            !roots
                .roots
                .iter()
                .any(|r| r.subject_public_key_info == anchor.roots[0].subject_public_key_info),
            "Bundled roots contain {name}"
        );
        roots.add(cert).unwrap();
    }
    let seeded_verifier = verifier(roots);
    let now = UnixTime::since_unix_epoch(Duration::from_secs(1793059200));
    let mut checked = 0;
    for entry in std::fs::read_dir("tests/fixtures/tls").unwrap() {
        let path = entry.unwrap().path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        if !filename.ends_with(".com.pem") {
            continue;
        }
        let chain = certificates(&std::fs::read(&path).unwrap());
        let hostname = filename.strip_suffix(".pem").unwrap();
        let name = ServerName::try_from(hostname).unwrap();
        assert!(
            default_verifier
                .verify_server_cert(&chain[0], &chain[1..], &name, &[], now)
                .is_err(),
            "Default roots accepted {hostname}"
        );
        let result = seeded_verifier.verify_server_cert(&chain[0], &chain[1..], &name, &[], now);
        if std::env::var_os("EXPECT_OLD_VERIFIER_ACCEPTANCE").is_some() {
            assert!(
                result.is_ok(),
                "Old verifier did not accept {hostname}: {result:?}"
            );
        } else {
            assert!(
                format!("{result:?}").contains("CertDistrusted"),
                "Expected compromised-key rejection for {hostname}: {result:?}"
            );
        }
        println!("{hostname}: {result:?}");
        checked += 1;
    }
    assert_eq!(checked, 14);
}

#[test]
fn private_ca_and_hostname_verification_still_work() {
    let certified = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert = certified.cert.der().clone();
    let mut roots = RootCertStore::empty();
    roots.add(cert.clone()).unwrap();
    let verifier = verifier(roots);
    let name = ServerName::try_from("localhost").unwrap();
    assert!(verifier
        .verify_server_cert(&cert, &[], &name, &[], UnixTime::now())
        .is_ok());
    let wrong = ServerName::try_from("wrong.example").unwrap();
    assert!(verifier
        .verify_server_cert(&cert, &[], &wrong, &[], UnixTime::now())
        .is_err());
}
