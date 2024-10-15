# lesson 6

### 影片
* https://www.youtube.com/watch?v=RjdVXTYZYqk

### 重點
[2:25]
why webAssembly?
* Wasm is a platform independent
1. sandboxed
2. fast
3. compact
4. well support

[3:10]
why rust?
1. build directly to Wasm
2. repidly developing ecosystem
3. ergonomic and easier to get right

[4:30]
contract components
* ink! vs solidity

[8:47]
* call substrate runtime
1. solidity > precompile
2. ink!
* call_runtime
* chain_extension
在EVM外運算
[13:00]
ink! 例子:
* ink1合約內部想拿鏈上的randomness
[15:11]
* parachain 可以相互調用
[15:48]
ink! 合約架構跟Solidity差不多
[16:00]
contract structure
* Storage: #[ink(storage)]
* Contract instantiation: #[ink(contructor)]
* Invokable function: #[ink(message)]
* Event: #[ink(event)]

[16:50]
STORAGE ink! support
* commmon types: bool, u[], i[], string, tuples, enum, struct
* ink! types: common types, mapping, vex, enum(Rust), struct
* Substrate types: AccountId, Balance, Hash

[17:45]
* 環境安裝
1. install substrate contracrt node
* git clone https://github.com/paritytech/substrate-contracts-node.git
* git checkout v0.25.0
2. install cargo-contract
* cargo install dylint-link
* cargo install cargo-contract
[18:00]
contract call substrate runtime
[18:09]
https://use.ink/ink-vs-solidity

* ink! rust, solidity, ask! assembly script 都可以encode to WebAssembly Bytecode, 
通過API，與pallet-contracts 互動

[20:20]
ink contract 中會有ink_env, 
會在ink_env中由env backend實現 env tree ?
* envbackend 會有off chain / on chain
* off chain > 會在std, 通常用於測試的功能 
* on chain > 會在鏈上面進行 (ext)
通過host function 去調用 wasm 中的pallet contracts

[21:47]
工具推薦
* https://polkadot.js.org/apps
* https://contracts-ui.substrate.io/

[22:30]
* git clone https://github.com/paritytech/substrate-contracts-node.git
2. install cargo-contract
* cargo install cargo-contract
3. create new project
cargo contract new 20241015_ink_project

[27:40]
如何寫ink! contract

[30:00]
* cargo contract --version
建議用3.0.1 version
* rustup show
建議用1.70.0 version
* rustup toolchain add nightly-2023-03-18
rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-18-x86_64-unknown-linux-gnu
進行檢查
* rustup show
rustup default nightly-2023-03-18-x86_64-unknown-linux-gnu
進行檢查
* rustup show

[32:15]
開始寫ink contract
由於ink是在wasm內(所以要說明是nostd)

