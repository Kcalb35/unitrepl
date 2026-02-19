musl:
	cargo build --release --target x86_64-unknown-linux-musl && cp target/x86_64-unknown-linux-musl/release/unitrepl ./
.PHONY: musl
	