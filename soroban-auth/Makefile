# === Top-level Makefile ===
# location of your contract subfolder
CONTRACT_DIR := contracts/auth

# === Stellar CLI tasks ===
generate-account1:
	stellar keys generate stellar_ahmet --network testnet --fund

generate-account2:
	stellar keys generate stellar_test --network testnet --fund

add-account:
	stellar keys add stellar_ahmet 

check-account:
	stellar keys address stellar_ahmet

deploy: 
	stellar contract deploy \
		--wasm target/wasm32v1-none/release/auth.wasm \
		--source-account stellar_ahmet \
		--network testnet \
		--alias auth\
		-- --admin_address GBLEM3MIX2B6EUSLMCQ252INUCYNKN2WQDJWTOKN7EJPBY5EI4WLDYHB\

invoke-increment-owner:
	stellar contract invoke \
		--id CAQHSMS34QJ4FWIFQ22SSE4TSSRVRYOYX6Z5MAPWTLRLBWC3BXDANAWV \
		--source-account stellar_ahmet \
		--network testnet \
		-- \
		increment\
		--value 69
### This should fail the simulation!
invoke-increment-nonowner:
	stellar contract invoke \
		--id CAQHSMS34QJ4FWIFQ22SSE4TSSRVRYOYX6Z5MAPWTLRLBWC3BXDANAWV \
		--source-account stellar_test \
		--network testnet \
		-- \
		increment\
		--value 69

invoke-get-state:
	stellar contract invoke \
		--id CCDL2LWGHBXSYNGT7VI3535KVDO3FBA23XVCQATJNGQ6X6SEVVTD4HMN \
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
