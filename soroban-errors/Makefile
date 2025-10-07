# === Top-level Makefile ===
# location of your contract subfolder
CONTRACT_DIR := contracts/errors

# === Stellar CLI tasks ===

add-account:
	stellar keys generate stellar_ahmet --network testnet --fund

check-account:
	stellar keys address stellar_ahmet

upload: 
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/errors.wasm \
		--source-account stellar_ahmet \
		--network testnet \
		--alias errors

deploy: build
	stellar contract deploy \
		--wasm-hash 781aa4cd9a3f64ee4028b2c9b4a2824d42e6e9b48115194eb17ecd5fa2cb0ae1 \
		--source-account stellar_ahmet \
		--network testnet \
		--alias errors

invoke-increment:
	stellar contract invoke \
		--id CBVPWMFO2NHWLFVSYBSXNJM36AAS2U3V6NCMDIFQ7QOMZZMLMJPQO6DI \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		increment



# === Local development/test helpers ===

test-with-logs:
	cargo test -- --nocapture

test:
	cargo test

build:
	$(MAKE) -C $(CONTRACT_DIR) build

fmt:
	$(MAKE) -C $(CONTRACT_DIR) fmt

clean:
	$(MAKE) -C $(CONTRACT_DIR) clean
