#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::{println, alloc::boxed::Box, null_terminate_bytes};

mod xbd;
use xbd::{Xbd, XbdFnsEnt, process_xbd_callbacks};

mod blogos12;
mod embassy;

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_fns_ptr: *const XbdFnsEnt,
    xbd_fns_sz: usize
) {
    println!("[src/lib.rs] rustmod_start(): ^^");

    xbd::init_once(xbd_fns_ptr, xbd_fns_sz);

    if 0 == 1 { // debug
        Xbd::usleep(1_000_000);
        blogos12::test_misc();
        return;
    }

    if 0 == 1 {
        println!("@@ running `xbd_main()` with `blogos12::Runtime` ...");
        blogos12::Runtime::new().unwrap().block_on(xbd_main());
        panic!("should be never reached");
    }

    if 1 == 1 {
        println!("@@ running `xbd_main()` with `embassy::Runtime` ...");
        embassy::Runtime::new_static().unwrap().run();
    }
}

async fn xbd_main() {

    if 0 == 1 { // non-blocking, ok
        use blogos12::keyboard::add_scancode as blogos12_add_scancode;

        let foo = Box::new(9);
        Xbd::set_timeout(2500, move |_| {
            println!("@@ ||aa: ^^ foo: {:?}", foo);
            blogos12_add_scancode(8);
            blogos12_add_scancode(*foo);
        });

        fn ff(_: ()) { println!("@@ ff(): ^^"); }
        Xbd::set_timeout(2500, ff);
    }

    if 0 == 1 { // async, ok
        Xbd::async_sleep(3500).await; // ok
        Xbd::async_set_timeout(3500, || { println!("@@ ||x: ^^"); }).await; // ok
    }

    //

    let req_internal_native = ("[fe80::78ec:5fff:febd:add9]:5683", "/.well-known/core");
    let req_external_native = ("[fe80::20be:cdff:fe0e:44a1]:5683", "/hello");

    if 0 == 1 { // non-blocking, ok
        let cb = |out| { println!("@@ out: {:?}", out); };

        //==== native, internal server
        let (addr, uri) = req_internal_native;
        Xbd::gcoap_get(addr, uri, cb);

        //==== native, external server -- LD_LIBRARY_PATH=./libcoap/local/lib libcoap-minimal/server 5683 fe80::20be:cdff:fe0e:44a1%tap1 &
        let (addr, uri) = req_external_native;
        Xbd::gcoap_get(addr, uri, cb);
    }

    if 1 == 1 { // async, dev, server !!!!
        let addr_self = req_internal_native.0;

        let out = Xbd::async_gcoap_get(addr_self, "/.well-known/core").await; println!("@@ out: {:?}", out);
        let out = Xbd::async_gcoap_get(addr_self, "/cli/stats").await; /* 1 */ println!("@@ out: {:?}", out);
        let out = Xbd::async_gcoap_get(addr_self, "/riot/board").await; println!("@@ out: {:?}", out);
        let out = Xbd::async_gcoap_get(addr_self, "/cli/stats").await; /* 3 */ println!("@@ out: {:?}", out);
        //
        panic!("ok");
    }

    if 100 == 1 { // async, ok
        Xbd::async_set_timeout(999, || { println!("@@ xbd_main(): ==== async APIs"); }).await;
        let (addr, uri) = req_internal_native;

        // test case invalid `addr`
        let out = Xbd::async_gcoap_get("[fe80::78ec:5fff:febd:aaaa]:5683", uri).await;
        println!("@@ out: {:?}", out);

        // test case invalid `uri`
        let out = Xbd::async_gcoap_get(addr, "/.well-known/cccc").await;
        println!("@@ out: {:?}", out);

        // test hitting the internal server, native-only!!
        let out = Xbd::async_gcoap_get(addr, uri).await;
        println!("@@ out: {:?}", out);

        // test hitting the external server
        let out = Xbd::async_gcoap_get(req_external_native.0, req_external_native.1).await;
        println!("@@ out: {:?}", out);
    }
}