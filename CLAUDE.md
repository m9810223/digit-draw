# CLAUDE

## 專案概述

- **專案名稱**: red-envelope-game（紅包遊戲）
- **框架**: Leptos 0.7 (CSR - Client-Side Rendering)
- **語言**: Rust -> WebAssembly
- **建構工具**: Trunk

## Language Preferences

- 沒有指定語言時，一律使用`正體中文`

## 技術規範

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

- 目前使用純 CSS，樣式檔在 `style/main.css`
- 透過 `index.html` 中的 `<link data-trunk rel="css" href="style/main.css" />` 引入
- 配色：紅色主體（`--red-primary`）+ 金黃色輔助（`--gold-primary`），使用 CSS 變數管理

## Rust Practices

- 定義 function 時，如果超過 3 個參數，考慮使用 builder pattern 或 struct 封裝
- 遵循 Rust 標準命名慣例：snake_case 函式/變數、PascalCase 型別/元件、SCREAMING_SNAKE_CASE 常數

## 開發指令

- 開發模式：`trunk serve`
- 建置生產版本：`trunk build --release`
- 開發伺服器：`http://127.0.0.1:8080`

## Software Design

- 以 SoC (Separation of Concerns) 為核心設計原則
- 元件職責單一：一個元件只負責一件事
- 狀態提升：共享狀態放在最近的共同父元件
- 可重用元件透過 props 接收資料，避免硬編碼
