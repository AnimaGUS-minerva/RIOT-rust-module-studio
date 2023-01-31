all: dist

PYTHON := LD_LIBRARY_PATH=../local/lib:$(LD_LIBRARY_PATH) python3
test: dist
	cd ./dist && \
		$(PYTHON) -c "from voucher import version; print((version, version.version))"
	cd ./dist && $(PYTHON) < ../test.py

local:
	cd ../.. && cargo build --lib --release
	mkdir -p local
	mkdir -p local/lib && cp ../../target/release/libvoucher_if.a local/lib/
	mkdir -p local/include && cp -r ../../include/mbedtls local/include/

# exports for 'setup.py'
export C_INCLUDE_PATH := local/include
export LIBRARY_PATH := local/lib
export LD_LIBRARY_PATH := local/lib
export DYLD_LIBRARY_PATH := local/lib
export SETUP_MODULE_NAME := voucher
export SETUP_EXTENSION_LIBS := voucher_if

dist: local
	ls -lrt local/include local/lib
	python3 ../scripts/setup.py bdist_wheel
	cd ./dist && \
		rm -rf voucher python_voucher-*info && \
		unzip python_voucher-*.whl && \
		ls -lrt
clean:
	rm -rf dist
purge:
	rm -rf dist build local ./src/*.egg-info
