# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""python-voucher is a wrapper to the Minerva voucher library."""

from .mbedtls import version as mbedtls_version
from .voucher import Vrq
from .voucher import Vch

ATTR_ASSERTION                        = voucher.ATTR_ASSERTION
ATTR_CREATED_ON                       = voucher.ATTR_CREATED_ON
ATTR_DOMAIN_CERT_REVOCATION_CHECKS    = voucher.ATTR_DOMAIN_CERT_REVOCATION_CHECKS
ATTR_EXPIRES_ON                       = voucher.ATTR_EXPIRES_ON
ATTR_IDEVID_ISSUER                    = voucher.ATTR_IDEVID_ISSUER
ATTR_LAST_RENEWAL_DATE                = voucher.ATTR_LAST_RENEWAL_DATE
ATTR_NONCE                            = voucher.ATTR_NONCE
ATTR_PINNED_DOMAIN_CERT               = voucher.ATTR_PINNED_DOMAIN_CERT
ATTR_PINNED_DOMAIN_PUBK               = voucher.ATTR_PINNED_DOMAIN_PUBK
ATTR_PINNED_DOMAIN_PUBK_SHA256        = voucher.ATTR_PINNED_DOMAIN_PUBK_SHA256
ATTR_PRIOR_SIGNED_VOUCHER_REQUEST     = voucher.ATTR_PRIOR_SIGNED_VOUCHER_REQUEST
ATTR_PROXIMITY_REGISTRAR_CERT         = voucher.ATTR_PROXIMITY_REGISTRAR_CERT
ATTR_PROXIMITY_REGISTRAR_PUBK         = voucher.ATTR_PROXIMITY_REGISTRAR_PUBK
ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256  = voucher.ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256
ATTR_SERIAL_NUMBER                    = voucher.ATTR_SERIAL_NUMBER

ASSERTION_VERIFIED  = voucher.ASSERTION_VERIFIED
ASSERTION_LOGGED    = voucher.ASSERTION_LOGGED
ASSERTION_PROXIMITY = voucher.ASSERTION_PROXIMITY

SA_ES256 = voucher.SA_ES256
SA_ES384 = voucher.SA_ES384
SA_ES512 = voucher.SA_ES512
SA_PS256 = voucher.SA_PS256

__version__ = "0.1.0"

from_cbor = voucher.from_cbor
version = voucher.version
voucher.init_psa_crypto()  # NOP in case already initialized

__all__ = (
    "mbedtls_version",
    "version",
    "Vrq",
    "Vch",
    "ATTR_ASSERTION",
    "ATTR_CREATED_ON",
    "ATTR_DOMAIN_CERT_REVOCATION_CHECKS",
    "ATTR_EXPIRES_ON",
    "ATTR_IDEVID_ISSUER",
    "ATTR_LAST_RENEWAL_DATE",
    "ATTR_NONCE",
    "ATTR_PINNED_DOMAIN_CERT",
    "ATTR_PINNED_DOMAIN_PUBK",
    "ATTR_PINNED_DOMAIN_PUBK_SHA256",
    "ATTR_PRIOR_SIGNED_VOUCHER_REQUEST",
    "ATTR_PROXIMITY_REGISTRAR_CERT",
    "ATTR_PROXIMITY_REGISTRAR_PUBK",
    "ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256",
    "ATTR_SERIAL_NUMBER",
    "ASSERTION_VERIFIED",
    "ASSERTION_LOGGED",
    "ASSERTION_PROXIMITY",
    "SA_ES256",
    "SA_ES384",
    "SA_ES512",
    "SA_PS256",
)
