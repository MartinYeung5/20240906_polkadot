https://www.youtube.com/watch?v=78cvpTAlp_s

# lession4 - 第四課：Pallet開發 2｜Substrate開發進階與項目實戰
## 20241002

* runtime upgrade
* try runtime tool usage
可以把現在的鏈的狀態導出來，然後放在本地，
或者直接連到live的鏈上，去看會不會出現什麼錯誤 (包括一些status, value)
* data migration
用舊數據去decode會有機會出現問題，所以有時候需要進行數據的migration

[3:50]
runtime upgrade
* 升級代碼是軟件開發經常遇到的事
* substrate 是第一個將runtime編譯成WASM，然後保存在鏈上
* runtime的在線升級,只是對on-chain狀態轉移一些數值，
如果是對Client部分, 就需要升級軟件

[7:35]
support of upgrade
1. execution in runtime
在runtime/src/lib.rs做設置, 通過修改Migrations 和 fn on_runtime_upgrade
[11:20]
2. on_runtime_upgrade in pallet
kitties/src/
[12:00]
try runtime tool usage
需要在一個定義的hook的方法做一個配合
kitties/src/hooks.rs
* pre_upgrade (方法)
* post_upgrade (方法)
[13:26]
判斷什麼的data要做data migration
* data in KV DB
1. storage value
2. storage map
[17:09]
要考慮以下3個情況
1. 不能得到之前的數據
2. 不能解析之前的數據
3. 解析出來的數據意義發生了變化

[18:02]
Data migration cases
1. pallet name or storage name change
2. 存儲的數據改變
3. x:u32 -> x:i32 (解析出來的數據意義發生了變化)
4. x:u32 -> x:(u,16, u16) (解析出來的數據意義發生了變化)
[20:18]
代碼練習
1. 安裝
* cargo install --git https://github.com/paritytech/try-runtime-cli --locked

2. 在線測試
try-runtime --runtime ./target/release/wbuild/solochain-template-runtime/solochain_template_runtime.wasm on-runtime-upgrade --checks pre-and-post --disable-idempotency-checks --no-weight-warnings live --url wa://127.0.0.1:9944

update runtime/src/lib.rs
[22:54]
in lesson 3 folder, 
(必須要執行狀態重置)
* rm -rf /tmp/blockchain
(起動blockchain)
* ./solochain-template-node --dev --base -path /tmp/blockchain

[23:10]
go to 
https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944

去開發者>交易

[23:58]
update runtime/src/lib.rs
1. 定義struct 
pub struct EcampleMigration<>

[26:30]
檢查鏈狀態:
1. go to 
https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944
2. 選擇鏈狀態
3. 選kitties
4. 選nextKittyId():u32
5. 按一下右手訪的+號
6. 會在最下面顯示 kitties.nextKittyId():u32的最新ID
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_7.png?raw=true)

[31:00]
為了測試，需要做version的升級
由100升到101

[31:05]
* cargorel --features try-runtime
* cargo build --release --features try-runtime
* cargo build --release

[31:40]
update runtime/src/lib.rs
type Migrations = (ExampleMigration<Runtime>);

[33:00]
invalid state
(必須要執行狀態重置)
* rm -rf /tmp/blockchain

my testing
* cargo test -p pallet-kitties
* ./polkadot-sdk-solo-template-dev-courses --dev --base-path /tmp/blockchain

重要:
### case 1
in 20240927_task3
在20240927_task3加入polkadot-sdk-solo-template-dev-courses folder
* cd polkadot-sdk-solo-template-dev-courses
cargo build --release
* ./polkadot-sdk-solo-template-dev-courses/target/release/solochain-template-node --dev --base-path /tmp/blockchain
* go to 
https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944
* 找不到kitties的pallet

### case 2
in 20240927_task3
* cargo build --release
成功
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_1.png?raw=true)

* ./target/release/solochain-template-node --dev --base-path /tmp/blockchain
成功
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_2.png?raw=true)

前往:
https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944
* 找不到kitties的pallet
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_3.png?raw=true)

問題在於runtime/lib.rs, 看有沒有加上kitties pallet
如果加上, 應該沒問題

![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_4.png?raw=true)

開始測試:
1. 提出交易 (create)
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_5.png?raw=true)
2. 完成交易 (會看到右上角有綠色剔號)
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_6.png?raw=true)

[34:00]
執行升級前要清空所有狀態(如果有提出過交易)
(必須要執行狀態重置)
1. rm -rf /tmp/blockchain
2. ./target/release/solochain-template-node --dev --base-path /tmp/blockchain
[34:50]
3. go to frontend 檢查
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_8.png?raw=true)

如果遇到以下錯誤，需要在Cargo.toml加入log
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_9.png?raw=true)

解決方法:
在Cargo.toml加入log:
"log = { version = "0.4.21", default-features = false }"

如果遇到以下錯誤，
![alt text](https://github.com/MartinYeung5/20240906_polkadot/blob/main/Image/20241002_10.png?raw=true)
需要在kitties/src/lib.rs 加入:
// TryRuntimeError
use sp_runtime::TryRuntimeError;