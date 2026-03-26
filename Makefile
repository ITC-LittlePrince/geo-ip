PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin

.PHONY: install uninstall build

build:
	cargo build --release

install: build
	sudo install -m 755 ./target/release/geo-ip $(BINDIR)/

uninstall:
	sudo rm -f $(BINDIR)/geo-ip
