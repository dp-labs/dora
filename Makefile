TARGET_DIR ?= target/release

bench:
	cargo bench -p dora-bench

build:
	cargo build -r --all

build-bin:
	cargo clean && cargo build -r --manifest-path crates/dora-cli/Cargo.toml

build-c-binding:
	cargo clean && cargo build -r --manifest-path bindings/c/Cargo.toml

check:
	cargo check -r --all

test:
	cargo test -r --all

accept:
	cargo insta accept --all

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --all-features --benches --examples --tests -- -D warnings

fix:
	cargo clippy --workspace --all-features --benches --examples --tests --fix --allow-dirty

dist:
	# Build and copy binary
	mkdir -p dist/bin && mkdir -p dist/include && mkdir -p dist/lib
	make build-bin
	cp -r $(TARGET_DIR)/dora dist/bin

	# Build and copy c binding
	make build-c-binding
	cp bindings/c/include/dora.h dist/include
	cp bindings/c/README.md dist/include/README.md

	if [ -f $(TARGET_DIR)/dora_c.dll ]; then \
		cp $(TARGET_DIR)/dora_c.dll dist/lib/dora_c.dll ;\
	fi

	if [ -f $(TARGET_DIR)/dora_c.dll.lib ]; then \
		cp $(TARGET_DIR)/dora_c.dll.lib dist/lib/dora_c.dll.lib ;\
	fi

	if [ -f $(TARGET_DIR)/dora_c.lib ]; then \
		cp $(TARGET_DIR)/dora_c.lib dist/lib/dora_c.lib ;\
	fi

	if [ -f $(TARGET_DIR)/libdora_c.dylib ]; then \
		cp $(TARGET_DIR)/libdora_c.dylib dist/lib/libdora_c.dylib ;\
	fi

	if [ -f $(TARGET_DIR)/libdora_c.so ]; then \
		cp $(TARGET_DIR)/libdora_c.so dist/lib/libdora_c.so ;\
	fi

	if [ -f $(TARGET_DIR)/libdora_c.a ]; then \
		cp $(TARGET_DIR)/libdora_c.a dist/lib/libdora_c.a ;\
	fi

	tar -C dist -zcvf dora.tar.gz bin lib include
	mv dora.tar.gz dist
	rm -rf dist/bin dist/include dist/lib
