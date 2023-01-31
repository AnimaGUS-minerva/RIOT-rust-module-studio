all: dist

PYTHON := LD_LIBRARY_PATH=../local/lib:$(LD_LIBRARY_PATH) python3
test: dist
	cd ./dist && \
		$(PYTHON) -c "from mbedtls import version; print((version, version.version))"
	cd ./dist && $(PYTHON) < ../test.py

local:
	git clone https://github.com/Mbed-TLS/mbedtls
	cd ./mbedtls && git checkout -q ae466e78f
	mkdir -p local/src && rsync -az ./mbedtls/ local/src
	../scripts/install-mbedtls.sh local/src local

# exports for 'setup.py'
export C_INCLUDE_PATH := local/include
export LIBRARY_PATH := local/lib
export LD_LIBRARY_PATH := local/lib
export DYLD_LIBRARY_PATH := local/lib
export SETUP_MODULE_NAME := mbedtls
export SETUP_EXTENSION_LIBS := mbedcrypto:mbedtls:mbedx509

dist: local
	ls -lrt local/include local/lib
	python3 ../scripts/setup.py bdist_wheel
	cd ./dist && \
		rm -rf mbedtls python_mbedtls-*info && \
		unzip python_mbedtls-*.whl && \
		ls -lrt
clean:
	rm -rf dist
purge:
	rm -rf dist build local ./src/*.egg-info
