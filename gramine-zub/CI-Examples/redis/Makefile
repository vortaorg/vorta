# Copyright (C) 2023 Gramine contributors
# SPDX-License-Identifier: BSD-3-Clause

# Build Redis as follows:
#
# - make               -- create non-SGX no-debug-log manifest
# - make SGX=1         -- create SGX no-debug-log manifest
# - make SGX=1 DEBUG=1 -- create SGX debug-log manifest
#
# Any of these invocations clones Redis' git repository and builds Redis in
# default configuration and in the latest-to-date (6.0.5) version.
#
# By default, Redis uses poll/epoll mechanism of Linux. To build Redis with
# select, use `make USE_SELECT=1`. For correct re-builds, always clean up
# Redis source code beforehand via `make distclean`.
#
# Use `make clean` to remove Gramine-generated files and `make distclean` to
# additionally remove the cloned Redis git repository.

################################# CONSTANTS ###################################

# directory with arch-specific libraries, used by Redis
# the below path works for Debian/Ubuntu; for CentOS/RHEL/Fedora, you should
# overwrite this default like this: `ARCH_LIBDIR=/lib64 make`
ARCH_LIBDIR ?= /lib/$(shell $(CC) -dumpmachine)

SRCDIR = src
COMMIT = 6.0.5
TAR_SHA256 = f7ded6c27d48c20bc78e797046c79b6bc411121f0c2d7eead9fea50d6b0b6290

ifeq ($(DEBUG),1)
GRAMINE_LOG_LEVEL = debug
else
GRAMINE_LOG_LEVEL = error
endif

.PHONY: all
all: redis-server redis-server.manifest
ifeq ($(SGX),1)
all: redis-server.manifest.sgx redis-server.sig
endif

############################## REDIS EXECUTABLE ###############################

# Redis is built as usual, without any changes to the build process (except to
# test select syscall instead of poll/epoll). The source is downloaded from the
# GitHub repo (6.0.5 tag) and built via `make`. The result of this build process
# is the final executable "src/redis-server".

$(SRCDIR)/Makefile:
	../common_tools/download --output redis.tar.gz \
		--sha256 $(TAR_SHA256) \
		--url https://github.com/antirez/redis/archive/$(COMMIT).tar.gz \
		--url https://packages.gramineproject.io/distfiles/redis-$(COMMIT).tar.gz
	mkdir $(SRCDIR)
	tar -C $(SRCDIR) --strip-components=1 -xf redis.tar.gz

ifeq ($(USE_SELECT),1)
$(SRCDIR)/src/redis-server: $(SRCDIR)/Makefile
	sed -i 's|#define HAVE_EPOLL 1|/* no HAVE_EPOLL */|g' src/src/config.h
	$(MAKE) -C $(SRCDIR)
else
$(SRCDIR)/src/redis-server: $(SRCDIR)/Makefile
	$(MAKE) -C $(SRCDIR)
endif

################################ REDIS MANIFEST ###############################

# The template file is a Jinja2 template and contains almost all necessary
# information to run Redis under Gramine / Gramine-SGX. We create
# redis-server.manifest (to be run under non-SGX Gramine) by replacing variables
# in the template file using the "gramine-manifest" script.
#
# "gramine-manifest" also measures all Redis trusted files and adds the
# measurements to the resulting manifest file.
redis-server.manifest: redis-server.manifest.template $(SRCDIR)/src/redis-server
	gramine-manifest \
		-Dlog_level=$(GRAMINE_LOG_LEVEL) \
		-Darch_libdir=$(ARCH_LIBDIR) \
		$< > $@

# Manifest for Gramine-SGX requires special "gramine-sgx-sign" procedure. This
# procedure creates the final manifest.sgx file and an SGX-specific
# redis-server.sig file (SIGSTRUCT object).
redis-server.sig redis-server.manifest.sgx &: redis-server.manifest
	gramine-sgx-sign \
		--manifest redis-server.manifest \
		--output redis-server.manifest.sgx

########################### COPIES OF EXECUTABLES #############################

# Redis build process creates the final executable as src/redis-server. For
# simplicity, copy it into our root directory.

redis-server: $(SRCDIR)/src/redis-server
	cp $< $@

############################## RUNNING TESTS ##################################

.PHONY: start-native-server
start-native-server: all
	./redis-server --save '' --protected-mode no

ifeq ($(SGX),)
GRAMINE = gramine-direct
else
GRAMINE = gramine-sgx
endif

# Note that command-line arguments are hardcoded in the manifest file.
.PHONY: start-gramine-server
start-gramine-server: all
	$(GRAMINE) redis-server

################################## CLEANUP ####################################

.PHONY: clean
clean:
	$(RM) *.token *.sig *.manifest.sgx *.manifest redis-server *.rdb

.PHONY: distclean
distclean: clean
	$(RM) -r $(SRCDIR) redis.tar.gz
