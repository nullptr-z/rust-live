# crag - Code RAG 调用图谱工具

[![Go Version](https://img.shields.io/badge/Go-1.24+-00ADD8?style=flat&logo=go)](https://go.dev/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**crag** (Code RAG) 是一个 Go 代码静态分析工具，用于构建函数调用图谱，帮助追踪代码变更的影响范围，减少 AI 编码时的漏改问题。

## 问题背景

使用 AI 辅助编码时，常遇到两个痛点：

1. **漏改问题**：修改某个函数后，AI 不知道还有哪些地方调用了它，导致下游代码未同步更新
2. **失忆问题**：跨对话时 AI 无法保持对项目结构的认知，每次都要重新理解

## 解决方案

crag 通过精确的静态分析构建函数调用图谱：

- 使用 `golang.org/x/tools/go/ssa` 构建 SSA 中间表示
- 使用 VTA (Variable Type Analysis) 精确处理 interface 调用
- 将调用关系持久化到 SQLite，支持快速查询
- 提供影响分析报告，可直接作为 AI 上下文

## 安装

```bash
# 从源码安装
git clone https://github.com/zheng/crag.git
cd crag
go build -o crag ./cmd/crag/

# 或直接 go install
go install github.com/zheng/crag/cmd/crag@latest
```

## 快速开始

```bash
# 1. 分析项目，生成调用图谱
crag analyze /path/to/your/go/project -o .crag.db

# 2. 导出完整 RAG 文档
crag export -d .crag.db -o crag.md

# 3. 查看某函数的影响范围
crag impact "main.HandleRequest" -d .crag.db

# 4. 查看谁调用了某函数
crag upstream "pkg/db.Query" -d .crag.db --depth 3

# 5. 查看某函数调用了谁
crag downstream "pkg/service.Process" -d .crag.db --depth 2

# 6. 搜索函数
crag search "Handler" -d .crag.db

# 7. 列出所有函数
crag list -d .crag.db --limit 20

# 8. 启动 watch 模式 (文件变更自动更新)
crag watch . -d .crag.db

# 9. 启动 Web UI 可视化 (交互式调用图)
crag serve -d .crag.db
```

## 命令详解

### `analyze` - 分析项目

```bash
crag analyze [项目路径] [flags]

Flags:
  -o, --output string   输出数据库路径
  -i, --incremental     增量分析模式 (只在有 git 变更时分析)
      --base string     git 比较基准 (默认 "HEAD")
```

**增量分析示例：**

```bash
# 检测未提交的变更，无变更时跳过分析
crag analyze . -i

# 与上次提交比较
crag analyze . -i --base HEAD~1

# 与特定分支比较
crag analyze . -i --base main
```

### `impact` - 影响分析

```bash
crag impact <函数名> [flags]

Flags:
  --upstream-depth int    上游递归深度 (默认 3)
  --downstream-depth int  下游递归深度 (默认 2)
  --format string         输出格式: text/json/markdown (默认 "text")
```

输出示例：

```markdown
## 变更影响分析: pkg/service.HandleRequest

**位置:** internal/service/handler.go:42
**签名:** `func(ctx context.Context, req *Request) (*Response, error)`

### 直接调用者 (需检查是否需要同步修改)
| 函数 | 文件 | 行号 |
|------|------|------|
| main.main | cmd/main.go | 42 |
| handler.ServeHTTP | internal/handler/http.go | 87 |

### 下游依赖 (本函数调用的)
| 函数 | 文件 | 行号 |
|------|------|------|
| db.Query | internal/db/query.go | 15 |
```

### `upstream` / `downstream` - 调用链查询

```bash
crag upstream <函数名> [flags]
crag downstream <函数名> [flags]

Flags:
  --depth int      递归深度，0 表示无限 (默认 0)
  --format string  输出格式: text/json/markdown
```

### `watch` - 监控模式

启动 watch 模式，监控项目中的 Go 文件变更，自动重新分析并更新调用图：

```bash
crag watch [项目路径] [flags]

Flags:
  --debounce int   防抖延迟，毫秒 (默认 500)
```

**使用示例：**

```bash
# 监控当前目录
crag watch .

# 指定数据库路径
crag watch . -d .crag.db

# 设置 1 秒防抖延迟（适用于频繁保存的场景）
crag watch . --debounce 1000
```

**特性：**
- 自动递归监控所有目录
- 防抖处理，避免频繁触发分析
- 忽略测试文件（`*_test.go`）
- 忽略隐藏目录、`vendor`、`node_modules` 等

### `serve` - Web UI 可视化

启动本地 Web 服务器，提供交互式调用图可视化界面：

```bash
crag serve [flags]

Flags:
  -p, --port int   服务器端口 (默认 9998)
```

**使用示例：**

```bash
# 使用默认端口 9998
crag serve -d .crag.db

# 指定端口
crag serve -d .crag.db -p 3000
```

**功能特性：**
- 🔍 **交互式图谱**：缩放、拖拽、点击节点
- 🎯 **影响分析**：双击节点高亮上下游调用链
- 🔎 **搜索过滤**：快速定位目标函数
- 📊 **详情面板**：查看节点的调用者/被调用者
- 🎨 **按包着色**：不同模块用不同颜色区分

**快捷键：**
- `/` 聚焦搜索框
- `Esc` 重置高亮

### `export` - 导出 RAG 文档

生成完整的项目调用图谱文档，可直接作为 AI 编码上下文：

```bash
crag export [flags]

Flags:
  -o, --output string   输出文件路径 (默认输出到 stdout)
  -i, --incremental     增量导出 (只输出 git 变更部分)
      --base string     git 比较基准 (默认 "HEAD")
      --no-mermaid      不生成 Mermaid 图表
```

**使用示例：**

```bash
# 导出完整 RAG 文档
crag export -d .crag.db -o crag.md

# 增量导出（只输出变更部分）
crag export -d .crag.db -i -o changes.md

# 与上次提交比较的变更
crag export -d .crag.db -i --base HEAD~1
```

**输出内容：**
- 项目统计（节点数、边数）
- Mermaid 调用关系图
- 按包分组的函数列表（位置、签名、调用关系）
- 修改影响速查表

## MCP 集成 (推荐)

crag 实现了 [MCP (Model Context Protocol)](https://modelcontextprotocol.io/)，让 AI 助手（Cursor、Claude 等）可以**直接查询**调用图，无需复制粘贴。

### 配置 Cursor

1. 确保 `crag` 在 PATH 中
2. 在 Cursor 设置中添加 MCP 服务器配置（`.cursor/mcp.json`）：

```json
{
  "mcpServers": {
    "crag": {
      "command": "crag",
      "args": ["mcp", "-d", "/path/to/your/project/.crag.db"]
    }
  }
}
```

3. 重启 Cursor，AI 即可直接使用以下工具：

| 工具 | 功能 |
|------|------|
| `impact` | 分析函数变更影响范围 |
| `upstream` | 查询上游调用者 |
| `downstream` | 查询下游被调用者 |
| `search` | 搜索函数 |
| `list` | 列出所有函数 |

### 使用示例

配置后，AI 会自动调用这些工具。你可以直接问：

```
"LoadPackages 函数被哪些地方调用了？"
"如果我修改 BuildSSA，会影响哪些函数？"
"搜索所有包含 Handler 的函数"
```

## AI 编码工作流

### 方式一：MCP 直接查询（推荐）

```bash
# 1. 分析项目
crag analyze . -o .crag.db

# 2. 配置 MCP (见上方)

# 3. 直接与 AI 对话，AI 会自动查询调用图
```

### 方式二：导出 RAG 文档

```bash
# 1. 导出完整 RAG
crag export -d .crag.db -o crag.md

# 2. 将 crag.md 作为 AI 上下文
```

### 持续更新

**方式一：通过 git hook 自动更新**

```bash
# .git/hooks/post-commit
#!/bin/bash
crag analyze . -i -o .crag.db
```

**方式二：使用 watch 模式实时更新**

```bash
# 在另一个终端启动 watch 模式
crag watch . -d .crag.db

# 输出示例：
# 执行初始分析...
# 初始分析完成: 42 节点, 128 边
#
# 开始监控目录: .
# 数据库路径: .crag.db
# 防抖延迟: 500ms
#
# 按 Ctrl+C 停止...
#
# [15:04:05] 检测到变更，开始分析...
# [15:04:06] 分析完成: 43 节点, 131 边 (耗时 892ms)
```

## 项目结构

```
crag/
├── cmd/crag/main.go              # CLI 入口
├── internal/
│   ├── analyzer/                 # 静态分析
│   │   ├── loader.go            # go/packages 加载
│   │   ├── ssa.go               # SSA 构建
│   │   ├── callgraph.go         # VTA 调用图
│   │   └── git.go               # Git 变更检测
│   ├── graph/                    # 图数据结构
│   │   ├── node.go              # 节点定义
│   │   ├── edge.go              # 边定义
│   │   └── builder.go           # 图构建器
│   ├── storage/                  # 数据持久化
│   │   ├── schema.sql           # SQLite 表结构
│   │   ├── db.go                # 数据库操作
│   │   └── queries.go           # 查询方法
│   ├── impact/
│   │   └── analyzer.go          # 影响分析
│   ├── export/
│   │   └── exporter.go          # RAG 文档导出
│   ├── mcp/
│   │   └── server.go            # MCP 服务器
│   ├── web/                      # Web UI
│   │   ├── server.go            # HTTP API 服务器
│   │   └── static/index.html    # 前端页面 (vis.js)
│   └── watcher/
│       └── watcher.go           # 文件监控器
├── go.mod
└── mcp.json                      # MCP 配置示例
```

## 技术栈

- **静态分析**: `golang.org/x/tools/go/packages`, `go/ssa`, `go/callgraph/vta`
- **数据存储**: `modernc.org/sqlite` (纯 Go 实现)
- **CLI 框架**: `github.com/spf13/cobra`

## 限制

- 目前仅支持 Go 项目
- 只分析项目自身代码，不包含依赖包

## 后续规划

- [x] 增量分析：检测 git 变更，无变更时跳过分析
- [x] RAG 导出：生成完整/增量的 Markdown 文档
- [x] MCP 集成：让 AI 直接查询调用图
- [x] watch 模式：文件变更自动更新
- [x] Web UI：可视化调用图（交互式力导向图）
- [ ] interface 实现分析：显示谁实现了什么接

## License

MIT License

