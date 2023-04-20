#![no_std]
#![feature(alloc_error_handler)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::println;
use minerva_mbedtls_test::{minerva_mbedtls::{self, psa_crypto}};

#[no_mangle]
pub extern fn start() {
    println!("[src/lib.rs] start(): ^^");

    println!("🧪 test before `psa_crypto::init()` ...");
    assert!(psa_crypto::initialized().is_err());
    println!("{}", if psa_demo() == Err(-16000) { "✅" } else { "❌" });

    psa_crypto::init().unwrap();

    println!("🧪 test after `psa_crypto::init()` ...");
    assert!(psa_crypto::initialized().is_ok());
    println!("{}", if psa_demo() == Ok(true) { "✅" } else { "❌" });

    psa_tests();
}

fn psa_demo() -> Result<bool, minerva_mbedtls::mbedtls_error> {
    println!("psa_demo(): ^^");

    { // ok
        use minerva_mbedtls::{psa_crypto::ffi, psa_ifce};
        assert_eq!(ffi::MD_SHA256, 4);
        assert_eq!(ffi::MD_SHA384, 5);
        assert_eq!(ffi::MD_SHA512, 6);
        let _ = psa_ifce::pk_context::new();
        let _ = psa_ifce::x509_crt::new();
    }

    let ret = {
        use minerva_mbedtls::psa_ifce::*;

        // product jada
        let hash = &md_info::from_type(MD_SHA256)
            .md(/* `to_verify` */ &[132, 106, 83, 105, 103, 110, 97, 116, 117, 114, 101, 49, 65, 160, 64, 88, 185, 161, 26, 0, 15, 70, 140, 166, 5, 105, 112, 114, 111, 120, 105, 109, 105, 116, 121, 6, 193, 26, 87, 247, 248, 30, 8, 193, 26, 89, 208, 48, 0, 14, 109, 74, 65, 68, 65, 49, 50, 51, 52, 53, 54, 55, 56, 57, 11, 105, 97, 98, 99, 100, 49, 50, 51, 52, 53, 13, 120, 124, 77, 70, 107, 119, 69, 119, 89, 72, 75, 111, 90, 73, 122, 106, 48, 67, 65, 81, 89, 73, 75, 111, 90, 73, 122, 106, 48, 68, 65, 81, 99, 68, 81, 103, 65, 69, 78, 87, 81, 79, 122, 99, 78, 77, 85, 106, 80, 48, 78, 114, 116, 102, 101, 66, 99, 48, 68, 74, 76, 87, 102, 101, 77, 71, 103, 67, 70, 100, 73, 118, 54, 70, 85, 122, 52, 68, 105, 102, 77, 49, 117, 106, 77, 66, 101, 99, 47, 103, 54, 87, 47, 80, 54, 98, 111, 84, 109, 121, 84, 71, 100, 70, 79, 104, 47, 56, 72, 119, 75, 85, 101, 114, 76, 53, 98, 112, 110, 101, 75, 56, 115, 103, 61, 61]);
        let sig = &[234, 232, 104, 236, 193, 118, 136, 55, 102, 197, 220, 91, 165, 184, 220, 162, 93, 171, 60, 46, 86, 165, 81, 206, 87, 5, 183, 147, 145, 67, 72, 225, 145, 46, 83, 95, 231, 182, 170, 68, 123, 26, 104, 156, 7, 204, 120, 204, 21, 231, 109, 98, 125, 108, 112, 63, 147, 120, 2, 102, 156, 19, 172, 227];

        let grp = ecp_group::from_id(ECP_DP_SECP256R1).unwrap();
        let mut pt = ecp_point::new();
        pt.read_binary(grp, /* `signer_cert` */ &[4, 186, 197, 177, 28, 173, 143, 153, 249, 199, 43, 5, 207, 75, 158, 38, 210, 68, 220, 24, 159, 116, 82, 40, 37, 90, 33, 154, 134, 214, 160, 158, 255, 32, 19, 139, 248, 45, 193, 182, 213, 98, 190, 15, 165, 74, 183, 128, 74, 58, 100, 182, 215, 44, 207, 237, 107, 111, 182, 237, 40, 187, 252, 17, 126]).unwrap();

        // `.verify()` should return an error in case PSA is not initialized yet!!
        pk_context::new()
            .setup(PK_ECKEY).unwrap()
            .set_grp(ecp_group::from_id(ECP_DP_SECP256R1).unwrap())
            .set_q(pt)
            .verify(MD_SHA256, hash, sig)
    };

    ret
}

fn psa_tests() {
    use minerva_mbedtls_test::*;
    println!("psa_tests(): ^^");

    type TestType = fn() -> Result<(), minerva_mbedtls::mbedtls_error>;
    let tv = [
        ("test_md", test_md as TestType),
        ("test_pk_context_verify_via_ecp", test_pk_context_verify_via_ecp),
        ("test_pk_context_verify_via_x509_crt", test_pk_context_verify_via_x509_crt),
        ("test_pk_context_sign", test_pk_context_sign),
        ("test_utils_is_asn1_signature", test_utils_is_asn1_signature),
    ];

    tv.iter().enumerate().for_each(|(i, (title, test))| {
        println!("🧪 [{}/{}] {} ... ", i + 1, tv.len(), title);
        println!("{}", if test().is_ok() { "✅" } else { "❌" });
    });

    println!("psa_tests(): $$");
}
