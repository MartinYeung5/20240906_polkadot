# 20240906
## task1 test
20240906 完成

# 20240909
https://youtu.be/yZ-LPBslotc

update:
* update runtime/src/lib.rs
row:345: [pallet_poe, PoeModule]

* cargo build --profile=production --features runtime-benchmarks
* ./target/production/solochain-template-node benchmark pallet \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet pallet_poe \
--extrinc "*" \
--steps 20 \
--repeat 10 \
--output pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs

## github
https://github.com/MartinYeung5/20240906_polkadot
