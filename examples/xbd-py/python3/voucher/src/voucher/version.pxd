# SPDX-License-Identifier: MIT
# Copyright (c) 2019, Mathias Laurin
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Declarations from `python3_if.h`."""

cdef extern from "python3_if.h" nogil:
    void voucher_version_get_string_full(char *ptr, size_t sz)
