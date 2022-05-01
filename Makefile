build:
	cargo build

release:
	cargo build --release

lint:
	cargo clippy

test: ut end-to-end-test

ut:
	cargo test

end-to-end-test: release
	./tests/test_epoch.sh
