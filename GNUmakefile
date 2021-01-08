debug:
	cargo build

release:
	cargo build --release

doc:
	cargo doc

all: debug release

clean:
	cargo clean

format:
	find . -type f -name '*.rs' -print0 | xargs -r0 rustfmt -v
