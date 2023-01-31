# SPDX-License-Identifier: MIT
# Copyright (c) 2019, Mathias Laurin
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Run-time version information"""

from libc.stdlib cimport malloc, free
#from libc.string cimport strcpy

from . cimport version as _ver

# def __version():
#     return 'rust voucher x.x.x'
#==== !!!!
cdef __version():
    """Return the version as a string."""
    #==== !!!! wip
    # cdef char *output = <char *>malloc(18 * sizeof(char))
    # cdef bytes buffer
    # if not output:
    #     raise MemoryError()
    # try:
    #     _ver.voucher_version_get_string_full(output)
    #     buffer = output
    #     return buffer.decode("ascii")
    # finally:
    #     free(output)
    #==== !!!! wip
    cdef int version = _ver.vi_square(2)
    return "aaa" if version == 4 else "bbb"



version = __version()
