build-mac:
	cargo build --release --target aarch64-apple-darwin --bin messiah
	cd target/aarch64-apple-darwin/release/ && tar -czf messiah-$(V)-aarch64-apple-darwin.tar.gz messiah
build-linux:
	cargo build --release --target x86_64-unknown-linux-gnu --bin messiah
	cd target/x86_64-unknown-linux-gnu/release/ && tar -czf messiah-$(V)-x86_64-unknown-linux-gnu.tar.gz messiah
build-win:
	cargo build --release --target x86_64-pc-windows-gnu --bin messiah
	cd target/x86_64-pc-windows-gnu/release/ && tar -czf messiah-$(V)-x86_64-pc-windows-gnu.tar.gz messiah.exe