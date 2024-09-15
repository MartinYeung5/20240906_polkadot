# 20240906
## task1 test
20240906 完成
# homework-1
修改/編寫文檔:
* poe/src/tests.rs
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
--extrinsic "*" \
--steps 20 \
--repeat 10 \
--output pallets/poe/src/weights.rs \
--template .maintain/frame-weight-template.hbs

# 20240913
# homework-2
修改/編寫/新增文檔:
* 第1個:poe/src/benchmarking.rs
1. 將benchmark功能引入到runtime

* 第2個:poe/src/Cargo.toml 
1. 引用 sp-std = { git = "https://github.com/paritytech/polkadot-sdk.git", tag = "polkadot-v1.10.0", default-features = false }
2. 加入 "sp-std/std",

* 第3個:create .maintain

* 第4個:runtime/src/lib.rs 
1. #[runtime::pallet_index(8)]
    pub type PoeModule = pallet_poe; (line298)
2. (加[pallet_poe, PoeModule]) (line 346)
3. /// Import the template pallet.
pub use pallet_poe;
4. impl pallet_poe::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxClaimLength = ConstU32<3>;
}
[9:00]
* runtime/Cargo.toml 
1. (加"pallet-poe/runtime-benchmarks", 它是一個runtime benchmark)
2. pallet-poe = { default-features = false, path = "../pallets/poe"}
3. "pallet-poe/std",
4. "pallet-poe/try-runtime",
* Cargo.toml

# 改錯
https://github.com/coretime-dev/play-substrate/blob/master/pallets/poe/src/benchmarking.rs
benchmarking.rs
use frame_support::{BoundedVec, pallet_prelude::Get};

# 測試方法：
cargo build --profile=production --features runtime-benchmarks

# error:
error: failed to load manifest for workspace member `/home/yeung/substrate/polkadot-sdk-solo-template-dev-courses/node`
referenced by workspace at `/home/yeung/substrate/polkadot-sdk-solo-template-dev-courses/Cargo.toml`

Caused by:
  failed to load manifest for dependency `solochain-template-runtime`

Caused by:
  failed to parse manifest at `/home/yeung/substrate/polkadot-sdk-solo-template-dev-courses/runtime/Cargo.toml`

Caused by:
  feature `runtime-benchmarks` includes `pallet-poe/runtime-benchmarks`, but `pallet-poe` is not a dependency

solution:
update runtime/Cargo.toml


## github
https://github.com/MartinYeung5/20240906_polkadot
