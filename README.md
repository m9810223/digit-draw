# Digit Draw

數字抽選遊戲 — 透過九宮格逐位選取數字，組成一個金額。

**[線上試玩](https://m9810223.github.io/digit-draw/)**

## 玩法

1. 設定金額範圍（位數或最大金額）
2. 在九宮格上逐一點選 1-9 的數字，從個位數開始填入
3. 填滿後揭曉金額
4. 按「再玩一次」重新開始

### 遊戲選項

- **不允許重複** — 每個數字只能選一次，點已選的數字可取消
- **隱藏模式** — 數字全部以 `$` 顯示，選完後按「確認開獎！」才揭曉

### 互動功能

- **隨機位置** — 打亂九宮格排列
- **隨機轉動** — 九宮格 3D 翻轉動畫
- **大小滑桿** — 調整九宮格顯示大小

## 技術

- [Leptos](https://leptos.dev/) 0.7（CSR）
- Rust → WebAssembly
- [Trunk](https://trunkrs.dev/) 建構
- GitHub Pages 部署

## 開發

### 首次設定

```sh
just setup
```

### 常用指令

```sh
just dev     # 啟動開發伺服器（hot-reload）
just check   # 語法檢查
just fmt     # 格式化
just lint    # Lint 檢查
just build   # 建置生產版本
```

執行 `just` 可查看所有可用指令。
