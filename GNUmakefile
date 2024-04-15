# GNUmakefile - just a shortcot for me to remember what to run

.PHONY: all
all:
	cargo build
	set -e; \
	for exe in target/debug/{rust-multi-multi,rmm-*}; do \
	  if test -x "$$exe"; then \
	    "$$exe"; \
	  fi; \
	done
