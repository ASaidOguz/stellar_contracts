deploy:
	stellar contract deploy \
  --wasm target/wasm32v1-none/release/increment.wasm \
  --source-account stellar_ahmet \
  --network testnet \
  --alias increment

invoke:
	stellar contract invoke \
  --id CASV2LJAKZ6DBKGO5LWKTFRBZXUKIQKMAHEU5THYWWBAGJC7CKFH7URS \
  --source-account stellar_ahmet \
  --network testnet \
  -- \
  increment \
  --