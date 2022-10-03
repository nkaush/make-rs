sentinel: debug

release:
	cargo build --release
	mv target/release/make-rs .

debug:
	cargo build
	mv target/debug/make-rs .

clean:
	cargo clean
	rm make-rs

a: b c

b: a

c:
	echo hi