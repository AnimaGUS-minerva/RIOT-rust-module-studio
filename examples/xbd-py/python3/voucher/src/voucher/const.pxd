# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Declarations from `voucher_if.h`."""


from libc.stdint cimport uint8_t


cdef extern from "voucher_if.h" nogil:
    uint8_t ATTR_ASSERTION
    uint8_t ATTR_CREATED_ON
    uint8_t ATTR_DOMAIN_CERT_REVOCATION_CHECKS
    uint8_t ATTR_EXPIRES_ON
    uint8_t ATTR_IDEVID_ISSUER
    uint8_t ATTR_LAST_RENEWAL_DATE
    uint8_t ATTR_NONCE
    uint8_t ATTR_PINNED_DOMAIN_CERT
    uint8_t ATTR_PINNED_DOMAIN_PUBK
    uint8_t ATTR_PINNED_DOMAIN_PUBK_SHA256
    uint8_t ATTR_PRIOR_SIGNED_VOUCHER_REQUEST
    uint8_t ATTR_PROXIMITY_REGISTRAR_CERT
    uint8_t ATTR_PROXIMITY_REGISTRAR_PUBK
    uint8_t ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256
    uint8_t ATTR_SERIAL_NUMBER

    uint8_t ASSERTION_VERIFIED
    uint8_t ASSERTION_LOGGED
    uint8_t ASSERTION_PROXIMITY

    uint8_t SA_ES256
    uint8_t SA_ES384
    uint8_t SA_ES512
    uint8_t SA_PS256
