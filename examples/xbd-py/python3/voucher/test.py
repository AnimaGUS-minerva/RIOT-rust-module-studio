import voucher
from voucher import *  # Vrq, Vch, ATTR_*, ...

if 1:  # debug
    print('@@ dir(voucher):', dir(voucher))
    #print('@@ dir(voucher.voucher):', dir(voucher.voucher))
    print('@@ ATTR_NONCE:', ATTR_NONCE)
    #exit()


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
    print('==== test_voucher_version: ^^')
    print('voucher.version:', voucher.version)
    test_assert('voucher.version', voucher.voucher.version.startswith('Rust voucher '))

def wip_python3():
    print('@@ ======== WIP ========')

    # voucher.vrq()
    # help(vrq)
    # vch = voucher.vch()
    # help(vch)
    #==== !!!! ok
    vrq = Vrq()
    #help(vrq)
    #vrq.debug_dump()

    vch = Vch()
    #help(vch)
    #vch.debug_dump()
    #====

    # ok
    vrq.set(ATTR_ASSERTION, ASSERTION_PROXIMITY) \
       .set(ATTR_CREATED_ON, 1599086034) \
       .set(ATTR_SERIAL_NUMBER, '00-D0-E5-F2-00-02') \
       .set(ATTR_NONCE, b'\x11\x22\x33') \
       .set(ATTR_DOMAIN_CERT_REVOCATION_CHECKS, True) \
       .debug_dump()

    # https://animagus-minerva.github.io/voucher/doc/minerva_voucher/index.html#2-encoding-a-voucher-into-cbor
    vrq = Vrq()
    vrq.set(ATTR_ASSERTION, ASSERTION_PROXIMITY) \
       .set(ATTR_CREATED_ON, 1599086034) \
       .set(ATTR_NONCE, bytes([48, 130, 1, 216, 48, 130, 1, 94, 160, 3, 2, 1, 2, 2, 1, 1, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 48, 115, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 66, 48, 64, 6, 3, 85, 4, 3, 12, 57, 35, 60, 83, 121, 115, 116, 101, 109, 86, 97, 114, 105, 97, 98, 108, 101, 58, 48, 120, 48, 48, 48, 48, 53, 53, 98, 56, 50, 53, 48, 99, 48, 100, 98, 56, 62, 32, 85, 110, 115, 116, 114, 117, 110, 103, 32, 70, 111, 117, 110, 116, 97, 105, 110, 32, 67, 65, 48, 30, 23, 13, 50, 48, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 23, 13, 50, 50, 48, 56, 50, 57, 48, 52, 48, 48, 49, 54, 90, 48, 70, 49, 18, 48, 16, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 2, 99, 97, 49, 25, 48, 23, 6, 10, 9, 146, 38, 137, 147, 242, 44, 100, 1, 25, 22, 9, 115, 97, 110, 100, 101, 108, 109, 97, 110, 49, 21, 48, 19, 6, 3, 85, 4, 3, 12, 12, 85, 110, 115, 116, 114, 117, 110, 103, 32, 74, 82, 67, 48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1, 7, 3, 66, 0, 4, 150, 101, 80, 114, 52, 186, 159, 229, 221, 230, 95, 246, 240, 129, 111, 233, 72, 158, 129, 12, 18, 7, 59, 70, 143, 151, 100, 43, 99, 0, 141, 2, 15, 87, 201, 124, 148, 127, 132, 140, 178, 14, 97, 214, 201, 136, 141, 21, 180, 66, 31, 215, 242, 106, 183, 228, 206, 5, 248, 167, 76, 211, 139, 58, 163, 16, 48, 14, 48, 12, 6, 3, 85, 29, 19, 1, 1, 255, 4, 2, 48, 0, 48, 10, 6, 8, 42, 134, 72, 206, 61, 4, 3, 2, 3, 104, 0, 48, 101, 2, 49, 0, 135, 158, 205, 227, 138, 5, 18, 46, 182, 247, 44, 178, 27, 195, 210, 92, 190, 230, 87, 55, 112, 86, 156, 236, 35, 12, 164, 140, 57, 241, 64, 77, 114, 212, 215, 85, 5, 155, 128, 130, 2, 14, 212, 29, 79, 17, 159, 231, 2, 48, 60, 20, 216, 138, 10, 252, 64, 71, 207, 31, 135, 184, 115, 193, 106, 40, 191, 184, 60, 15, 136, 67, 77, 157, 243, 247, 168, 110, 45, 198, 189, 136, 149, 68, 47, 32, 55, 237, 204, 228, 133, 91, 17, 218, 154, 25, 228, 232])) \
       .set(ATTR_PROXIMITY_REGISTRAR_CERT, bytes([102, 114, 118, 85, 105, 90, 104, 89, 56, 80, 110, 86, 108, 82, 75, 67, 73, 83, 51, 113, 77, 81])) \
       .set(ATTR_SERIAL_NUMBER, '00-D0-E5-F2-00-02')

    KEY_PEM_F2_00_02 = voucher.voucher.debug_get_key_pem_F2_00_02()  # debug, privkey
    test_assert_eq('debug_get_key_pem_F2_00_02', len(KEY_PEM_F2_00_02), 227)

    DEVICE_CRT_F2_00_02 = voucher.voucher.debug_get_device_crt_F2_00_02()  # debug, pubkey
    test_assert_eq('debug_get_device_crt_F2_00_02', len(DEVICE_CRT_F2_00_02), 644)

    voucher.init_psa_crypto()

    test_assert('vrq.validate(DEVICE_CRT_F2_00_02) - with pubkey PEM, should fail (unsigned)',
        not vrq.validate(DEVICE_CRT_F2_00_02))
    test_assert('vrq.validate(KEY_PEM_F2_00_02) - with privkey PEM, should fail (unsigned)',
        not vrq.validate(KEY_PEM_F2_00_02))

    vrq.sign(KEY_PEM_F2_00_02, SA_ES256)#.debug_dump()

    test_assert('vrq.validate(DEVICE_CRT_F2_00_02) - with pubkey PEM',
        vrq.validate(DEVICE_CRT_F2_00_02))
    test_assert('vrq.validate(KEY_PEM_F2_00_02) - with privkey PEM',
        vrq.validate(KEY_PEM_F2_00_02))



def test_voucher_xx():
    print('==== test_voucher_xx(): ^^')

    # init_psa_crypto()  # TODO

    wip_python3()


if 1:
    test_voucher_mbedtls_version()
    test_voucher_version()
    test_voucher_xx()