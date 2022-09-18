run-server:
	cargo run --bin server

run-client:
	cargo run --bin client

curl:
	./scripts/curl.greeter.sh

cli-client:
	cargo run --bin cli -- -c client