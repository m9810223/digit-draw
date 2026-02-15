[private]
default:
    just --fmt --unstable 2> /dev/null
    just --list --unsorted

# 啟動開發伺服器（hot-reload）
[group("dev")]
dev:
    trunk serve

# 檢查語法（不實際編譯）
[group("dev")]
check:
    cargo check --target wasm32-unknown-unknown

# 建置生產版本
[group("build")]
build:
    trunk build --release

# 清除建置產物
[group("build")]
clean:
    cargo clean
    rm -rf dist

# 格式化程式碼
[group("quality")]
fmt:
    cargo fmt
    leptosfmt src/**/*.rs 2>/dev/null || true

# Lint 檢查
[group("quality")]
lint:
    cargo clippy --target wasm32-unknown-unknown -- -D warnings

# 部署到 GitHub Pages
[group("deploy")]
deploy repo:
    trunk build --release --public-url /{{repo}}/
    npx gh-pages -d dist

# 安裝開發所需工具（首次設定用）
[group("setup")]
setup:
    rustup target add wasm32-unknown-unknown
    cargo install trunk --locked
    cargo install leptosfmt --locked
