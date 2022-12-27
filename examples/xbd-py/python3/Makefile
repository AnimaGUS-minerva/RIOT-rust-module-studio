all: dist

PYTHON := LD_LIBRARY_PATH=../local/lib:$(LD_LIBRARY_PATH) python3
test: dist
	cd ./dist && \
		$(PYTHON) -c "from voucher import version; print((version, version.version))"
	cd ./dist && $(PYTHON) < ../test.py

local:
	git clone https://github.com/Mbed-TLS/mbedtls
	cd ./mbedtls && git checkout -q ae466e78f
	mkdir -p local/src && rsync -az ./mbedtls/ local/src
	./scripts/install-mbedtls.sh local/src local
voucher_if:
	cd .. && cargo build --lib --release
	mkdir -p local/lib && cp ../target/release/libvoucher_if.a local/lib/
dist: local voucher_if
	ls -lrt local/include local/lib
	./scripts/build.sh
	cd ./dist && \
		rm -rf voucher python_voucher-*info && \
		unzip python_voucher-*.whl && \
		ls -lrt
clean:
	rm -rf dist
purge:
	rm -rf dist build local ./src/*.egg-info
