.PHONY: all install uninstall

all:
	cargo build

install: all
	cp -f ./target/debug/launchio /bin
	chmod 755 /bin/launchio

uninstall:
	rm -f /bin/launchio
