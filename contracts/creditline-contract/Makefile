build-deps:
	rustup target add wasm32-unknown-unknown
	cargo build --target wasm32-unknown-unknown --release \
	  --manifest-path ../../Cargo.toml \
	  -p liquidity-pool-contract
	cargo build --target wasm32-unknown-unknown --release \
	  --manifest-path ../../Cargo.toml \
	  -p vendor-registry-contract
