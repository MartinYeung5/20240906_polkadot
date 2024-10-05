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
