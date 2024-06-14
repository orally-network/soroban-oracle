# Soroban Oracle contracts

# How to use

Install Soroban-SDK:
Version 21.0.0-rc.1 fixes error `xdr processing error: xdr value invalid` 
'''
cargo install --locked --version 21.0.0-rc.1 --features opt  soroban-cli
'''

Install contract wasm and deploy
```
soroban contract build && soroban contract optimize --wasm ${PATH_TO_WASM}
WASM_HASH=$(soroban contract install --network ${NETWORK} --source ${SOROBAN_KEY} --wasm ${PATH_TO_WASM})
soroban contract deploy  --network ${NETWORK} --source ${SOROBAN_KEY} --wasm-hash ${WASM_HASH}      
```

Upgrade contract 
```
soroban contract build && soroban contract optimize --wasm ${PATH_TO_WASM} && \
WASM_HASH=$(soroban contract install --network ${NETWORK} --source ${SOROBAN_KEY} --wasm ${PATH_TO_WASM}) && \
soroban contract invoke  --network ${NETWORK} --source ${SOROBAN_KEY} --id ${CONTRACT_ID} -- upgrade --wasm_hash ${WASM_HASH} 
```


Example of usage
```
soroban contract invoke  --network ${NETWORK} --source ${SOROBAN_KEY} --id ${CONTRACT_ID} -- is_reporter --address "0x61E9658dFE7c620E96ae41f97b89B079Ef7ECd1A"

soroban contract invoke  --network ${NETWORK} --source ${SOROBAN_KEY} --id ${CONTRACT_ID} -- verify_generic --data ${TEST_GET_XRC_DATA_BYTES}

```