# 20240906
## task1 test
20240906 完成
# homework-1
修改/編寫文檔:
* poe/src/ests.rs
* poe/src/lib.rs
* poe/src/mock.rs
* Cargo.toml
* poe/src/Cargo.toml

# 測試方法：
cargo test -p pallet-poe


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

# 20240913
# homework-2
修改/編寫文檔:
* 將benchmark功能引入到runtime
* runtime/src/lib.rs (加[pallet_poe, PoeModule])
* runtime/Cargo.toml (加"pallet-poe/runtime-benchmarks", 它是一個runtime benchmark)
* poe/src/mock.rs
* Cargo.toml
* poe/src/Cargo.toml

# 測試方法：


## github
https://github.com/MartinYeung5/20240906_polkadot
