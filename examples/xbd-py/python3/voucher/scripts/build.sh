#!/bin/sh

set -e

libdir="local"

C_INCLUDE_PATH="$libdir/include"
LIBRARY_PATH="$libdir/lib"
LD_LIBRARY_PATH=$LIBRARY_PATH
DYLD_LIBRARY_PATH=$LIBRARY_PATH

export C_INCLUDE_PATH
export LIBRARY_PATH
export LD_LIBRARY_PATH
export DYLD_LIBRARY_PATH
## TODO mbedtls2 ##EXTENSION_LIBS=mbedcrypto:mbedtls:mbedx509  python3 setup.py bdist_wheel
EXTENSION_LIBS=voucher_if  python3 setup.py bdist_wheel
