use std::{println, vec, vec::Vec};

use super::cose_data::SignatureAlgorithm;
use crate::null_terminate_bytes;

use mbedtls::pk::{EcGroup, EcGroupId, Pk};
use mbedtls::ecp::EcPoint;
use mbedtls::x509::certificate::Certificate;
use mbedtls::hash as mbedtls_hash;

pub fn validate(
    masa_pem: Option<&[u8]>,
    (signer_cert, signature, alg, msg): (Option<&[u8]>, &[u8], &SignatureAlgorithm, &[u8])
) -> Result<(), mbedtls::Error> {
    println!("validate::validate(): ^^");

    //---- TODO custom error stuff
    let err_todo_00 = mbedtls::Error::Utf8Error(None); // println!("validate(): failed to compute digest of msg: {:?}", msg);
    let err_todo_01 = mbedtls::Error::Utf8Error(None); // println!("Neither external masa cert nor signer cert is available.");
    //----

    let (ref digest, md_type) = compute_digest(msg, alg).ok_or(err_todo_00)?;
    println!("validate(): msg: {:?}\n  --> digest: {:?}", msg, digest);

    let signature = &asn1_from_signature(signature);

    if let Some(pem) = masa_pem {
        println!("validate(): [masa pem] len: {}", pem.len());

        Certificate::from_pem(&null_terminate_bytes!(pem))?
            .public_key_mut()
            .verify(md_type, digest, signature)
    } else if let Some(cert) = signer_cert {
        println!("validate(): [signer cert] len: {}", cert.len());

        let grp = EcGroup::new(EcGroupId::SecP256R1)?;

        let prefix = *cert.get(0).unwrap();
        if prefix == 0x02 || prefix == 0x03 {
            // "Compressed point, which mbedtls does not understand"
            // per 'src/ecp/mod.rs' of rust-mbedtls crate
            println!("validate(): warning: `cert` cannot be processed by vanilla mbedtls");
        }
        let pt = EcPoint::from_binary(&grp, cert)?;

        Pk::public_from_ec_components(grp.clone(), pt)?
            .verify(md_type, digest, signature)
    } else {
        Err(err_todo_01)
    }
}

fn asn1_from_signature(signature: &[u8]) -> Vec<u8> {
    let half = signature.len() / 2;
    let h = half as u8;
    let mut asn1 = vec![];
    asn1.extend_from_slice(&[48, 2 * h + 6, 2, h + 1, 0]);
    asn1.extend_from_slice(&signature[..half]); // r
    asn1.extend_from_slice(&[2, h + 1, 0]);
    asn1.extend_from_slice(&signature[half..]); // s

    asn1
}

fn compute_digest(msg: &[u8], alg: &SignatureAlgorithm) -> Option<(Vec<u8>, mbedtls_hash::Type)> {
    let (md_type, digest_len) = match *alg {
        SignatureAlgorithm::ES256 => (mbedtls_hash::Type::Sha256, 32),
        SignatureAlgorithm::ES384 => (mbedtls_hash::Type::Sha384, 48),
        SignatureAlgorithm::ES512 => (mbedtls_hash::Type::Sha512, 64),
        SignatureAlgorithm::PS256 => unimplemented!("TODO: handle PS256"),
    };

    let mut digest = vec![0; digest_len];
    mbedtls_hash::Md::hash(md_type, msg, &mut digest).ok()?;

    Some((digest, md_type))
}