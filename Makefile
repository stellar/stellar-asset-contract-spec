gen:
	cargo run -- generate --format json > stellar-asset-spec.json
	cargo run -- generate --format xdr > stellar-asset-spec.xdr
