EXE_DIR=.

sentinel: debug

release:
	cargo build --release
	mv target/release/make-rs $(EXE_DIR)

debug:
	cargo build
	mv target/debug/make-rs $(EXE_DIR)

clean:
	cargo clean
	rm make-rs