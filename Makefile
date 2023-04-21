build:
	cargo build --release

install:
	sudo mkdir -p $(DESTDIR)/usr/bin
	sudo cp target/release/rranch $(DESTDIR)/usr/bin/

clean:
	cargo clean

redeploy: | build install

all: | clean build install