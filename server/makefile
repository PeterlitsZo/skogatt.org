test:
	RUST_LOG=debug \
	JSON_FILE_NAME=/tmp/data.json \
	DB_FILE_NAME=/tmp/db.sqlite \
	cargo run

release-build:
	JSON_FILE_NAME=/var/server/data.json \
	DB_FILE_NAME=/var/server/db.sqlite \
	cargo build --release
