#![no_std]
#![feature(alloc_error_handler)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::println;

#[no_mangle]
pub extern fn start() {
    println!("[src/lib.rs] start(): ^^");
    psa_demo();
    psa_tests();
}

fn psa_demo() {
    use minerva_mbedtls_test::minerva_mbedtls::{psa_crypto::{self, ffi}, psa_ifce};
    println!("psa_demo(): ^^");

    println!("enum value of `ffi::MD_SHA256`: {:?}", ffi::MD_SHA256); // 4
    println!("enum value of `ffi::MD_SHA384`: {:?}", ffi::MD_SHA384); // 5
    println!("enum value of `ffi::MD_SHA512`: {:?}", ffi::MD_SHA512); // 6

    psa_crypto::init().unwrap();
    psa_crypto::initialized().unwrap();

    let _ = psa_ifce::pk_context::new(); // ok
    let _ = psa_ifce::x509_crt::new(); // ok

    println!("psa_demo(): vv");
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

    println!("psa_tests(): vv");
}