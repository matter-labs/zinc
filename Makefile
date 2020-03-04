.PHONY: book
book:
	mdbook build --open --dest-dir './build/' './zinc-book/'

.PHONY: install
install:
	cargo install --force --path zinc-vm
	cargo install --force --path zinc-compiler
	cargo install --force --path zargo
	cargo install --force --path zinc-tester
	cargo install --force --path schnorr

.PHONY: test-integration
test-integration: install
	./test/integration.sh

.PHONY: build-release-musl
build-release-musl:
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir zinc-linux
	cp target/x86_64-unknown-linux-musl/release/{zargo,znc,zinc} zinc-linux
	tar -czf zinc-linux.tar.gz zinc-linux
	rm -rf zinc-linux
