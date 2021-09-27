#![allow(unused_imports, unused_variables)]

use mcu_if::{println, alloc::{vec, vec::Vec}};

use super::cose_data::SignatureAlgorithm;

pub fn validate(
    masa_pem: Option<&[u8]>,
    (signer_cert, signature, alg, msg): (Option<&[u8]>, &[u8], &SignatureAlgorithm, &[u8])
) -> bool {

    println!("⚠️ ECDSA verification under `no_std` is WIP!");

    false
}
