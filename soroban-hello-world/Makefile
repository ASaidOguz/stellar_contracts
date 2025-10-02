deploy:
	stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source-account stellar_ahmet \
  --network testnet \
  --alias hello_world
invoke:
	stellar contract invoke \
  --id CBTCA3DVYV3UGPCJ6YOUXFCTMDCVAZTGVETWNYWSPVYEL5DCI2RCP5S7 \
  --source-account stellar_ahmet \
  --network testnet \
  -- \
  hello \
  --to World