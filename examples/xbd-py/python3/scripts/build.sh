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
python3 setup.py bdist_wheel
