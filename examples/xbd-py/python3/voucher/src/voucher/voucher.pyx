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

    def set(self, attr_key, attr_val):
        result = None

        if isinstance(attr_val, bool):  # Yang::Boolean
            result = _vou.vi_provider_set_attr_bool(self.provider_ptr, attr_key, attr_val)
        elif isinstance(attr_val, int):  # Yang::{Enumeration,DateAndTime}
            result = _vou.vi_provider_set_attr_int(self.provider_ptr, attr_key, attr_val)
        elif isinstance(attr_val, str):  # Yang::String
            print('@@ !!!! WIP `<bytes>attr_val`:', <bytes>attr_val, len(attr_val))

            result = _vou.vi_provider_set_attr_bytes(# FIXME---vv
                self.provider_ptr, attr_key, <uint8_t *><bytes>attr_val, len(attr_val))
        elif isinstance(attr_val, bytes):  # Yang::Binary
            result = _vou.vi_provider_set_attr_bytes(
                self.provider_ptr, attr_key, <uint8_t *>attr_val, len(attr_val))
        else:
            raise ValueError("invalid 'attr_val' type")

        if not result:
            raise ValueError(f"'set' operation failed for attr key({attr_key})")

        return self


cdef class Vrq(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, True)


cdef class Vch(Vou):

    def __cinit__(self):
        _vou.vi_provider_allocate(&self.provider_ptr, False)
