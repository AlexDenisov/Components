default: test

build: 
	cargo build --release --target=x86_64-apple-darwin

test: build
	COMPONENTS_EXEC=./target/x86_64-apple-darwin/release/components cargo test -- --nocapture

