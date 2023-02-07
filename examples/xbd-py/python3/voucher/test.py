import voucher
####from voucher import *  # for `{ATTR,SA}_*` constants (?? , `debug_*()` methods)

print('@@ dir(voucher):', dir(voucher))


#### #### #### #### TODO refactor w.r.t. 'ports/riot/main/boot.py'
def test_assert_eq(title, left, right, diag=True):
    result = left == right
    print('[test]', title, ':', '✅' if result else '❌')
    if diag and not result:
        print('test failed with')
        print('  left:', left)
        print('  right:', right)

def test_assert(title, condition):
    test_assert_eq(title, condition, True, diag=False)
#### #### #### ####


def test_voucher_mbedtls_version():
    import voucher.mbedtls.version as mbedtls_version

    print('==== test_voucher_mbedtls_version(): ^^')
    print('mbedtls_version.version:', mbedtls_version.version)
    test_assert('mbedtls_version.version', mbedtls_version.version.startswith('mbed TLS 3.'))

def test_voucher_version():
    import voucher.version as version

    print('==== test_voucher_version(): ^^')
    print('version.version:', version.version)
    test_assert('version.version', version.version.startswith('Rust voucher '))

def wip_python3():
    print('@@ ======== WIP ========')

    # import mbedtls
    # from mbedtls.pk import ECC
    #
    #
    # def test_pk():
    #     print('==== test_pk(): ^^')
    #
    #     ecdsa = ECC()
    #     _prv = ecdsa.generate()
    #     sig = ecdsa.sign(b"Please sign here.")
    #     print('sig:', sig)
    #==== !!!!
    # ....

    # voucher.vrq()
    # help(vrq)
    # vch = voucher.vch()
    # help(vch)
    #==== !!!!
    from voucher.voucher import Vrq
    from voucher.voucher import Vch

    vrq = Vrq()
    #help(vrq)
    vrq.debug_dump()

    vch = Vch()
    #help(vch)
    vch.debug_dump()
    #====

    # vrq.set(ATTR_ASSERTION, ASSERTION_PROXIMITY) \
    #    .set(ATTR_CREATED_ON, 1599086034) \
    #    .set(ATTR_SERIAL_NUMBER, '00-D0-E5-F2-00-02') \
    #    .set(ATTR_NONCE, b'\x11\x22\x33') \
    #    .set(ATTR_DOMAIN_CERT_REVOCATION_CHECKS, True) \
    #    .debug_dump()

def test_voucher_xx():
    print('==== test_voucher_xx(): ^^')

    # init_psa_crypto()  # TODO

    wip_python3()


if 1:
    test_voucher_mbedtls_version()
    test_voucher_version()
    test_voucher_xx()