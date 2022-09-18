run-server:
	RUST_LOG=info cargo run --bin server

run-client:
	RUST_LOG=info cargo run --bin client

curl:
	./scripts/curl.greeter.sh

cli-client:
	RUST_LOG=info cargo run --bin cli -- -c client

cli-server:
	RUST_LOG=info cargo run --bin cli -- -c server