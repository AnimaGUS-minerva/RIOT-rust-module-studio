# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Declarations from `voucher_if.h`."""


from libc.stdint cimport uint8_t


cdef extern from "voucher_if.h" nogil:
    uint8_t ATTR_ASSERTION
    uint8_t ATTR_NONCE
