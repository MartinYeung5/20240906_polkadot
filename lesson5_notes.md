# lesson5

### 重點
* XCM
* parachain

[2:00]
* 跟其他parachain互動，需要在本地測試

[2:20]
* parachain & relaychain

有3個特性
1. 不可能三角
* security: relaychain
交易處理，打包都需要區塊鏈驗證
* scalability: parachain
* decentralization: validators DPoS

2. 其他解決方法
* L1, L2 OP, ZK Rollup, AppChain,

polkadot協議: 有relaychain統一負責 (例如: polkadot)，有自己的節點可以來驗證區塊，
還會驗證parachain所產生的block。

[5:00]
* 把安仕放到relaychain
[5:25]
涉及到大量業務就會放到parachain，可以根據自己業務進行擴展，最終區塊需要到relaychain上驗證
* 把做抉擇的權利分散

* ZK Rollup 需要大量算力，存儲十分大

[9:40]
* Appchain, 每條鏈都有自己的安全機制，通過消息進行交互，由於每條鏈都有自己的安全機制，所以當其中一條鏈出問題，會影響其他的鏈或者是影響鏈之間的通訊。

[11:50]
polkadot 提升
1. security: protocal 在 relaychain高效驗證，validators輪換
2. Scalability: parachain TPS and relaychain 驗證能力
* relaychain 驗證能力 會影響到parachain的TPS
3. decentralization: validators, nominators and tokenomics in relaychain

[13:40]
* XCM
1. XCM 是一種語言消息的格式
例如: 可以傳送一個資產，需要定義資產的資料 (id 是什麼，數量用什麼表示)
2. XCM 可以用於不同的共識系統，不單單是parachain，可以是pallet, contract, or 橋接另一個系統
3. XCM 是有版本的，可不斷進行，
* 不會同時支持很多版本，只可以支持向下的一個版本，可以不斷逼使大家更新到最新版本

[17:20]
有4個A
1. Agnostic: 沒有對共識系統的實現做任何假設
* 例如: in XCM 的資產, 就只有一個資產id, 不會判斷資產是erc20或者是parachain上native的資產
2. Absolute: 有一個能保證消息發送, 正確的解析和順序
3. Asynchronous:消息是異步，不會在某個地方block某個消息
4. Asymmetric: (不對稱的) 消息是不對稱的，消息不會保證有回覆/或者確認，除非有顯式的要求

[21:18]
* XCM location & asset

[22:20]
* Location
* 每個location is struct, each location will have junction

* universal location: 類似根目錄

[24:26]
* location hierarchy
1. relay
2. parachain
3. account or pallet or contracts

[24:50]
Asset
* asset 最重要的是會包含一個location

[26:00]
代碼解釋
* 可以在pokadot-sdk下找到
https://github.com/paritytech/polkadot-sdk

[27:20]
* junctions是一個數組，在外面有一個enum

```
pub enum Junctions {
	/// The interpreting consensus system.
	Here,
	/// A relative path comprising 1 junction.
	X1(Arc<[Junction; 1]>),
	/// A relative path comprising 2 junctions.
	X2(Arc<[Junction; 2]>),
	/// A relative path comprising 3 junctions.
	X3(Arc<[Junction; 3]>),
	/// A relative path comprising 4 junctions.
	X4(Arc<[Junction; 4]>),
	/// A relative path comprising 5 junctions.
	X5(Arc<[Junction; 5]>),
	/// A relative path comprising 6 junctions.
	X6(Arc<[Junction; 6]>),
	/// A relative path comprising 7 junctions.
	X7(Arc<[Junction; 7]>),
	/// A relative path comprising 8 junctions.
	X8(Arc<[Junction; 8]>),
}
```

* junction 也是一個enum
```
Parachain(#[codec(compact)] u32),
respected as a sovereign
AccountId32 { network: Option<NetworkId>, id: [u8; 32] },
AccountIndex64 {
		network: Option<NetworkId>,
		#[codec(compact)]
		index: u64,
	},
AccountKey20 { network: Option<NetworkId>, key: [u8; 20] },
PalletInstance(u8),
GeneralIndex(#[codec(compact)] u128),
GeneralKey { length: u8, data: [u8; 32] },
OnlyChild,
Plurality { id: BodyId, part: BodyPart },
GlobalConsensus(NetworkId),
```

[29:15]
asset.rs

[30:16]
* XCVM
* 它是處理XCM消息的虛擬機，基於寄存器的
* message是一個指令序列

* messages:
1. UMP
2. DMP
3. XCMP
4. HRMP
* relaychain: UMP & HRMP
* parachain: DMP & XCMP

[35:00]
* XCVM registers
* 指令是它的origin, holdings
* XCM message is collecyion of XCVM instructions
* XCVM is a status machine, state is kept in registers.

[35:48]
XCVM instruction
* command
* trusted indiction
* information
* system notification

[37:10]
看PPT展示
* example: WithdrawAsset

[38:00]
* example: ReceiveTeleportedAsset

[38:30]
* example: QueryResopnse
* 向對方report information, 之後可以獲得一個回覆

[39:00]
* XCVM operation
首先會有Registers :Registers 內有origins + holding (當前是空的)，會有一個executor來進行fetch動作，之後就會去到"program"‧。
* WithdrawAsset: 
1. 會從origins中get asset
2. 將asset 通過"put asset"放到holding
3. BuyExecution 可以獲得手續費 (需要與Exector互動,執行fetch)
4. DepositAsset (需要與Exector互動,執行fetch), 去holding get asset
5. 如果順利完成, 會put aaset 去 Beneficiary

[43:20]
2. Reserve asset transfers

[45:00]
* example: parachain native tokens

[46:00]
* XCM 組成 
* 每個步驟都會通過一個instruction定義

[47:55]
XCM pallets

[52:00]
代碼解釋:
* xcm/xcm-executor/src/lib.rs
* pub struct XcmExecutor

[54:00]
* fn execute

[55:15]
* substrate/frame/auara/lib.rs


[56:30]
* cumulus/pallets/aura-ext/src/lib.rs
(與共識相關的pallet)

[57:20]
會包含在parachain的runtime
* pallet_aura
* cumulus_pallet_aura_ext

[58:00]
xcm/xcm-executor/src/lib.rs

[58:26]
* fn reserve_transfer_assets (轉移資產)
* fn force_xcm_version (xcm版本處理)

[59:10]
* 定義origins
* cumulus/pallets/xcm/src/lib.rs
* pub enum Origin

[1:01:00]
* cumulus/pallets/xcmp-queue/src/lib.rs

[1:03:30]
* cumulus/parachains/pallets/ping/src/lib.rs
* 在兩個不同的鏈，可以互發消息, 有來有往
* 首先要進行pallet綁定
* 跟已經配置好的target chain發消息

[1:10:20]
* ZombiNet
* 創建一個有relaychain + parachain環境

[1:14:00]
* zombinet.toml

[1:!6:10]
zombinet --provider natice spawn ./zombinet.tome

* lesson5:
https://github.com/papermoonio/polkadot-sdk-course-code/tree/main/advance/lesson-5

