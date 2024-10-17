# lesson 6

### 影片
* https://www.youtube.com/watch?v=RjdVXTYZYqk

### 問題
1. 影片中的字體太小，難以看到代碼
2. 影片內容已經是幾個月前，有些操作已經用不了

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
(因為以上version出問題)
error:

修改version:
rustup default stable

進行檢查
* rustup show

cargo contract new ink_2_project

[32:15]
開始寫ink contract
由於ink是在wasm內(所以要說明是nostd)

[37:45]
通過設置可變引用或不可變引用來表示能否改變它在區塊鏈上的狀態

[39:10]
```
$[ink(message)]
        pub fn balance_of(&self, who: AccountId) -> Balance { 
            self.balance.get(&who).unwrap_or_default()
        }
```
如果none就會返回0

[39:55]
需要聲明result
type Result<T> = core::result::Result<T, Error>
當中的Error需要自行定義!

[41:20]
transfer function

[44:25]
定義event

[45:44]
* cargo contract build


### notes:
0. 版本修正:
* 轉用rust stable版本
* rustup default stable
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_3.png?raw=true)

1. 當下版本
* rustup show
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_0.png?raw=true)

2. 進行項目 E2E test
* export CONTRACTS_NODE="/root/substrate/20241015/substrate-contracts-node"
* cargo test --features e2e-tests
* 但有錯誤msg:
The 'substrate-contracts-node' executable was not found. Install 'substrate-contracts-node' on the PATH, or specify the `CONTRACTS_NODE` environment variable.
* screencap:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_2.png?raw=true)

* cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git
  error[E0152]: duplicate lang item in crate `std` (which `memchr` depends on): `panic_impl`
    |
    = note: the lang item is first defined in crate `sp_io` (which `frame_support` depends on)
    = note: first definition in `sp_io` loaded from /tmp/cargo-installFnwm0I/release/wbuild/contracts-parachain-runtime/target/wasm32-unknown-unknown/release/deps/libsp_io-c274a413a0a278be.rmeta
    = note: second definition in `std` loaded from /tmp/cargo-installFnwm0I/release/wbuild/contracts-parachain-runtime/target/wasm32-unknown-unknown/release/deps/libstd-38fc9796a84a90f2.rmeta
* screencap:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_1.png?raw=true)


* cargo install contracts-node
  error[E0152]: duplicate lang item in crate `std` (which `memchr` depends on): `panic_impl`
    |
    = note: the lang item is first defined in crate `sp_io` (which `frame_support` depends on)
    = note: first definition in `sp_io` loaded from /tmp/cargo-installIBdi32/release/wbuild/contracts-parachain-runtime/target/wasm32-unknown-unknown/release/deps/libsp_io-c274a413a0a278be.rmeta
    = note: second definition in `std` loaded from /tmp/cargo-installIBdi32/release/wbuild/contracts-parachain-runtime/target/wasm32-unknown-unknown/release/deps/libstd-38fc9796a84a90f2.rmeta
* screencap:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_1.png?raw=true)


* git clone https://github.com/paritytech/substrate-contracts-node.git
* cd substrate-contracts-node
* cargo build
* 遇到錯誤訊息:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_4.png?raw=true)


* cargo build dylint-link
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_5.png?raw=true)

* git clone https://github.com/paritytech/substrate-contracts-node.git
* cd substrate-contracts-node
* cargo build
* 遇到錯誤訊息:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_6.png?raw=true)

解決方法:
* sudo apt-get install libxcb1-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-render0-dev

* git clone https://github.com/paritytech/substrate-contracts-node.git
* cd substrate-contracts-node
* cargo build
* 遇到錯誤訊息:
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241016_7.png?raw=true)


### testing
* cargo install contracts-node --git https://github.com/MartinYeung5/substrate-contracts-node.git


remove old version
* rustup toolchain remove 1.76.0-x86_64-unknown-linux-gnu