from voucher.pk import ECC

def test_mbedtls_pk():
    print('==== test_mbedtls_pk(): ^^')

    ecdsa = ECC()
    _prv = ecdsa.generate()
    sig = ecdsa.sign(b"Please sign here.")
    print('sig:', sig)

def test_voucher_xx():
    print('==== test_voucher_xx(): ^^')


if 1:
    test_mbedtls_pk()
    test_voucher_xx()
