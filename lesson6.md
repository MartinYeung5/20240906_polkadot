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