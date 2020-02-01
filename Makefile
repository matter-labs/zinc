.PHONY: book
book:
	mdbook build --open --dest-dir './build/' './zinc-book/'

.PHONY: install
install:
	cargo install --force --path zinc-vm
	cargo install --force --path zinc-compiler
	cargo install --force --path zargo

.PHONY: test-integration
test-integration: install
	./test/integration.sh
