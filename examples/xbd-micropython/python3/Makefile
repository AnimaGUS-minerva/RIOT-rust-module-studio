all: dist

PYTHON := LD_LIBRARY_PATH=../local/lib:$(LD_LIBRARY_PATH) python3
test: dist
	cd ./dist && \
		$(PYTHON) -c "from voucher import version; print((version, version.version))"
	#WIP#cd ./dist && $(PYTHON) < ../test.py

local:
	git clone https://github.com/Mbed-TLS/mbedtls
	cd ./mbedtls && git checkout -q ae466e78f
	mkdir -p local/src && rsync -az ./mbedtls/ local/src
	./scripts/install-mbedtls.sh local/src local
	ls -lrt local/lib
dist: local
	./scripts/build.sh
	cd ./dist && \
		rm -rf voucher minerva_voucher-*info && \
		unzip minerva_voucher-*.whl && \
		echo KLUDGE_SHIM && mv mbedtls mbedtls-- && ln -s voucher mbedtls && \
		ls -lrt
clean:
	rm -rf dist
purge:
	rm -rf dist build local ./src/*.egg-info
