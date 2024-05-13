#![no_std]
#![feature(alloc_error_handler)]
#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]
#![allow(unexpected_cfgs)]

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! { mcu_if::panic(info) }

#[alloc_error_handler]
fn alloc_error(layout: mcu_if::alloc::alloc::Layout) -> ! { mcu_if::alloc_error(layout) }

use mcu_if::{println, alloc::boxed::Box};

mod xbd;
use xbd::{Xbd, XbdFnsEnt, GcoapMemoState};

mod blogos12;
mod embassy;

//

#[no_mangle]
pub extern fn rustmod_start(
    xbd_fns_ptr: *const XbdFnsEnt,
    xbd_fns_sz: usize
) {
    println!("[src/lib.rs] rustmod_start(): ^^ ================");

    if 10 == 1 {
        //use mcu_if::{alloc::vec::Vec};
        let _ = [0u8].to_vec();
        panic!("ok");
    }
    if 10 == 1 { // https://docs.rs/heapless/latest/heapless/struct.Vec.html
        use heapless::Vec;
        let mut x = Vec::<_, 2>::new();
        x.push(1).unwrap();
        x.push(2).unwrap();
        //x.push(3).unwrap();
        println!("x: {:?}", x);
        panic!("ok");
    }

    xbd::init_once(xbd_fns_ptr, xbd_fns_sz);

    if 0 == 1 { // debug
        Xbd::usleep(1_000_000);
        blogos12::test_misc();
        return;
    }

    if 1 == 1 {
        println!("@@ [debug] `xbd_main()` with `embassy::Runtime` ...");
        embassy::Runtime::new_static().unwrap().run();
    } else {
        println!("@@ [debug] `xbd_main()` with `blogos12::Runtime` ...");
        let _ = blogos12::Runtime::new()
            .unwrap()
            .block_on(xbd_main());
        panic!("should be never reached");
    }
}

async fn xbd_main() -> Result<(), i8> {

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

    if 0 == 1 { // non-blocking
        let cb = |out| { println!("@@ out: {:?}", out); };

        //==== native, internal server
        let (addr, uri) = req_internal_native;
        Xbd::gcoap_get(addr, uri, cb);

        //==== native, external server -- LD_LIBRARY_PATH=./libcoap/local/lib libcoap-minimal/server 5683 fe80::20be:cdff:fe0e:44a1%tap1 &
        let (addr, uri) = req_external_native;
        Xbd::gcoap_get(addr, uri, cb);
    }

    if 1 == 1 { // async, dev, server !!!!
        let addr_self = "[::1]:5683";

        if 10 == 1 {
            let out = Xbd::async_gcoap_get(addr_self, "/.well-known/core").await; println!("@@ out: {:?}", out);
            let out = Xbd::async_gcoap_get(addr_self, "/cli/stats").await; /* 1 */ println!("@@ out: {:?}", out);
            let out = Xbd::async_gcoap_get(addr_self, "/riot/board").await; println!("@@ out: {:?}", out);
            let out = Xbd::async_gcoap_get(addr_self, "/cli/stats").await; /* 3 */ println!("@@ out: {:?}", out);
            panic!("ok");
        }

        if 10 == 1 {
            let _out = Xbd::async_gcoap_post(addr_self, "/cli/stats", b"3000").await; // static lifetime payload
            //--
            use mcu_if::alloc::string::ToString;
            let payload = "4000".to_string(); // non-static lifetime payload
            //--
            let _out = Xbd::async_gcoap_post(addr_self, "/cli/stats", payload.as_bytes()).await;
            println!("@@ out: {:?}", Xbd::async_gcoap_get(addr_self, "/cli/stats").await);
            panic!("ok");
        }

        if 10 == 1 {
            let _out = Xbd::async_gcoap_put(addr_self, "/cli/stats", b"1000").await; // static lifetime payload
            //--
            use mcu_if::alloc::string::ToString;
            let payload = "2000".to_string(); // non-static lifetime payload
            //--
            let _out = Xbd::async_gcoap_put(addr_self, "/cli/stats", payload.as_bytes()).await;
            println!("@@ out: {:?}", Xbd::async_gcoap_get(addr_self, "/cli/stats").await);
            panic!("ok");
        }

        if 0 == 1 {
            println!("!! ======== dev calling non-blocking");
            Xbd::gcoap_get(addr_self, "/cli/stats", |out| { println!("!! out: {:?}", out); });
            Xbd::gcoap_get(addr_self, "/cli/stats", |out| { println!("!! out: {:?}", out); });
        }

        // TODO async gcoap ping

        if 0 == 1 { // fileserver, blockwise, stream
            test_blockwise(addr_self).await.unwrap();
            //panic!("debug ok"); // !!!!
        }
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

    Ok(())
}

use futures_util::stream::StreamExt;
use xbd::{BlockwiseError, BLOCKWISE_STATES_MAX, blockwise_states_print, blockwise_states_debug};

async fn test_blockwise(addr_self: &str) -> Result<(), BlockwiseError> {

    if 0 == 1 { // !! do test with alias='nns'
        println!("!! debug NEW [gcoap-dtls]");

        //---- ok <-- gcoap: authentication timed out
        //println!("@@ debug out: {:?}", Xbd::async_gcoap_get("[::1]:5684", "/cli/stats").await);
        //---- ok
        // $ libcoap/local/bin/coap-client -m get coaps://[fe80::10ef:d5ff:fe61:c7c%tap1]/cli/stats -k "secretPSK" -u "Client_identity"
        // $ libcoap/local/bin/coap-client -m get coaps://[fe80::10ef:d5ff:fe61:c7c%tap1]/const/song.txt -k "secretPSK" -u "Client_identity"
    /* note
    > ifconfig
    ...
              inet6 addr: fe80::10ef:d5ff:fe61:c7c  scope: link  VAL  <==== ok
              inet6 addr: fe80::78ec:5fff:febd:add9  scope: link  VAL <==== ? NG ?
    ...
    */
        //---- !!!!
        /* cf.
$ libcoap/local/bin/coap-server -k "secretPSK"  # TODO `-u`

$ libcoap/local/bin/coap-client -m get coaps://[::1]/.well-known/core -k "secretPSK" -u "Client_identity"
</>;title="General Info";ct=0,</time>;if="clock";rt="ticks";title="Internal Clock";ct=0;obs,</async>;ct=0,</example_data>;title="Example Data";ct=0;obs

# w.r.t. coap-server, for now, only `-k` is working
$ libcoap/local/bin/coap-client -m get coaps://[::1]/.well-known/core -k "secretPSK" -u "Client_identity_foo"
</>;title="General Info";ct=0,</time>;if="clock";rt="ticks";title="Internal Clock";ct=0;obs,</async>;ct=0,</example_data>;title="Example Data";ct=0;obs
 */
        for _ in 0..4 {
            Xbd::async_sleep(1000).await;

            //---- w.r.t. $ libcoap/local/bin/coap-server
            //         or $ libcoap/local/bin/coap-server -k "secretPSK"
            // let out = Xbd::async_gcoap_get( // nn
            //     "[fe80::20be:cdff:fe0e:44a1]:5683", "/.well-known/core").await; // ok
            //---- w.r.t. $ libcoap/local/bin/coap-server -k "secretPSK"
            // !!!! TODO integrate 'libcoap/examples/riot/examples_libcoap_client'
            // !!!! TODO error return on **auth** timeout
            let out = Xbd::async_gcoap_get( // nns
                                            "[fe80::20be:cdff:fe0e:44a1]:5684", "/.well-known/core").await; // WIP
            println!("@@ debug out: {:?}", out);
        }
    }
    //----

    if 0 == 1 { return Ok(()); }

    // first, make sure non-blockwise get works
    println!("!! debug NEW [non-blockwise-1]");
    println!("@@ debug out: {:?}", Xbd::async_gcoap_get(addr_self, "/cli/stats").await);
    println!("!! debug NEW [non-blockwise-2]");
    println!("@@ debug out: {:?}", Xbd::async_gcoap_get(addr_self, "/cli/stats").await);
    if 1 == 1 { panic!("!!"); }

    //

    let get_blockwise = || Xbd::async_gcoap_get_blockwise(addr_self, "/const/song.txt");

    //

    println!("!! debug NEW [blockwise-1]");
    let mut bs = get_blockwise()?;
    assert!(blockwise_states_debug()[0].is_some(), "debug");

    let mut debug_count = 0;
    while let Some(req) = bs.next().await {
        println!("req: {:?}", req);

        let out = req.await;
        println!("@@ out_1: {:?}", out);
        debug_count += 1;

        if debug_count == 3 {
        //if debug_count == 9 { // right after [blockwise-1] done
            println!("!! debug NEW [blockwise-2]");
            let mut bs = get_blockwise()?;
            assert!(blockwise_states_debug()[1].is_some(), "debug");

            while let Some(req) = bs.next().await {
                let out = req.await;
                println!("@@ out_2: {:?}", out);
            }

            blockwise_states_print();
            assert!(blockwise_states_debug()[1].is_none(), "debug");
        }
    }

    blockwise_states_print();
    assert!(blockwise_states_debug()[0].is_none(), "debug");

    //

    println!("!! debug NEW [blockwise-3]");
    let mut bs = get_blockwise()?;
    assert!(blockwise_states_debug()[0].is_some(), "debug");

    while let Some(req) = bs.next().await {
        //blockwise_states_print();
        let out = req.await;
        println!("@@ out_3: {:?}", out);
    }

    blockwise_states_print();
    assert!(blockwise_states_debug()[0].is_none(), "debug");

    //

    let mut bss = heapless::Vec::<_, BLOCKWISE_STATES_MAX>::new();
    for _ in 0..BLOCKWISE_STATES_MAX {
        bss.push(get_blockwise()?).unwrap();
    }
    assert_eq!(get_blockwise().err(), Some(BlockwiseError::StateNotAvailable));

    //

    let req0 = bss[0].next().await.unwrap();
    let req1 = bss[1].next().await.unwrap();

    // before `.close()`
    assert!(match req0.await {
        GcoapMemoState::Resp(Some(x)) => x.len() > 0,
        _ => false,
    });

    bss.iter().for_each(|bs| bs.close());
    blockwise_states_debug().iter().for_each(|x| assert!(x.is_none()));

    // after `.close()`
    assert!(bss[0].next().await.is_none());
    assert!(bss[1].next().await.is_none());
    assert_eq!(req1.await, GcoapMemoState::Err);

    //

    println!("!! debug NEW [blockwise-timeout]");
    let get_blockwise_timeout = || Xbd::async_gcoap_get_blockwise(
        "[::1]:5680", "/const/song.txt"); // induce `Timeout`, not 5683

    let mut bs = get_blockwise_timeout()?;
    while let Some(req) = bs.next().await {
        match req.await {
            GcoapMemoState::Timeout => bs.close(),
            _ => panic!(),
        };
    }

    //

    println!("!! debug NEW [blockwise-resp-none]");
    let get_blockwise_resp_none = || Xbd::async_gcoap_get_blockwise(
        "[::1]:5683", "/const/song2.txt"); // induce `Resp(None)`

    let mut bs = get_blockwise_resp_none()?;
    while let Some(req) = bs.next().await {
        match req.await {
            GcoapMemoState::Resp(None) => bs.close(),
            _ => panic!(),
        };
    }

    //

    Ok(())
}