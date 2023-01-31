import mbedtls
from mbedtls.pk import ECC


def test_pk():
    print('==== test_pk(): ^^')

    ecdsa = ECC()
    _prv = ecdsa.generate()
    sig = ecdsa.sign(b"Please sign here.")
    print('sig:', sig)

def test_version():
    print('==== test_version(): ^^')
    print(dir(mbedtls))
    print('version:', mbedtls.version)

if 1:
    test_pk()
    test_version()
