# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""The Voucher library."""

# from libc.stdlib cimport malloc, free
# from libc.string cimport strcpy
from . cimport voucher as _vou


cdef class Vou:

    def __dealloc__(self):
        _vou.vi_provider_free(&self.provider_ptr)

    def debug_dump(self):
        _vou.vi_provider_dump(self.provider_ptr)


cdef class Vrq(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, True)


cdef class Vch(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, False)
