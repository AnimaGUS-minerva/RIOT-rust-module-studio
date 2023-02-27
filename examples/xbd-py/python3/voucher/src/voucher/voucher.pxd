# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

from libcpp cimport bool as bool_t
from libc.stdint cimport uint8_t, int8_t, uint64_t, uintptr_t


cdef extern from "voucher_if.h" nogil:
    size_t voucher_version_get_string_full(uint8_t **buf)

    void vi_init_psa_crypto()

    ctypedef struct vi_provider_t:
        pass

    void vi_provider_allocate(vi_provider_t **pp, bool_t is_vrq)
    bool_t vi_provider_allocate_from_cbor(vi_provider_t **pp, const uint8_t *buf, size_t sz)
    void vi_provider_free(vi_provider_t **pp)

    bool_t vi_provider_is_vrq(vi_provider_t *p)
    size_t vi_provider_to_cbor(vi_provider_t *p, uint8_t **buf)
    void vi_provider_dump(vi_provider_t *p)
    size_t vi_provider_len(vi_provider_t *p)

    bool_t vi_provider_has_attr_int(vi_provider_t *p, uint8_t attr_key)
    bool_t vi_provider_has_attr_bool(vi_provider_t *p, uint8_t attr_key)
    bool_t vi_provider_has_attr_bytes(vi_provider_t *p, uint8_t attr_key)

    uint64_t vi_provider_get_attr_int_or_panic(vi_provider_t *p, uint8_t attr_key)
    bool_t vi_provider_get_attr_bool_or_panic(vi_provider_t *p, uint8_t attr_key)
    size_t vi_provider_get_attr_bytes_or_panic(vi_provider_t *p, uint8_t attr_key, uint8_t **buf)

    bool_t vi_provider_set_attr_int(vi_provider_t *p, uint8_t attr_key, uint64_t attr_val)
    bool_t vi_provider_set_attr_bool(vi_provider_t *p, uint8_t attr_key, bool_t attr_val)
    bool_t vi_provider_set_attr_bytes(vi_provider_t *p, uint8_t attr_key, const uint8_t *buf, size_t sz)

    bool_t vi_provider_remove_attr(vi_provider_t *p, uint8_t attr_key)
    uint8_t vi_provider_attr_key_at(vi_provider_t *p, size_t n)

    size_t vi_provider_get_signer_cert(vi_provider_t *p, uint8_t **buf)
    void vi_provider_set_signer_cert(vi_provider_t *p, const uint8_t *buf, size_t sz)
    size_t vi_provider_get_content(vi_provider_t *p, uint8_t **buf)
    size_t vi_provider_get_signature_bytes(vi_provider_t *p, uint8_t **buf)
    int8_t vi_provider_get_signature_alg(vi_provider_t *p)

    bool_t vi_provider_sign(vi_provider_t *p, const uint8_t *ptr_key, size_t sz_key, uint8_t alg)
    bool_t vi_provider_validate(vi_provider_t *p)
    bool_t vi_provider_validate_with_pem(vi_provider_t *p, const uint8_t *ptr_pem, size_t sz_pem)

    # `*pp` points to a static address after calling
    size_t vi_get_voucher_jada(uint8_t **pp)
    size_t vi_get_voucher_F2_00_02(uint8_t **pp)
    size_t vi_get_masa_pem_F2_00_02(uint8_t **pp)
    size_t vi_get_key_pem_F2_00_02(uint8_t **pp)
    size_t vi_get_device_crt_F2_00_02(uint8_t **pp)
    size_t vi_get_vrq_F2_00_02(uint8_t **pp)


cdef class Vou:
    cdef vi_provider_t *provider_ptr


cdef class Vrq(Vou):
    pass


cdef class Vch(Vou):
    pass
