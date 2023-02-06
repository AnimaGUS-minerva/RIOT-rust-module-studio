# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Declarations from `voucher_if.h`."""

from libcpp cimport bool
from libc.stdint cimport uint8_t

cdef extern from "voucher_if.h" nogil:
    ctypedef struct vi_provider_t:
        pass

    void vi_provider_allocate(vi_provider_t **pp, bool is_vrq);
    bool vi_provider_allocate_from_cbor(vi_provider_t **pp, const uint8_t *buf, size_t sz);
    void vi_provider_free(vi_provider_t **pp);

    void vi_provider_dump(vi_provider_t *p);
