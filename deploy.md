cargo stylus deploy \
  --endpoint='https://sepolia-rollup.arbitrum.io/rpc' \
  --private-key="0x8ff739615077fc354b94336e31bee972aeb5f821043e3ff25ac20b24128f5f9d" \
  --no-verify

cargo stylus deploy \
    --wasm-file target/wasm32-unknown-unknown/release/music_streaming_contracts.wasm \
    --private-key="0x8ff739615077fc354b94336e31bee972aeb5f821043e3ff25ac20b24128f5f9d" \
    --endpoint="https://sepolia-rollup.arbitrum.io/rpc" \
    --no-verify