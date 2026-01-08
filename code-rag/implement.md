# Go 代码 RAG 图谱工具 - 实施方案

## 问题回顾

1. **AI 编码漏改**: 函数变更后下游调用者未同步更新
2. **AI 失忆**: 跨对话无法保持项目结构认知

## 解决方案概述

构建一个基于静态分析的 Go 代码调用图谱工具 `crag`，通过 `golang.org/x/tools` 进行精确分析，持久化到 SQLite，用于追踪变更影响范围。

## 技术架构

```
输入层         静态分析层                  存储层       查询层
┌─────────┐   ┌─────────────────────────┐ ┌────────┐  ┌────────────┐
│Go项目   │→ │go/packages → SSA → VTA  │→│SQLite  │→ │CLI工具     │
│源码     │   │加载        构建   调用图│  │        │  │影响分析    │
└─────────┘   └─────────────────────────┘ └────────┘  └────────────┘
```

## 项目结构

```
code-rag/
├── cmd/crag/main.go              # CLI 入口
├── internal/
│   ├── analyzer/
│   │   ├── loader.go             # go/packages 加载器
│   │   ├── ssa.go                # SSA 构建
│   │   └── callgraph.go          # VTA 调用图分析
│   ├── graph/
│   │   ├── node.go               # 节点定义
│   │   ├── edge.go               # 边定义
│   │   └── builder.go            # 图构建器
│   ├── storage/
│   │   ├── schema.sql            # SQLite 表结构
│   │   ├── db.go                 # 数据库操作
│   │   └── queries.go            # 查询方法（含递归CTE）
│   └── impact/
│       └── analyzer.go           # 变更影响分析
├── go.mod
└── go.sum
```

## 核心依赖

- `golang.org/x/tools` - 静态分析（go/packages, SSA, VTA）
- `modernc.org/sqlite` - 纯 Go 实现的 SQLite
- `github.com/spf13/cobra` - CLI 框架

## 数据模型

### 节点表 (nodes)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| kind | TEXT | 类型 (func/struct/interface/package) |
| name | TEXT | 完整限定名 |
| package | TEXT | 包路径 |
| file | TEXT | 源文件路径 |
| line | INTEGER | 起始行号 |
| signature | TEXT | 函数签名 |
| doc | TEXT | 文档注释 |

### 边表 (edges)
| 字段 | 类型 | 说明 |
|------|------|------|
| id | INTEGER | 主键 |
| from_id | INTEGER | 调用者节点ID |
| to_id | INTEGER | 被调用者节点ID |
| kind | TEXT | 关系类型 (calls/implements/references) |
| call_site_file | TEXT | 调用发生的文件 |
| call_site_line | INTEGER | 调用发生的行号 |

## CLI 命令

```bash
# 分析项目并构建调用图
crag analyze /path/to/project -o project.db

# 查询上游调用者（谁调用了它）
crag upstream "pkg/service.HandleRequest" --depth 3

# 查询下游依赖（它调用了谁）
crag downstream "pkg/service.HandleRequest" --depth 2

# 完整影响分析报告
crag impact "pkg/service.HandleRequest" --format markdown

# 搜索函数
crag search "HandleRequest"

# 列出所有函数
crag list --limit 20
```

## 影响分析输出示例

```markdown
## 变更影响分析: pkg/service.HandleRequest

**位置:** internal/service/handler.go:42
**签名:** `func(ctx context.Context, req *Request) (*Response, error)`

### 直接调用者 (需检查是否需要同步修改)
| 函数 | 文件 | 行号 |
|------|------|------|
| main.main | cmd/main.go | 42 |
| handler.ServeHTTP | internal/handler/http.go | 87 |

### 间接调用者 (可能受影响)
| 函数 | 文件 | 行号 |
|------|------|------|
| router.Setup | internal/router/setup.go | 23 |

### 下游依赖 (本函数调用的)
| 函数 | 文件 | 行号 |
|------|------|------|
| db.Query | internal/db/query.go | 15 |
| cache.Get | internal/cache/cache.go | 30 |
```

## AI 编码工作流集成

1. **项目初始化**: `crag analyze . -o .crag.db`
2. **修改前**: 运行 `crag impact <函数名>` 获取影响范围
3. **提供给 AI**: 将影响报告作为上下文
4. **修改后**: 再次运行确认无遗漏
5. **定期更新**: git hook 触发重新分析

## 实现状态

- [x] 项目结构和依赖初始化
- [x] SQLite schema 和数据库操作层
- [x] go/packages 加载器
- [x] SSA 构建和 VTA 调用图分析
- [x] 图构建器（调用图写入 SQLite）
- [x] 上下游影响分析查询（递归 CTE）
- [x] CLI 命令实现 (analyze/upstream/downstream/impact/list/search)

## 后续扩展方向

- 增量分析：只分析 git diff 涉及的文件
- VS Code/Cursor 插件：可视化调用图
- AI 自动集成：自动注入影响分析到 prompt
- 支持更多关系类型：interface 实现、struct 引用

