# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""The Voucher library."""

# from libc.stdlib cimport malloc, free
# from libc.string cimport strcpy
from . cimport voucher as _vou
from . cimport const as _const

ATTR_ASSERTION = _const.ATTR_ASSERTION
ATTR_NONCE = _const.ATTR_NONCE

cdef class Vou:

    def __dealloc__(self):
        _vou.vi_provider_free(&self.provider_ptr)

    def debug_dump(self):
        _vou.vi_provider_dump(self.provider_ptr)

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


cdef class Vrq(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, True)


cdef class Vch(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, False)
