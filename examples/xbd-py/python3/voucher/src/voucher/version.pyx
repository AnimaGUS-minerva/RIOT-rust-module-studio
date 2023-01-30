# SPDX-License-Identifier: MIT
# Copyright (c) 2019, Mathias Laurin
# Copyright (c) 2023, ANIMA Minerva toolkit

"""Run-time version information"""

from libc.stdlib cimport malloc, free
from libc.string cimport strcpy

#from . cimport version as _ver

#cdef __version():
def __version():
    return 'rust voucher x.x.x'


version = __version()
