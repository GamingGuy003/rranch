build:
	cargo build --release

install:
	sudo cp target/release/rranch /usr/bin/

clean:
	cargo clean

redeploy: | build install

all: | clean build install