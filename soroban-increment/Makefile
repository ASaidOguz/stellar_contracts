# === Top-level Makefile ===
# location of your contract subfolder
CONTRACT_DIR := contracts/increment

# === Stellar CLI tasks ===

add-account:
	stellar keys generate stellar_ahmet --network testnet --fund

check-account:
	stellar keys address stellar_ahmet

upload: build
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/increment.wasm \
		--source-account stellar_ahmet \
		--network testnet \
		--alias increment

deploy: build
	stellar contract deploy \
		--wasm-hash 781aa4cd9a3f64ee4028b2c9b4a2824d42e6e9b48115194eb17ecd5fa2cb0ae1 \
		--source-account stellar_ahmet \
		--network testnet \
		--alias increment

invoke-increment:
	stellar contract invoke \
		--id CCCOKAFQR5G5ABSWCJODK5OSFYENUWJSBH2HHQ2XP2EROHJIDTFYMCC6 \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		increment

invoke-decrement:
	stellar contract invoke \
		--id CCCOKAFQR5G5ABSWCJODK5OSFYENUWJSBH2HHQ2XP2EROHJIDTFYMCC6 \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		decrement

invoke-reset:
	stellar contract invoke \
		--id CCCOKAFQR5G5ABSWCJODK5OSFYENUWJSBH2HHQ2XP2EROHJIDTFYMCC6 \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		reset_count

invoke-get-count:
	stellar contract invoke \
		--id CCCOKAFQR5G5ABSWCJODK5OSFYENUWJSBH2HHQ2XP2EROHJIDTFYMCC6 \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		get_current_count

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
