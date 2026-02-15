# CLAUDE

## 專案概述

- **專案名稱**: digit-draw（數字抽選遊戲）
- **框架**: Leptos 0.7 (CSR - Client-Side Rendering)
- **語言**: Rust -> WebAssembly
- **建構工具**: Trunk
- **部署**: GitHub Pages（GitHub Actions 自動部署）
- **線上版本**: `https://m9810223.github.io/digit-draw/`

## Language Preferences

- 沒有指定語言時，一律使用`正體中文`

## 遊戲核心邏輯

### 遊戲流程

1. **設定範圍** → 2. **點選數字逐位填入** → 3. **金額揭曉** → 4. **再玩一次**

玩家透過九宮格（1-9）逐位選取數字，組成一個金額。選取順序：個位 → 十位 → 百位...（index 0 = 個位）。

### 兩種範圍模式（`LimitMode`）

| 模式 | 設定方式 | 位數計算 | 限制邏輯 |
|---|---|---|---|
| `ByDigits` | stepper 選 1-9 位 | 直接使用設定值 | 只限制位數 |
| `ByMaxAmount` | 輸入最大金額 | `floor(log10(max)) + 1` | 位數 + 金額不超過上限（`would_exceed()` 即時計算） |

### 兩種遊戲選項

| 選項 | 效果 |
|---|---|
| **不允許重複** | 每個數字只能選一次；已選的數字可再點一次來取消選取 |
| **隱藏模式** | 九宮格按鈕和金額格都顯示 `$`；全部選完後需按「確認開獎！」才揭曉 |

### 互動功能

- **隨機位置**：Fisher-Yates shuffle 打亂九宮格排列
- **隨機轉動**：3D CSS transform（`TRANSFORM_PAIRS` 常數定義 8 組配對），cell 內文字做反向 transform 維持可讀性
- **大小滑桿**：調整九宮格 max-width（50%-100%）

## 技術規範

### 狀態管理

所有狀態集中在 `App` 元件，子元件透過 `ReadSignal` / `WriteSignal` / `Callback` 單向傳遞。

| Signal | 型別 | 用途 |
|---|---|---|
| `mode` | `LimitMode` | 範圍限制模式 |
| `selected_digits` | `u8` | 位數（ByDigits 模式） |
| `max_amount` | `u64` | 最大金額（ByMaxAmount 模式） |
| `no_repeat` | `bool` | 不允許重複 |
| `hidden_mode` | `bool` | 隱藏模式 |
| `selected_numbers` | `Vec<u8>` | 已選數字序列（index 0 = 個位） |
| `game_finished` | `bool` | 遊戲是否結束 |
| `digits_needed` | `Memo<u8>` | 衍生：需要的位數 |

**URL 同步**：`mode` / `digits` / `max_amount` 變更時透過 `Effect` + `history.replace_state` 同步至 URL query string（`?digits=N` 或 `?max=N`），頁面載入時從 URL 讀取初始值。

### Leptos 慣例

- 使用 `leptos::prelude::*` 作為標準 prelude import
- 元件使用 `#[component]` 巨集，回傳 `impl IntoView`
- 狀態管理使用 `signal()`（fine-grained reactivity）
- 模板使用 `view!` 巨集
- 字串在 `view!` 巨集內必須用雙引號包裹，例如 `"文字"`
- 事件處理使用 `on:click=move |_| { ... }` 格式

### 元件架構

| 元件名 | 檔案 | 職責 |
|---|---|---|
| `App` | `app.rs` | 根元件，管理所有狀態 + URL query 同步 |
| `AmountDisplay` | `amount_display.rs` | 金額顯示（預覽 + 開獎/再玩按鈕） |
| `NumberPad` | `number_pad.rs` | 九宮格數字鍵盤 + 大小 slider + 隨機轉動/位置 |
| `RangeConfig` | `range_config.rs` | 範圍設定（位數 stepper / 最大金額輸入 + 模式切換） |
| `GameOptions` | `game_options.rs` | 遊戲選項（不允許重複 / 隱藏模式 toggle button） |

### 檔案組織

- 每個元件一個檔案，放在 `src/` 下
- 元件檔案以 snake_case 命名，元件本身以 PascalCase 命名
- 樣式放在 `style/` 目錄下
- 共用型別與工具放在 `src/utils.rs` 或 `src/types.rs`

### CSS

- 純 CSS，樣式檔在 `style/main.css`
- 透過 `index.html` 中的 `<link data-trunk rel="css" href="style/main.css" />` 引入
- 配色方案：紅色主體 + 金黃色輔助，使用 CSS 變數管理

| 變數 | 值 | 用途 |
|---|---|---|
| `--red-primary` | `#c41e2a` | 主紅色 |
| `--red-dark` | `#9a1520` | 深紅色 |
| `--red-light` | `#e63946` | 亮紅色（hover） |
| `--gold-primary` | `#f0c040` | 主金色 |
| `--gold-light` | `#f5d76e` | 淺金色 |
| `--gold-dark` | `#c9a020` | 深金色 |
| `--bg-dark` | `#1a0a0a` | 深色背景 |

### 重要輔助函式（`number_pad.rs`）

- `current_amount(selected: &[u8]) -> u64`：將已選數字組成十進位金額
- `would_exceed(selected, candidate, max_amount) -> bool`：判斷新增數字是否超過上限
- `random_transform(current: &str) -> (String, String)`：隨機選取不同的 CSS transform 配對

## Rust Practices

- 超過 3 個參數，考慮使用 builder pattern 或 struct 封裝
- 遵循 Rust 標準命名慣例：snake_case 函式/變數、PascalCase 型別/元件、SCREAMING_SNAKE_CASE 常數

## 開發指令

透過 `just` 執行所有指令，不直接呼叫底層工具：

| 指令 | 說明 | 群組 |
|---|---|---|
| `just dev` | 啟動開發伺服器（hot-reload） | dev |
| `just check` | 檢查語法（`cargo check --target wasm32-unknown-unknown`） | dev |
| `just build` | 建置生產版本 | build |
| `just clean` | 清除建置產物 | build |
| `just fmt` | 格式化程式碼（cargo fmt + leptosfmt） | quality |
| `just lint` | Lint 檢查（clippy） | quality |
| `just build-pages <repo>` | 建置 GitHub Pages 版本 | deploy |
| `just deploy <repo>` | 手動部署到 GitHub Pages | deploy |
| `just setup` | 安裝開發所需工具（首次設定用） | setup |

## Software Design

- 以 SoC (Separation of Concerns) 為核心設計原則
- 元件職責單一：一個元件只負責一件事
- 狀態提升：共享狀態放在最近的共同父元件（目前全部集中在 `App`）
- 可重用元件透過 props 接收資料，避免硬編碼
