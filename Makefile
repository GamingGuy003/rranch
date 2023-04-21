build:
	cargo build --release

install:
	mkdir -p $(DESTDIR)/usr/bin
	cp target/release/rranch $(DESTDIR)/usr/bin/

clean:
	cargo clean

redeploy: | build install

all: | clean build install