# plread

基于 Polars 的 CLI 工具，快速预览 Arrow IPC / IPC Stream / Parquet 文件。

## 安装

```bash
cargo install --git https://github.com/GreyRaphael/plread.git --tag v1.1.0
```

或从 [Releases](https://github.com/GreyRaphael/plread/releases) 页面下载预编译二进制。

## 用法

```bash
# 读取 IPC 文件（LazyFrame scan，原生 glob 支持）
plread ipc "data/*.feather"

# 读取 IPC Stream 文件（eager 模式，手动 glob）
plread ipc-stream "logs/*.arrow"

# 读取 Parquet 文件（LazyFrame scan，原生 glob 支持）
plread parquet "data/*.parquet"

# 控制最大显示行数（默认 10）
plread ipc big.feather --max-rows 20
```

## 参数

| 参数 | 默认值 | 说明 |
|------|--------|------|
| `--max-rows N` | 10 | 最大显示行数（由 `POLARS_FMT_MAX_ROWS` 控制） |

## 列显示策略

程序自动检测终端宽度，动态计算最大显示列数：

- `POLARS_TABLE_WIDTH` = 终端实际宽度（fallback 120）
- `POLARS_FMT_MAX_COLS` = `terminal_width / 15`（每列约 15 字符宽）

终端越宽，显示的列越多。超出部分由 polars 自动截断并显示 `…`。

## 架构

| 子命令 | 实现方式 | glob 支持 |
|--------|---------|-----------|
| `ipc` | `LazyFrame::scan_ipc` → `collect()` | 原生 |
| `parquet` | `LazyFrame::scan_parquet` → `collect()` | 原生 |
| `ipc-stream` | 手动 `glob::glob()` → `IpcStreamReader` → `concat_df_diagonal` | 手动 |

glob 匹配的多个文件会自动纵向合并（concat）。

## 构建

```bash
cargo build --release
```
