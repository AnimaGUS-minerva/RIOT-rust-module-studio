# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""The Voucher library."""

from libc.stdlib cimport malloc, free
#from libc.string cimport strcpy
from . cimport voucher as _vou
from . cimport const as _const

ATTR_ASSERTION                        = _const.ATTR_ASSERTION
ATTR_CREATED_ON                       = _const.ATTR_CREATED_ON
ATTR_DOMAIN_CERT_REVOCATION_CHECKS    = _const.ATTR_DOMAIN_CERT_REVOCATION_CHECKS
ATTR_EXPIRES_ON                       = _const.ATTR_EXPIRES_ON
ATTR_IDEVID_ISSUER                    = _const.ATTR_IDEVID_ISSUER
ATTR_LAST_RENEWAL_DATE                = _const.ATTR_LAST_RENEWAL_DATE
ATTR_NONCE                            = _const.ATTR_NONCE
ATTR_PINNED_DOMAIN_CERT               = _const.ATTR_PINNED_DOMAIN_CERT
ATTR_PINNED_DOMAIN_PUBK               = _const.ATTR_PINNED_DOMAIN_PUBK
ATTR_PINNED_DOMAIN_PUBK_SHA256        = _const.ATTR_PINNED_DOMAIN_PUBK_SHA256
ATTR_PRIOR_SIGNED_VOUCHER_REQUEST     = _const.ATTR_PRIOR_SIGNED_VOUCHER_REQUEST
ATTR_PROXIMITY_REGISTRAR_CERT         = _const.ATTR_PROXIMITY_REGISTRAR_CERT
ATTR_PROXIMITY_REGISTRAR_PUBK         = _const.ATTR_PROXIMITY_REGISTRAR_PUBK
ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256  = _const.ATTR_PROXIMITY_REGISTRAR_PUBK_SHA256
ATTR_SERIAL_NUMBER                    = _const.ATTR_SERIAL_NUMBER

ASSERTION_VERIFIED  = _const.ASSERTION_VERIFIED
ASSERTION_LOGGED    = _const.ASSERTION_LOGGED
ASSERTION_PROXIMITY = _const.ASSERTION_PROXIMITY

SA_ES256 = _const.SA_ES256
SA_ES384 = _const.SA_ES384
SA_ES512 = _const.SA_ES512
SA_PS256 = _const.SA_PS256


cdef class Vou:
    UINTPTR_NULL = <uintptr_t>NULL

    def __dealloc__(self):
        _vou.vi_provider_free(&self.provider_ptr)

    def init_provider_ptr(self, uintptr_t ptr, is_vrq):
        if ptr == Vou.UINTPTR_NULL:
            _vou.vi_provider_allocate(&self.provider_ptr, is_vrq)
        else:
            self.provider_ptr = <vi_provider_t *>ptr

    def debug_dump(self):
        _vou.vi_provider_dump(self.provider_ptr)
        return self

    def to_cbor(self):
        cdef uint8_t *buf
        sz = _vou.vi_provider_to_cbor(self.provider_ptr, &buf)
        if buf == NULL:
            raise ValueError("'to_cbor' operation failed")

        cbor = buf[:sz]
        free(buf)
        return cbor

    def len(self):
        return _vou.vi_provider_len(self.provider_ptr)

    def set(self, key, val):
        ptr = self.provider_ptr
        result = None

        if isinstance(val, bool):  # Yang::Boolean
            result = _vou.vi_provider_set_attr_bool(ptr, key, val)
        elif isinstance(val, int):  # Yang::{Enumeration,DateAndTime}
            result = _vou.vi_provider_set_attr_int(ptr, key, val)
        elif isinstance(val, str):  # Yang::String
            result = _vou.vi_provider_set_attr_bytes(ptr, key, val.encode(), len(val))
        elif isinstance(val, bytes):  # Yang::Binary
            result = _vou.vi_provider_set_attr_bytes(ptr, key, val, len(val))
        else:
            raise ValueError(f"invalid 'val' type ({type(val)})")

        if not result:
            raise ValueError(f"'set' operation failed for attr key ({key})")

        return self

    def get(self, key):
        ptr = self.provider_ptr
        cdef uint8_t *buf = NULL
        obj = None

        if _vou.vi_provider_has_attr_int(ptr, key):
            obj = _vou.vi_provider_get_attr_int_or_panic(ptr, key)
        elif _vou.vi_provider_has_attr_bool(ptr, key):
            obj = vi_provider_get_attr_bool_or_panic(ptr, key)
        elif _vou.vi_provider_has_attr_bytes(ptr, key):
            sz = _vou.vi_provider_get_attr_bytes_or_panic(ptr, key, &buf)
            obj = b'' if buf == NULL else buf[:sz]

        return obj

    def sign(self, key_pem, alg):
        ptr = self.provider_ptr

        if not isinstance(key_pem, bytes):
            raise ValueError("'pem' arg must be <class 'bytes'>")

        if not _vou.vi_provider_sign(ptr, key_pem, len(key_pem), alg):
            raise ValueError(f"'sign' operation failed for alg({alg})")

        return self

    def validate(self, pem=None):
        ptr = self.provider_ptr

        if pem is None:  # without PEM (`signer_cert` is used instead)
            return _vou.vi_provider_validate(ptr);
        elif isinstance(pem, bytes):
            return _vou.vi_provider_validate_with_pem(ptr, pem, len(pem))
        else:
            raise ValueError("'pem' arg must be <class 'bytes'>")


cdef class Vrq(Vou):
    def __cinit__(self, uintptr_t ptr=Vou.UINTPTR_NULL):
        self.init_provider_ptr(ptr, True)


cdef class Vch(Vou):
    def __cinit__(self, uintptr_t ptr=Vou.UINTPTR_NULL):
        self.init_provider_ptr(ptr, False)


cdef __from_cbor(cbor):
    cdef vi_provider_t *provider_ptr = NULL
    if not isinstance(cbor, bytes):
        raise ValueError("'cbor' arg must be <class 'bytes'>")

    if not _vou.vi_provider_allocate_from_cbor(&provider_ptr, cbor, len(cbor)):
        raise ValueError("bad cbor voucher")

    ptr = <uintptr_t>provider_ptr
    return Vrq(ptr) if _vou.vi_provider_is_vrq(provider_ptr) else Vch(ptr)


cdef __version():
    sz = 32 * sizeof(uint8_t)
    cdef uint8_t *output = <uint8_t *>malloc(sz)
    cdef bytes buffer
    if not output:
        raise MemoryError()
    try:
        _vou.voucher_version_get_string_full(output, sz)
        buffer = output
        return buffer.decode("ascii")
    finally:
        free(output)


ctypedef size_t (*f_type)(uint8_t **pp)

cdef bytes __debug_f_static(f_type f):
    cdef uint8_t *ptr_static
    sz = f(&ptr_static)
    return ptr_static[:sz]

cdef __debug_get_vch_jada():
    return __debug_f_static(_vou.vi_get_voucher_jada)

cdef __debug_get_vch_F2_00_02():
    return __debug_f_static(_vou.vi_get_voucher_F2_00_02)

cdef __debug_get_masa_pem_F2_00_02():
    return __debug_f_static(_vou.vi_get_masa_pem_F2_00_02)

cdef __debug_get_key_pem_F2_00_02():
    return __debug_f_static(_vou.vi_get_key_pem_F2_00_02)

cdef __debug_get_device_crt_F2_00_02():
    return __debug_f_static(_vou.vi_get_device_crt_F2_00_02)

cdef __debug_get_vrq_F2_00_02():
    return __debug_f_static(_vou.vi_get_vrq_F2_00_02)


from_cbor = __from_cbor
version = __version()
init_psa_crypto = _vou.vi_init_psa_crypto
debug_get_vch_jada = __debug_get_vch_jada
debug_get_vch_F2_00_02 = __debug_get_vch_F2_00_02
debug_get_masa_pem_F2_00_02 = __debug_get_masa_pem_F2_00_02
debug_get_key_pem_F2_00_02 = __debug_get_key_pem_F2_00_02
debug_get_device_crt_F2_00_02 = __debug_get_device_crt_F2_00_02
debug_get_vrq_F2_00_02 = __debug_get_vrq_F2_00_02