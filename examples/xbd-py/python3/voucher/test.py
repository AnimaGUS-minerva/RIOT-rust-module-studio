import voucher.mbedtls.version as mbedtls_version
import voucher.version as version

def test_voucher_mbedtls_version():
    print('==== test_voucher_mbedtls_version(): ^^')
    #print(dir(mbedtls_version))
    print('mbedtls_version.version:', mbedtls_version.version)  # 'mbed TLS 3.0.0'

def test_voucher_version():
    print('==== test_voucher_version(): ^^')
    print('version.version:', version.version)  # 'rust voucher x.x.x'

def test_voucher_xx():
    print('==== test_voucher_xx(): ^^')

if 1:
    test_voucher_mbedtls_version()
    test_voucher_version()
    test_voucher_xx()
