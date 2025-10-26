# 安装最新、体验工具链

```bash
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

# 测试

```bash
rustup show

# =========> 显示
installed toolchains
--------------------
stable-x86_64-pc-windows-msvc (default)
nightly-x86_64-pc-windows-msvc

active toolchain
----------------
name: stable-x86_64-pc-windows-msvc
```

```bash
rustup target list --installed

# =========> 显示
wasm32-unknown-unknown
```

# 指定项目使用 最新、体验工具链

```bash
cd substrate-node-template   # 需要 nightly 的项目
rustup override set nightly  # 仅对当前目录生效
```

```bash
# 在需要 nightly 的项目里
rustc --version                 # 1.78.0-nightly (xxxx)

# 回到自己项目
cd ~/my-app
rustc --version                 # 1.77.0 (stable)
```

# 临时一次

```bash
cargo +nightly build          # 仅这次用 nightly，不影响任何目录
```
