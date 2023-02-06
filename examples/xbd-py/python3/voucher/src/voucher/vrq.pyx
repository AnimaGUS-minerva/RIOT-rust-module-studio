# SPDX-License-Identifier: MIT
# Copyright (c) 2023, ANIMA Minerva toolkit

"""The voucher request class"""

# from libc.stdlib cimport malloc, free
# from libc.string cimport strcpy
from . cimport vrq as _vrq

cdef __test():
    cdef vi_provider_t *provider = NULL
    _vrq.vi_provider_allocate(&provider, True)

    _vrq.vi_provider_dump(provider)

    print('provider != NULL:', provider != NULL)
    _vrq.vi_provider_free(&provider)
    print('provider != NULL:', provider != NULL)

    return '!!!! done'

test = __test()
