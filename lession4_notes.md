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
2. on_runtime_upgrade in pallet