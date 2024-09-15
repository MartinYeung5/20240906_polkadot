# 20240915
# 第3課筆記
* Kitties Pallet
1. create
2. breed
3. transfer
4. sale
在指定時間進行銷售?
5. bid (highest, earliest)

* Hooks [4:25]
1. on_initialized
在區塊開始時
2. on_finalize
在區塊結束時
3. on_idle
當有一些處理可以在不同區塊執行，即是不一定在當下區塊執行，就可以利用on_idle。
4. on_runtime_upgrade
在區塊開始時，又有一些代碼改變時, 可以進行upgrade
5. on_genesis

6. offchain_worker
超動一些線下線程
7. on_poll
是一個optional的動作
8. pre_upgrade (不討論)
9. post_upgrade (不討論)
10. try_state (不討論)

* Block執行過程 [8:10]
首先，在有需要時會執行OnRuntimeUpgrade，
在BeforeExtrinsics，會執行OnInitialize (會優先)，
之後會去到Inherents，當完成之後，
會去到OnPoll部分，然後執行OnPoll，但這部分是選擇性的，不一定會執行。
接著會去到Extrinsics，交易會按照順序執行，
最後會去到AfterExtrinsics，在這部分會執行Onidle和OnFinialize。

* on_runtime_upgrade[12:15]
在處理一個區塊時，第一個被調用的module(模塊)就是它。
1. 當下的區塊的資料是不會知道，只能夠知道上一個區塊的資料。[12:50]
2. 在upgrade時，會花大量時間去處理各種事，有機會出現超時，影響同步狀態。
3. 可以透過多個的區塊進行upgrade

* on_initialized [14:22]
所有必須在這個區塊的動作，會在這裡做好準備。
值得注意: 在當下所用到的所有數值都是舊的，包括pallet-timestamp，因為在當下是仍未完成更新。

* on_poll[15:52]
它是一個可以選擇的調用單元，如果一個區塊在upgrade時，initialize消耗太多weights時，poll就不會被調用。
這個也是它和on_initialize的最大分別，另外，它是在inherent之後才執行。

* 代碼講解[17:00]

* [21:32]
RUST_LOG=info cargo test

* [23:01]
--dev --tmp
了解相關的調用
kittie的調用順序是按照runtime所編排的順序
mock.rs (可改動順序?)

* Balances PAllet [25:04]
1. 怎樣和一條鏈的native token進行交互
2. 把reserve/unreserve應用到kitties中
3. 把轉移Kitty的所有權的時候做Token的轉移

* pallet 實現 (balances) [27:38]
 建議看官方源代碼

* src/extrinsics.rs [29:42]
random value還沒實現

* src/impls.rs [30:27]

* src/config.rs [30:47]
已經可以從Randomness獲得random number

* src/extrinsics.rs [31:10]

* src/error.rs [34:35]
update error.rs
加入 KittyIdOverflow

* check [34:48]

* src/tests.rs [35:19]
寫測試代碼

* src/extrinsics.rs [38:25]

* src/tests.rs [40:00]
寫測試代碼

* events.rs [43:00]

* src/tests.rs [44:20]

* Offchain worker [47:50]
1. how to add offchain worker in runtime
2. implement an offchain worker in kitties pallet
3. get the Dot price and put the Dollar amount in event

* offchain worker 例子 [50:50]
1. 官方例子

* 與balance交互 [58:00]


# homework-3
影片中提到的homework3的repo是這個
https://github.com/papermoonio/polkadot-sdk-course-code/tree/main/advance/lesson-3