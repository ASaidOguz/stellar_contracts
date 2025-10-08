# === Top-level Makefile ===
# location of your contract subfolder
CONTRACT_DIR := contracts/auth

# === Stellar CLI tasks ===

add-account:
	stellar keys generate stellar_ahmet --network testnet --fund

check-account:
	stellar keys address stellar_ahmet

upload: 
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/auth.wasm \
		--source-account stellar_ahmet \
		--network testnet \
		--alias auth

deploy: 
	stellar contract deploy \
		--wasm-hash 781aa4cd9a3f64ee4028b2c9b4a2824d42e6e9b48115194eb17ecd5fa2cb0ae1 \
		--source-account stellar_ahmet \
		--network testnet \
		--alias auth

invoke-increment:
	stellar contract invoke \
		--id CC4VJI3DHIXC6KU7DTOPHVQHCHPMLUDBFTZJJENHFEQ3WRKXTSDTU5UQ \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		increment\
		--value 69

invoke-init:
	stellar contract invoke \
		--id CC4VJI3DHIXC6KU7DTOPHVQHCHPMLUDBFTZJJENHFEQ3WRKXTSDTU5UQ \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		init

invoke-get-state:
	stellar contract invoke \
		--id CC4VJI3DHIXC6KU7DTOPHVQHCHPMLUDBFTZJJENHFEQ3WRKXTSDTU5UQ \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		get_initstate\
		



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
