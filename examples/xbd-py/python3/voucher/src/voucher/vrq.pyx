# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""The voucher request class"""

# from libc.stdlib cimport malloc, free
# from libc.string cimport strcpy
from . cimport vrq as _vrq


cdef class Vrq:

    def __init__(self):
        pass

    def __cinit__(self):
        print('[__cinit__] self.provider_ptr == NULL:', self.provider_ptr == NULL)  # !!!! True
        _vrq.vi_provider_allocate(&self.provider_ptr, True)
        print('[__cinit__] self.provider_ptr == NULL:', self.provider_ptr == NULL)  # !!!! False

    def __dealloc__(self):
        print('[__dealloc__] self.provider_ptr == NULL:', self.provider_ptr == NULL)  # !!!! False
        _vrq.vi_provider_free(&self.provider_ptr)
        print('[__dealloc__] self.provider_ptr == NULL:', self.provider_ptr == NULL)  # !!!! True

    def debug_dump(self):
        _vrq.vi_provider_dump(self.provider_ptr)


cdef __test():
    cdef vi_provider_t *provider = NULL
    _vrq.vi_provider_allocate(&provider, True)

    _vrq.vi_provider_dump(provider)

    print('provider != NULL:', provider != NULL)
    _vrq.vi_provider_free(&provider)
    print('provider != NULL:', provider != NULL)

    return '!!!! done'

test = __test()
