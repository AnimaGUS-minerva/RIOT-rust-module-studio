use crate::{VOUCHER_JADA, VOUCHER_F2_00_02, MASA_PEM_F2_00_02};
use minerva_voucher::{Voucher, Validate, SignatureAlgorithm};
use core::convert::TryFrom;

#[test]
fn test_voucher_decode_jada() {
    crate::init_psa_crypto();

    let vch = Voucher::try_from(VOUCHER_JADA).unwrap();

    let (sig, alg) = vch.get_signature().unwrap();
    assert_eq!(sig.len(), 64);
    assert_eq!(*alg, SignatureAlgorithm::ES256);

    assert_eq!(vch.get_signer_cert().unwrap().len(), 65);
}

#[test]
fn test_voucher_validate_jada() {
    crate::init_psa_crypto();

    let vch = Voucher::try_from(VOUCHER_JADA).unwrap();

    // No external masa cert; use `signer_cert` embedded in COSE unprotected
    assert!(vch.validate(None).is_ok());
}

#[test]
fn test_voucher_decode_f2_00_02() {
    crate::init_psa_crypto();

    let vch = Voucher::try_from(VOUCHER_F2_00_02).unwrap();

    let (sig, alg) = vch.get_signature().unwrap();
    assert_eq!(sig.len(), 64);
    assert_eq!(*alg, SignatureAlgorithm::ES256);

    assert_eq!(vch.get_signer_cert(), None);
}

#[test]
fn test_voucher_validate_f2_00_02() {
    crate::init_psa_crypto();

    let vch = Voucher::try_from(VOUCHER_F2_00_02).unwrap();

    let masa_pem = MASA_PEM_F2_00_02;
    assert_eq!(masa_pem.len(), 684);

    assert!(vch.validate(Some(masa_pem)).is_ok());
}
