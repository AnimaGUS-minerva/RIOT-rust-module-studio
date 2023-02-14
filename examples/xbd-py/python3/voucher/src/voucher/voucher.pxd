# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

from libcpp cimport bool as bool_t
from libc.stdint cimport uint8_t
from libc.stdint cimport uint64_t


cdef extern from "python3_if.h" nogil:
    void voucher_version_get_string_full(uint8_t *ptr, size_t sz)


cdef extern from "voucher_if.h" nogil:

    # void vi_init_psa_crypto(void);
    #
    # // `*pp` points to a static address after calling
    # size_t vi_get_voucher_jada(uint8_t **pp);
    # size_t vi_get_voucher_F2_00_02(uint8_t **pp);
    # size_t vi_get_masa_pem_F2_00_02(uint8_t **pp);
    size_t vi_get_key_pem_F2_00_02(uint8_t **pp);
    # size_t vi_get_device_crt_F2_00_02(uint8_t **pp);
    # size_t vi_get_vrq_F2_00_02(uint8_t **pp);
    #
    # // `*pp` points to a heap address after calling
    # size_t vi_create_vrq_F2_00_02(uint8_t **pp);
    #
    # size_t vi_sign(const uint8_t *ptr_raw, size_t sz_raw, const uint8_t *ptr_key, size_t sz_key,
    #                uint8_t **pp, uint8_t alg);
    # bool vi_validate(const uint8_t *ptr, size_t sz);
    # bool vi_validate_with_pem(const uint8_t *ptr, size_t sz, const uint8_t *ptr_pem, size_t sz_pem);
    # void vi_dump(const uint8_t *ptr, size_t sz);

    ctypedef struct vi_provider_t:
        pass

    void vi_provider_allocate(vi_provider_t **pp, bool_t is_vrq);
    bool_t vi_provider_allocate_from_cbor(vi_provider_t **pp, const uint8_t *buf, size_t sz);
    void vi_provider_free(vi_provider_t **pp);

    void vi_provider_dump(vi_provider_t *p);

    bool_t vi_provider_set_attr_int(vi_provider_t *p, uint8_t attr_key, uint64_t attr_val);
    bool_t vi_provider_set_attr_bool(vi_provider_t *p, uint8_t attr_key, bool_t attr_val);
    bool_t vi_provider_set_attr_bytes(vi_provider_t *p, uint8_t attr_key, const uint8_t *buf, size_t sz);


cdef class Vou:
    cdef vi_provider_t *provider_ptr


cdef class Vrq(Vou):
    pass


cdef class Vch(Vou):
    pass
