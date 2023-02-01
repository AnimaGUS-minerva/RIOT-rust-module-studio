# SPDX-License-Identifier: MIT
# Copyright (c) 2019, Mathias Laurin
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Run-time version information"""

from libc.stdlib cimport malloc, free
#from libc.string cimport strcpy

from . cimport version as _ver

cdef __version():
    """Return the version as a string."""
    sz = 32 * sizeof(char)
    cdef char *output = <char *>malloc(sz)
    cdef bytes buffer
    if not output:
        raise MemoryError()
    try:
        _ver.voucher_version_get_string_full(output, sz)
        buffer = output
        return buffer.decode("ascii")
    finally:
        free(output)


version = __version()
