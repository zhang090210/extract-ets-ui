# e听说提取（高中版）`v5.7.1`

这是一个e听说答案提取的软件，软件版本号与e听说版本号保持一致，这个仓库提供了软件的页面部分。

## 使用方法：

### 1.从源码构建：

1. 安装[nodejs v20.18.0](https://nodejs.org/dist/v20.18.0/node-v20.18.0-win-x64.zip)
2. 安装pnpm `npm install --global pnpm --egistry=https://registry.npmmirror.com`
3. 安装[rust 1.81.0](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
3. 安装[wkhtmltopdf 0.12.3](https://github.com/wkhtmltopdf/wkhtmltopdf/releases/tag/0.12.3.2)
4. 克隆源代码：

```
git clone -b senior https://github.com/zhang090210/extract-ets-ui.git
cd extract-ets-ui
pnpm install
pnpm tauri build
```

若要编译x86版本：

```
rustup target add i686-pc-windows-msvc
cargo install create-tauri-app --locked
git clone -b senior https://github.com/zhang090210/extract-ets-ui.git
cd extract-ets-ui
pnpm install
cargo tauri build --target i686-pc-windows-msvc
```

### 2.下载预编译版

打开[Release v5.7.1]([Release v5.7.1 · zhang090210/extract-ets-ui (github.com)](https://github.com/zhang090210/extract-ets-ui/releases/tag/v5.7.1))页面
