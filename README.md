# YD - 文本翻译软件

YD 是一个基于 Rust 的文本翻译软件，使用有道翻译 API 进行翻译。

## 功能

- 支持多种语言的文本翻译。
- 简单易用的命令行界面。
- 高性能的异步请求处理。

## 安装

1. 克隆仓库：

   ```bash
   git clone https://github.com/YorigamiChitose/yd.git
   ```

2. 进入项目目录：

   ```bash
   cd yd
   ```

3. 构建项目：

   ```bash
   cargo build --release
   ```

4. 将生成的可执行文件添加到 PATH 中：

   ~~下面的指令仅供参考，请根据实际情况调整~~

   ```bash
   export PATH=$PATH:$(pwd)/target/release
   ```

## 使用方法

1. 获取[有道翻译 API ](https://ai.youdao.com/doc.s#guide)的 `YD_APP_ID` 和 `YD_APP_KEY`。

2. 在环境变量中添加以下内容：

   ```env
   YD_APP_ID=your_app_id
   YD_APP_KEY=your_app_key
   ```

3. 运行程序并输入要翻译的文本：

   ```bash
   yd "要翻译的文本"
   ```
   ```bash
   echo "要翻译的文本" | yd
   ```

## 示例

```bash
yd "Hello, world!"
```

输出：

```
你好，世界！
```

## 依赖项

- [clap](https://crates.io/crates/clap) - 命令行参数解析器。
- [reqwest](https://crates.io/crates/reqwest) - HTTP 客户端。
- [serde](https://crates.io/crates/serde) - 序列化和反序列化库。
- [serde_json](https://crates.io/crates/serde_json) - JSON 序列化和反序列化库。
- [sha2](https://crates.io/crates/sha2) - SHA-2 哈希函数库。
- [tokio](https://crates.io/crates/tokio) - 异步运行时。
- [uuid](https://crates.io/crates/uuid) - UUID 生成库。


## 贡献

欢迎贡献代码！请先阅读 [CONTRIBUTING.md](CONTRIBUTING.md) 了解贡献指南。

## 许可证

本项目采用 MIT 许可证。详细信息请参阅 [LICENSE](LICENSE) 文件。

## 联系

如有任何问题或建议，请通过 [GitHub Issues](https://github.com/YorigamiChitose/yd/issues) 联系我们。
