use crate::{VOUCHER_JADA, VOUCHER_F2_00_02, MASA_PEM_F2_00_02};
use minerva_voucher::{Voucher, Validate, SignatureAlgorithm};

#[test]
fn test_voucher_decode_jada() {
    #[cfg(feature = "v3")]
    crate::init_psa_crypto();

    let vch = Voucher::from(VOUCHER_JADA);

    let (sig, alg) = vch.get_signature();
    assert_eq!(sig.len(), 64);
    assert_eq!(*alg, SignatureAlgorithm::ES256);

    assert_eq!(vch.get_signer_cert().unwrap().len(), 65);
}

#[test]
fn test_voucher_validate_jada() {
    #[cfg(feature = "v3")]
    crate::init_psa_crypto();

    let vch = Voucher::from(VOUCHER_JADA);

    // No external masa cert; use `signer_cert` embedded in COSE unprotected
    assert!(vch.validate(None));
}

#[test]
fn test_voucher_decode_f2_00_02() {
    #[cfg(feature = "v3")]
    crate::init_psa_crypto();

    let vch = Voucher::from(VOUCHER_F2_00_02);

    let (sig, alg) = vch.get_signature();
    assert_eq!(sig.len(), 64);
    assert_eq!(*alg, SignatureAlgorithm::ES256);

    assert_eq!(vch.get_signer_cert(), None);
}

#[test]
fn test_voucher_validate_f2_00_02() {
    #[cfg(feature = "v3")]
    crate::init_psa_crypto();

    let vch = Voucher::from(VOUCHER_F2_00_02);

    let masa_pem = MASA_PEM_F2_00_02;
    assert_eq!(masa_pem.len(), 684);

    assert!(vch.validate(Some(masa_pem)));
}
