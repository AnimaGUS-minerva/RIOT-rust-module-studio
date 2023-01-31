import ctypes
import ctypes.util
import os
import platform
import re
import sys
from contextlib import suppress

from setuptools import Extension, find_packages, setup

WINDOWS = platform.system() == "Windows"

def from_env(var):
    ENVSEP = ";" if WINDOWS else ":"

    with suppress(KeyError):
        return list(filter(None, os.environ[var].split(ENVSEP)))
    return ()

MODULE_NAME = from_env("SETUP_MODULE_NAME")[0]

def _get_version():
    pattern = re.compile(r'^__version__ = ["]([.\w]+?)["]')
    with open(
        os.path.join(
            "src", MODULE_NAME, "__init__.py"
        )
    ) as f:
        for line in f:
            match = pattern.match(line)
            if match:
                return match.group(1)
        raise RuntimeError()


VERSION = _get_version()


if "--with-coverage" in sys.argv:
    sys.argv.remove("--with-coverage")
    COVERAGE = True
else:
    COVERAGE = False


setup_requires = [
    # Setuptools 18.0 properly handles Cython extensions.
    "setuptools >= 18.0",
    # Cython 0.28 handles const memoryviews.
    "cython >= 0.28.0",
]
install_requires = [
    "certifi",
    'contextlib2; python_version < "3.0"',
    'enum34 != 1.1.8; python_version < "3.0"',
    'pathlib2; python_version < "3.0"',
]
tests_require = [
    "readme_renderer",
    'contextlib2; python_version < "3.0"',
]

def extensions(coverage=False):
    libraries = (
        [
            "AdvAPI32",  # `Crypt*` calls from `library/entropy_poll.c`
            "mbedTLS",
        ]
        if WINDOWS
        else from_env("SETUP_EXTENSION_LIBS")
    )
    library_dirs = from_env("LIBPATH" if WINDOWS else "LIBRARY_PATH")

    for dirpath, _, filenames in os.walk("src"):
        for fn in filenames:
            root, ext = os.path.splitext(fn)
            if ext != ".pyx":
                continue
            mod = ".".join(dirpath.split(os.sep)[1:] + [root])
            extension = Extension(
                mod,
                sources=[os.path.join(dirpath, fn)],
                library_dirs=library_dirs,
                libraries=libraries,
                define_macros=[
                    ("CYTHON_TRACE", "1"),
                    ("CYTHON_TRACE_NOGIL", "1"),
                ]
                if coverage
                else [],
            )
            extension.cython_directives = {"language_level": "3str"}
            if coverage:
                extension.cython_directives["linetrace"] = True
            yield extension


def options(coverage=False):
    if coverage:
        return {}

    return {
        "build": {
            "build_base": os.sep.join(
                ("build", "%i.%i.%i" % sys.version_info[:3])
            )
        },
        "build_ext": {"cython_c_in_temp": True},
    }


print('@@ COVERAGE:', COVERAGE)
print('@@ ext_modules:', list(extensions(COVERAGE)))
print('@@ packages:', find_packages("src"))
setup(
    name=f"python-{MODULE_NAME}",
    version=VERSION,
    ext_modules=list(extensions(COVERAGE)),
    #options=options(COVERAGE),
    package_dir={"": "src"},
    packages=find_packages("src"),
    setup_requires=setup_requires,
    install_requires=install_requires,
    tests_require=tests_require,
)
