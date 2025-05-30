# Copyright (C) 2023 Gramine contributors
# SPDX-License-Identifier: BSD-3-Clause

ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

SRCDIR = src
MEMCACHED_SRC ?= memcached-1.6.21.tar.gz
MEMCACHED_MIRRORS ?= \
    https://memcached.org/files \
    https://packages.gramineproject.io/distfiles

MEMCACHED_SHA256 ?= c788980efc417dd5d93c442b1c8b8769fb2018896c29de3887d22a2f143da2ee

ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = debug
else
GRAMINE_LOG_LEVEL = error
endif

.PHONY: all
all: memcached memcached.manifest
ifeq ($(SGX),1)
all: memcached.manifest.sgx memcached.sig
endif

$(SRCDIR)/configure:
	../common_tools/download --output memcached.tar.gz --sha256 $(MEMCACHED_SHA256) \
		$(foreach mirror,$(MEMCACHED_MIRRORS),--url $(mirror)/$(MEMCACHED_SRC))
	mkdir $(SRCDIR)
	tar -C $(SRCDIR) --strip-components=1 -xf memcached.tar.gz

$(SRCDIR)/memcached: $(SRCDIR)/configure
	cd $(SRCDIR) && ./configure
	$(MAKE) -C $(SRCDIR)

memcached.manifest: memcached.manifest.template memcached
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

memcached.manifest.sgx memcached.sig &: memcached.manifest
	gramine-sgx-sign \
		--manifest $< \
		--output $<.sgx

# for simplicity, copy memcached executable into our root directory
memcached: $(SRCDIR)/memcached
	cp $< $@

.PHONY: start-native-server
start-native-server: all
	./memcached

ifeq ($(SGX),)
GRAMINE = gramine-direct
else
GRAMINE = gramine-sgx
endif

.PHONY: start-gramine-server
start-gramine-server: all
	$(GRAMINE) memcached

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest memcached .lck

.PHONY: distclean
distclean: clean
	$(RM) -r $(SRCDIR) memcached.tar.gz
