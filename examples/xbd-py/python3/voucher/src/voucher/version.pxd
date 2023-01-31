# SPDX-License-Identifier: MIT
# Copyright (c) 2019, Mathias Laurin
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Declarations from `voucher_if.h`."""


cdef extern from "voucher_if.h" nogil:
    #==== !!!! wip
    # void voucher_version_get_string_full(char *string)
    int vi_square(int)