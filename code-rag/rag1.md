# 项目调用图谱 (RAG)

> 生成时间: 2026-01-09 13:41:30
> 函数节点: 129 | 调用边: 201

## 项目结构

```
├── crag/
│   ├── internal/
│   │   ├── analyzer/
│   │   ├── export/
│   │   ├── graph/
│   │   ├── impact/
│   │   ├── mcp/
│   │   ├── storage/
│   │   ├── watcher/
│   │   ├── web/
```

## 架构图

```mermaid
flowchart TB
    subgraph analyzer [静态分析层]
        BuildSSA[BuildSSA]
        GetGitChanges[GetGitChanges]
        String[String]
        GetCallGraphStats[GetCallGraphStats]
        LoadPackages[LoadPackages]
        GetChangedPackagePatterns[GetChangedPackagePatterns]
        BuildCallGraph[BuildCallGraph]
        FilterMainPackages[FilterMainPackages]
        HasChanges[HasChanges]
    end

    subgraph graph [图构建层]
        GetNodeCount[GetNodeCount]
        NewBuilder[NewBuilder]
        Build[Build]
    end

    subgraph storage [存储层]
        GetNodeByID[GetNodeByID]
        Conn[Conn]
        GetDownstreamCallees[GetDownstreamCallees]
        GetCallEdgesForNode[GetCallEdgesForNode]
        GetDirectCallees[GetDirectCallees]
        DeleteNodesByPackage[DeleteNodesByPackage]
        GetStats[GetStats]
        InsertNode[InsertNode]
        GetUpstreamCallers[GetUpstreamCallers]
        InsertEdge[InsertEdge]
        Clear[Clear]
        GetNodesByPackage[GetNodesByPackage]
        GetNodeByName[GetNodeByName]
        Open[Open]
        FindNodesByPattern[FindNodesByPattern]
        GetAllFunctions[GetAllFunctions]
        GetAllEdges[GetAllEdges]
        DeleteOrphanEdges[DeleteOrphanEdges]
        GetDirectCallers[GetDirectCallers]
        Close[Close]
    end

    subgraph impact [影响分析层]
        AnalyzeImpact[AnalyzeImpact]
        Summary[Summary]
        NewAnalyzer[NewAnalyzer]
        FormatMarkdown[FormatMarkdown]
    end

    subgraph export [导出层]
        Export[Export]
        ExportIncremental[ExportIncremental]
        DefaultExportOptions[DefaultExportOptions]
        NewExporter[NewExporter]
    end

    subgraph other [其他]
        Run[Run]
        NewServer[NewServer]
        Run[Run]
        NewServer[NewServer]
        New[New]
        WithOnAnalysisDone[WithOnAnalysisDone]
        Stop[Stop]
        WithOnError[WithOnError]
        WithOnAnalysisStart[WithOnAnalysisStart]
        Start[Start]
        WithDebounceDelay[WithDebounceDelay]
    end

    % 关键调用关系
    Export --> GetAllFunctions
    Export --> GetStats
    ExportIncremental --> GetAllFunctions
    ExportIncremental --> GetDirectCallers
    AnalyzeImpact --> GetNodeByName
    AnalyzeImpact --> FindNodesByPattern
    AnalyzeImpact --> GetDirectCallers
    AnalyzeImpact --> GetUpstreamCallers
    AnalyzeImpact --> GetDirectCallees
    AnalyzeImpact --> GetDownstreamCallees
```

---

## 模块详解

### 📦 internal/analyzer

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `GetChangedPackagePatterns` | GetChangedPackagePatterns r... | 0 | 0 |
| `HasChanges` | HasChanges returns true if ... | 0 | 0 |
| `String` | String returns a summary st... | 0 | 0 |
| `BuildCallGraph` | BuildCallGraph builds the c... | 2 | 0 |
| `BuildSSA` | BuildSSA builds the SSA rep... | 2 | 0 |
| `FilterMainPackages` | FilterMainPackages filters ... | 2 | 0 |
| `GetCallGraphStats` | GetCallGraphStats returns s... | 0 | 0 |
| `GetGitChanges` | GetGitChanges returns the l... | 0 | 0 |
| `LoadPackages` | LoadPackages loads all Go p... | 2 | 0 |

#### `GetChangedPackagePatterns`

- **位置**: `internal/analyzer/git.go:95`
- **签名**: `func() []string`
- **说明**: GetChangedPackagePatterns returns package patterns for go/packages.Load

#### `HasChanges`

- **位置**: `internal/analyzer/git.go:85`
- **签名**: `func() bool`
- **说明**: HasChanges returns true if there are any Go file changes

#### `String`

- **位置**: `internal/analyzer/git.go:90`
- **签名**: `func() string`
- **说明**: String returns a summary string of the changes

#### `BuildCallGraph`

- **位置**: `internal/analyzer/callgraph.go:12`
- **签名**: `func(prog *golang.org/x/tools/go/ssa.Program) (*golang.org/x/tools/go/callgraph.Graph, error)`
- **说明**: BuildCallGraph builds the call graph using VTA (Variable Type Analysis)
VTA is more precise than other algorithms for handling interface calls
- **被调用**: `runAnalysis`, `indexCmd$1`

#### `BuildSSA`

- **位置**: `internal/analyzer/ssa.go:10`
- **签名**: `func(pkgs []*golang.org/x/tools/go/packages.Package) (*golang.org/x/tools/go/ssa.Program, []*golang.org/x/tools/go/ssa.Package)`
- **说明**: BuildSSA builds the SSA representation for the given packages
- **被调用**: `runAnalysis`, `indexCmd$1`

#### `FilterMainPackages`

- **位置**: `internal/analyzer/loader.go:47`
- **签名**: `func(pkgs []*golang.org/x/tools/go/packages.Package) []*golang.org/x/tools/go/packages.Package`
- **说明**: FilterMainPackages filters packages to only include those with source files
- **被调用**: `runAnalysis`, `indexCmd$1`

#### `GetCallGraphStats`

- **位置**: `internal/analyzer/callgraph.go:29`
- **签名**: `func(cg *golang.org/x/tools/go/callgraph.Graph) github.com/zheng/crag/internal/analyzer.CallGraphStats`
- **说明**: GetCallGraphStats returns statistics about the call graph

#### `GetGitChanges`

- **位置**: `internal/analyzer/git.go:21`
- **签名**: `func(projectPath string, base string) (*github.com/zheng/crag/internal/analyzer.GitChanges, error)`
- **说明**: GetGitChanges returns the list of changed Go files since the last commit
If base is empty, it compares with HEAD (uncommitted changes)
If base is "HEAD~1", it compares with the previous commit

#### `LoadPackages`

- **位置**: `internal/analyzer/loader.go:10`
- **签名**: `func(projectPath string) ([]*golang.org/x/tools/go/packages.Package, error)`
- **说明**: LoadPackages loads all Go packages from the given project path
- **被调用**: `runAnalysis`, `indexCmd$1`

### 📦 internal/graph

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Build` | Build processes the call gr... | 2 | 2 |
| `GetNodeCount` | GetNodeCount returns the nu... | 0 | 0 |
| `NewBuilder` | NewBuilder creates a new gr... | 2 | 0 |
| `createFunctionNode` | createFunctionNode creates ... | 1 | 1 |
| `getDocComment` | getDocComment extracts the ... | 1 | 0 |
| `isProjectFunction` | isProjectFunction checks if... | 1 | 0 |

#### `Build`

- **位置**: `internal/graph/builder.go:59`
- **签名**: `func(cg *golang.org/x/tools/go/callgraph.Graph) error`
- **说明**: Build processes the call graph and stores nodes/edges
- **被调用**: `runAnalysis`, `indexCmd$1`
- **调用**: `isProjectFunction`, `createFunctionNode`

#### `GetNodeCount`

- **位置**: `internal/graph/builder.go:188`
- **签名**: `func() int`
- **说明**: GetNodeCount returns the number of nodes created

#### `NewBuilder`

- **位置**: `internal/graph/builder.go:25`
- **签名**: `func(fset *go/token.FileSet, pkgs []*golang.org/x/tools/go/packages.Package, insertFn func(*github.com/zheng/crag/internal/graph.Node) (int64, error), edgeFn func(*github.com/zheng/crag/internal/graph.Edge) error) *github.com/zheng/crag/internal/graph.Builder`
- **说明**: NewBuilder creates a new graph builder
- **被调用**: `runAnalysis`, `indexCmd$1`

### 📦 internal/storage

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Clear` | Clear removes all data from... | 2 | 0 |
| `Close` | Close closes the database c... | 5 | 0 |
| `Conn` | Conn returns the underlying... | 0 | 1 |
| `DeleteNodesByPackage` | DeleteNodesByPackage delete... | 0 | 3 |
| `DeleteOrphanEdges` | DeleteOrphanEdges deletes e... | 0 | 0 |
| `FindNodesByPattern` | FindNodesByPattern returns ... | 6 | 1 |
| `GetAllEdges` | GetAllEdges returns all edg... | 1 | 0 |
| `GetAllFunctions` | GetAllFunctions returns all... | 5 | 2 |
| `GetCallEdgesForNode` | GetCallEdgesForNode returns... | 0 | 0 |
| `GetDirectCallees` | GetDirectCallees returns fu... | 8 | 1 |
| `GetDirectCallers` | GetDirectCallers returns fu... | 8 | 1 |
| `GetDownstreamCallees` | GetDownstreamCallees return... | 4 | 1 |
| `GetNodeByID` | GetNodeByID returns a node ... | 2 | 1 |
| `GetNodeByName` | GetNodeByName returns a nod... | 1 | 1 |
| `GetNodesByPackage` | GetNodesByPackage returns a... | 0 | 2 |
| `GetStats` | GetStats returns database s... | 4 | 0 |
| `GetUpstreamCallers` | GetUpstreamCallers returns ... | 4 | 1 |
| `InsertEdge` | InsertEdge inserts an edge ... | 0 | 0 |
| `InsertNode` | InsertNode inserts a node i... | 0 | 0 |
| `Open` | Open opens or creates a SQL... | 5 | 0 |
| `hi` | - | 1 | 0 |
| `joinStrings` | - | 4 | 0 |
| `printhi1` | - | 1 | 0 |
| `scanNode` | - | 2 | 0 |
| `scanNodes` | - | 7 | 0 |

#### `Clear`

- **位置**: `internal/storage/db.go:47`
- **签名**: `func() error`
- **说明**: Clear removes all data from the database
- **被调用**: `runAnalysis`, `indexCmd$1`

#### `Close`

- **位置**: `internal/storage/db.go:42`
- **签名**: `func() error`
- **说明**: Close closes the database connection
- **被调用**: `runAnalysis`, `exportCmd$1`, `serveCmd$1`, `mcpCmd$1`, `indexCmd$1`

#### `Conn`

- **位置**: `internal/storage/db.go:53`
- **签名**: `func() *database/sql.DB`
- **说明**: Conn returns the underlying database connection for advanced queries
- **调用**: `hi`

#### `DeleteNodesByPackage`

- **位置**: `internal/storage/queries.go:277`
- **签名**: `func(packages []string) (int64, error)`
- **说明**: DeleteNodesByPackage deletes all nodes belonging to the specified packages
Also deletes all edges referencing those nodes
Returns the number of deleted nodes
- **调用**: `joinStrings`, `joinStrings`, `joinStrings`

#### `DeleteOrphanEdges`

- **位置**: `internal/storage/queries.go:309`
- **签名**: `func() (int64, error)`
- **说明**: DeleteOrphanEdges deletes edges that reference non-existent nodes

#### `FindNodesByPattern`

- **位置**: `internal/storage/queries.go:52`
- **签名**: `func(pattern string) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: FindNodesByPattern returns nodes matching a name pattern (using LIKE)
- **被调用**: `toolUpstream`, `toolSearch`, `toolDownstream`, `toolMermaid`, `AnalyzeImpact`, `handleSearch`
- **调用**: `scanNodes`

#### `GetAllEdges`

- **位置**: `internal/storage/queries.go:246`
- **签名**: `func() ([]*github.com/zheng/crag/internal/graph.Edge, error)`
- **说明**: GetAllEdges returns all edges in the database
- **被调用**: `handleGraph`

#### `GetAllFunctions`

- **位置**: `internal/storage/queries.go:228`
- **签名**: `func() ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetAllFunctions returns all function nodes
- **被调用**: `handleNodes`, `toolList`, `Export`, `handleGraph`, `ExportIncremental`
- **调用**: `printhi1`, `scanNodes`

#### `GetCallEdgesForNode`

- **位置**: `internal/storage/queries.go:197`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Edge, error)`
- **说明**: GetCallEdgesForNode returns all call edges where the node is the caller

#### `GetDirectCallees`

- **位置**: `internal/storage/queries.go:81`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDirectCallees returns functions that the given function directly calls
- **被调用**: `writeImpactTable`, `writeArchitectureDiagram`, `writePackageSection`, `writePackageSection`, `toolMermaid`, `toolMermaid`, `handleNode`, `AnalyzeImpact`
- **调用**: `scanNodes`

#### `GetDirectCallers`

- **位置**: `internal/storage/queries.go:65`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDirectCallers returns functions that directly call the given function
- **被调用**: `writeImpactTable`, `writePackageSection`, `writePackageSection`, `toolMermaid`, `toolMermaid`, `handleNode`, `AnalyzeImpact`, `ExportIncremental`
- **调用**: `scanNodes`

#### `GetDownstreamCallees`

- **位置**: `internal/storage/queries.go:148`
- **签名**: `func(nodeID int64, maxDepth int) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDownstreamCallees returns all downstream callees recursively up to maxDepth
If maxDepth is 0, it returns all callees with no depth limit
- **被调用**: `toolDownstream`, `toolMermaid`, `AnalyzeImpact`, `handleImpact`
- **调用**: `scanNodes`

#### `GetNodeByID`

- **位置**: `internal/storage/queries.go:43`
- **签名**: `func(id int64) (*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodeByID returns a node by its ID
- **被调用**: `handleNode`, `handleImpact`
- **调用**: `scanNode`

#### `GetNodeByName`

- **位置**: `internal/storage/queries.go:34`
- **签名**: `func(name string) (*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodeByName returns a node by its fully qualified name
- **被调用**: `AnalyzeImpact`
- **调用**: `scanNode`

#### `GetNodesByPackage`

- **位置**: `internal/storage/queries.go:322`
- **签名**: `func(packages []string) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodesByPackage returns all nodes in the specified packages
- **调用**: `joinStrings`, `scanNodes`

#### `GetStats`

- **位置**: `internal/storage/queries.go:344`
- **签名**: `func() (nodeCount int64, edgeCount int64, err error)`
- **说明**: GetStats returns database statistics
- **被调用**: `Export`, `runAnalysis`, `handleStats`, `indexCmd$1`

#### `GetUpstreamCallers`

- **位置**: `internal/storage/queries.go:98`
- **签名**: `func(nodeID int64, maxDepth int) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetUpstreamCallers returns all upstream callers recursively up to maxDepth
If maxDepth is 0, it returns all callers with no depth limit
- **被调用**: `toolUpstream`, `toolMermaid`, `AnalyzeImpact`, `handleImpact`
- **调用**: `scanNodes`

#### `InsertEdge`

- **位置**: `internal/storage/queries.go:24`
- **签名**: `func(edge *github.com/zheng/crag/internal/graph.Edge) error`
- **说明**: InsertEdge inserts an edge into the database

#### `InsertNode`

- **位置**: `internal/storage/queries.go:11`
- **签名**: `func(node *github.com/zheng/crag/internal/graph.Node) (int64, error)`
- **说明**: InsertNode inserts a node into the database and returns its ID

#### `Open`

- **位置**: `internal/storage/db.go:20`
- **签名**: `func(path string) (*github.com/zheng/crag/internal/storage.DB, error)`
- **说明**: Open opens or creates a SQLite database at the given path
- **被调用**: `runAnalysis`, `exportCmd$1`, `serveCmd$1`, `mcpCmd$1`, `indexCmd$1`

### 📦 internal/impact

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `AnalyzeImpact` | AnalyzeImpact analyzes the ... | 1 | 6 |
| `FormatMarkdown` | FormatMarkdown formats the ... | 1 | 0 |
| `Summary` | FormatJSON formats the impa... | 0 | 0 |
| `NewAnalyzer` | NewAnalyzer creates a new i... | 1 | 0 |

#### `AnalyzeImpact`

- **位置**: `internal/impact/analyzer.go:31`
- **签名**: `func(funcName string, upstreamDepth int, downstreamDepth int) (*github.com/zheng/crag/internal/impact.ImpactReport, error)`
- **说明**: AnalyzeImpact analyzes the impact of changing a function
- **被调用**: `toolImpact`
- **调用**: `GetNodeByName`, `FindNodesByPattern`, `GetDirectCallers`, `GetUpstreamCallers`, `GetDirectCallees`, `GetDownstreamCallees`

#### `FormatMarkdown`

- **位置**: `internal/impact/analyzer.go:109`
- **签名**: `func() string`
- **说明**: FormatMarkdown formats the impact report as markdown
- **被调用**: `toolImpact`

#### `Summary`

- **位置**: `internal/impact/analyzer.go:175`
- **签名**: `func() string`
- **说明**: FormatJSON formats the impact report as JSON (use encoding/json for actual serialization)

#### `NewAnalyzer`

- **位置**: `internal/impact/analyzer.go:17`
- **签名**: `func(db *github.com/zheng/crag/internal/storage.DB) *github.com/zheng/crag/internal/impact.Analyzer`
- **说明**: NewAnalyzer creates a new impact analyzer
- **被调用**: `toolImpact`

### 📦 internal/export

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Export` | Export generates a complete... | 1 | 8 |
| `ExportIncremental` | ExportIncremental generates... | 0 | 7 |
| `DefaultExportOptions` | DefaultExportOptions return... | 1 | 0 |
| `NewExporter` | NewExporter creates a new e... | 1 | 0 |
| `categorizePackages` | - | 1 | 0 |
| `writeArchitectureDiagram` | writeArchitectureDiagram wr... | 1 | 10 |
| `writeImpactTable` | writeImpactTable writes a s... | 1 | 4 |
| `writeImpactTable$1` | - | 0 | 0 |
| `writePackageSection` | writePackageSection writes ... | 1 | 12 |
| `writePackageSection$1` | - | 0 | 2 |
| `writeProjectStructure` | writeProjectStructure write... | 1 | 0 |
| `containsPackage` | - | 0 | 1 |
| `getLayerDisplayName` | - | 1 | 0 |
| `getLayerOrder` | - | 2 | 0 |
| `getRelativePath` | - | 4 | 0 |
| `getShortDisplayName` | - | 11 | 0 |
| `getShortPackageName` | - | 1 | 0 |
| `getSortedPackageNames` | - | 1 | 0 |
| `getSortedPackageNames$1` | - | 0 | 2 |
| `groupByPackage` | - | 1 | 0 |
| `isExportedFunc` | - | 2 | 1 |
| `isKeyFunction` | - | 4 | 1 |
| `makeNodeID` | - | 3 | 1 |
| `matchPackageSuffix` | - | 2 | 0 |
| `truncateDoc` | - | 1 | 0 |

#### `Export`

- **位置**: `internal/export/exporter.go:44`
- **签名**: `func(w io.Writer, opts github.com/zheng/crag/internal/export.ExportOptions) error`
- **说明**: Export generates a complete RAG document
- **被调用**: `exportCmd$1`
- **调用**: `GetAllFunctions`, `GetStats`, `groupByPackage`, `writeProjectStructure`, `writeArchitectureDiagram`, `getSortedPackageNames`, `writePackageSection`, `writeImpactTable`

#### `ExportIncremental`

- **位置**: `internal/export/exporter.go:297`
- **签名**: `func(w io.Writer, changedPackages []string, opts github.com/zheng/crag/internal/export.ExportOptions) error`
- **说明**: ExportIncremental generates a RAG document for changed packages only
- **调用**: `GetAllFunctions`, `matchPackageSuffix`, `getShortDisplayName`, `GetDirectCallers`, `getRelativePath`, `getShortDisplayName`, `getRelativePath`

#### `DefaultExportOptions`

- **位置**: `internal/export/exporter.go:34`
- **签名**: `func() github.com/zheng/crag/internal/export.ExportOptions`
- **说明**: DefaultExportOptions returns default export options
- **被调用**: `exportCmd$1`

#### `NewExporter`

- **位置**: `internal/export/exporter.go:21`
- **签名**: `func(db *github.com/zheng/crag/internal/storage.DB) *github.com/zheng/crag/internal/export.Exporter`
- **说明**: NewExporter creates a new exporter
- **被调用**: `exportCmd$1`

### 📦 zheng/crag

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `exportCmd` | - | 1 | 0 |
| `exportCmd$1` | - | 0 | 5 |
| `indexCmd` | - | 1 | 0 |
| `indexCmd$1` | - | 0 | 10 |
| `main` | - | 0 | 5 |
| `mcpCmd` | - | 1 | 0 |
| `mcpCmd$1` | - | 0 | 4 |
| `serveCmd` | - | 1 | 0 |
| `serveCmd$1` | - | 0 | 4 |
| `watchCmd` | - | 1 | 0 |
| `watchCmd$1` | - | 0 | 2 |

### 📦 internal/mcp

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Run` | Run starts the MCP server | 1 | 2 |
| `NewServer` | NewServer creates a new MCP... | 1 | 0 |
| `handleInitialize` | - | 1 | 1 |
| `handleRequest` | - | 1 | 4 |
| `handleToolsCall` | - | 1 | 8 |
| `handleToolsList` | - | 1 | 1 |
| `send` | - | 2 | 0 |
| `sendError` | - | 3 | 1 |
| `sendResult` | - | 3 | 1 |
| `toolDownstream` | - | 1 | 2 |
| `toolImpact` | - | 1 | 3 |
| `toolList` | - | 1 | 1 |
| `toolMermaid` | - | 1 | 20 |
| `toolSearch` | - | 1 | 1 |
| `toolUpstream` | - | 1 | 2 |
| `indexOf` | - | 2 | 0 |
| `lastIndex` | - | 4 | 0 |
| `nodeID` | - | 9 | 1 |
| `shortName` | Helper functions for Mermai... | 5 | 6 |

#### `Run`

- **位置**: `internal/mcp/server.go:113`
- **签名**: `func() error`
- **说明**: Run starts the MCP server
- **被调用**: `mcpCmd$1`
- **调用**: `sendError`, `handleRequest`

#### `NewServer`

- **位置**: `internal/mcp/server.go:22`
- **签名**: `func(db *github.com/zheng/crag/internal/storage.DB) *github.com/zheng/crag/internal/mcp.Server`
- **说明**: NewServer creates a new MCP server
- **被调用**: `mcpCmd$1`

### 📦 internal/web

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Run` | Run starts the web server | 1 | 0 |
| `NewServer` | NewServer creates a new web... | 1 | 0 |
| `handleGraph` | handleGraph returns the com... | 0 | 4 |
| `handleImpact` | handleImpact returns impact... | 0 | 7 |
| `handleNode` | handleNode returns a single... | 0 | 7 |
| `handleNodes` | handleNodes returns all nodes | 0 | 3 |
| `handleSearch` | handleSearch searches for n... | 0 | 4 |
| `handleStats` | handleStats returns databas... | 0 | 2 |
| `getPackageGroup` | - | 1 | 0 |
| `nodeToData` | Helper functions | 5 | 2 |
| `nodesToData` | - | 5 | 1 |
| `shortName` | - | 1 | 0 |
| `writeJSON` | - | 7 | 0 |

#### `Run`

- **位置**: `internal/web/server.go:67`
- **签名**: `func() error`
- **说明**: Run starts the web server
- **被调用**: `serveCmd$1`

#### `NewServer`

- **位置**: `internal/web/server.go:27`
- **签名**: `func(db *github.com/zheng/crag/internal/storage.DB, port int) *github.com/zheng/crag/internal/web.Server`
- **说明**: NewServer creates a new web server
- **被调用**: `serveCmd$1`

### 📦 internal/watcher

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Start` | Start begins watching for c... | 1 | 1 |
| `Stop` | Stop stops the watcher | 0 | 0 |
| `New` | New creates a new Watcher | 1 | 1 |
| `WithDebounceDelay` | WithDebounceDelay sets the ... | 0 | 0 |
| `WithDebounceDelay$1` | - | 0 | 0 |
| `WithOnAnalysisDone` | WithOnAnalysisDone sets the... | 0 | 0 |
| `WithOnAnalysisDone$1` | - | 0 | 0 |
| `WithOnAnalysisStart` | WithOnAnalysisStart sets th... | 0 | 0 |
| `WithOnAnalysisStart$1` | - | 0 | 0 |
| `WithOnError` | WithOnError sets the callba... | 0 | 0 |
| `WithOnError$1` | - | 0 | 0 |
| `addDirs` | addDirs recursively adds al... | 1 | 0 |
| `addDirs$1` | - | 0 | 0 |
| `eventLoop` | eventLoop handles file syst... | 1 | 1 |
| `handleEvent` | handleEvent processes a sin... | 1 | 0 |
| `runAnalysis` | runAnalysis performs the ac... | 1 | 10 |
| `triggerAnalysis` | triggerAnalysis runs the an... | 0 | 1 |

#### `Start`

- **位置**: `internal/watcher/watcher.go:119`
- **签名**: `func()`
- **说明**: Start begins watching for changes
- **被调用**: `watchCmd$1`
- **调用**: `eventLoop`

#### `Stop`

- **位置**: `internal/watcher/watcher.go:124`
- **签名**: `func() error`
- **说明**: Stop stops the watcher

#### `New`

- **位置**: `internal/watcher/watcher.go:70`
- **签名**: `func(projectPath string, dbPath string, opts ...github.com/zheng/crag/internal/watcher.WatcherOption) (*github.com/zheng/crag/internal/watcher.Watcher, error)`
- **说明**: New creates a new Watcher
- **被调用**: `watchCmd$1`
- **调用**: `addDirs`

#### `WithDebounceDelay`

- **位置**: `internal/watcher/watcher.go:42`
- **签名**: `func(d time.Duration) github.com/zheng/crag/internal/watcher.WatcherOption`
- **说明**: WithDebounceDelay sets the debounce delay

#### `WithOnAnalysisDone`

- **位置**: `internal/watcher/watcher.go:56`
- **签名**: `func(fn func(nodeCount int64, edgeCount int64, duration time.Duration)) github.com/zheng/crag/internal/watcher.WatcherOption`
- **说明**: WithOnAnalysisDone sets the callback for when analysis completes

#### `WithOnAnalysisStart`

- **位置**: `internal/watcher/watcher.go:49`
- **签名**: `func(fn func()) github.com/zheng/crag/internal/watcher.WatcherOption`
- **说明**: WithOnAnalysisStart sets the callback for when analysis starts

#### `WithOnError`

- **位置**: `internal/watcher/watcher.go:63`
- **签名**: `func(fn func(error)) github.com/zheng/crag/internal/watcher.WatcherOption`
- **说明**: WithOnError sets the callback for errors

---

## 修改影响速查

| 函数 | 位置 | 被调用次数 | 调用次数 | 风险 |
|------|------|-----------|----------|------|
| `getShortDisplayName` | internal/export/exporter.go:452 | 11 | 0 | 🔴 高 |
| `nodeID` | internal/mcp/server.go:627 | 9 | 1 | 🔴 高 |
| `GetDirectCallers` | internal/storage/queries.go:65 | 8 | 1 | 🔴 高 |
| `GetDirectCallees` | internal/storage/queries.go:81 | 8 | 1 | 🔴 高 |
| `writeJSON` | internal/web/server.go:308 | 7 | 0 | 🔴 高 |
| `scanNodes` | internal/storage/queries.go:382 | 7 | 0 | 🔴 高 |
| `FindNodesByPattern` | internal/storage/queries.go:52 | 6 | 1 | 🔴 高 |
| `shortName` | internal/mcp/server.go:587 | 5 | 6 | 🔴 高 |
| `nodesToData` | internal/web/server.go:246 | 5 | 1 | 🔴 高 |
| `GetAllFunctions` | internal/storage/queries.go:228 | 5 | 2 | 🔴 高 |
| `Close` | internal/storage/db.go:42 | 5 | 0 | 🔴 高 |
| `Open` | internal/storage/db.go:20 | 5 | 0 | 🔴 高 |
| `nodeToData` | internal/web/server.go:232 | 5 | 2 | 🔴 高 |
| `GetDownstreamCallees` | internal/storage/queries.go:148 | 4 | 1 | 🟡 中 |
| `getRelativePath` | internal/export/exporter.go:505 | 4 | 0 | 🟡 中 |
| `isKeyFunction` | internal/export/exporter.go:516 | 4 | 1 | 🟡 中 |
| `GetUpstreamCallers` | internal/storage/queries.go:98 | 4 | 1 | 🟡 中 |
| `joinStrings` | internal/storage/queries.go:353 | 4 | 0 | 🟡 中 |
| `lastIndex` | internal/mcp/server.go:641 | 4 | 0 | 🟡 中 |
| `GetStats` | internal/storage/queries.go:344 | 4 | 0 | 🟡 中 |
| `sendResult` | internal/mcp/server.go:659 | 3 | 1 | 🟡 中 |
| `makeNodeID` | internal/export/exporter.go:494 | 3 | 1 | 🟡 中 |
| `sendError` | internal/mcp/server.go:668 | 3 | 1 | 🟡 中 |
| `isExportedFunc` | internal/export/exporter.go:540 | 2 | 1 | 🟢 |
| `Build` | internal/graph/builder.go:59 | 2 | 2 | 🟢 |
| `BuildCallGraph` | internal/analyzer/callgraph.go:12 | 2 | 0 | 🟢 |
| `scanNode` | internal/storage/queries.go:366 | 2 | 0 | 🟢 |
| `FilterMainPackages` | internal/analyzer/loader.go:47 | 2 | 0 | 🟢 |
| `NewBuilder` | internal/graph/builder.go:25 | 2 | 0 | 🟢 |
| `indexOf` | internal/mcp/server.go:650 | 2 | 0 | 🟢 |
| `matchPackageSuffix` | internal/export/exporter.go:571 | 2 | 0 | 🟢 |
| `getLayerOrder` | internal/export/exporter.go:408 | 2 | 0 | 🟢 |
| `LoadPackages` | internal/analyzer/loader.go:10 | 2 | 0 | 🟢 |
| `Clear` | internal/storage/db.go:47 | 2 | 0 | 🟢 |
| `GetNodeByID` | internal/storage/queries.go:43 | 2 | 1 | 🟢 |
| `BuildSSA` | internal/analyzer/ssa.go:10 | 2 | 0 | 🟢 |
| `send` | internal/mcp/server.go:677 | 2 | 0 | 🟢 |
| `AnalyzeImpact` | internal/impact/analyzer.go:31 | 1 | 6 | 🟢 |
| `New` | internal/watcher/watcher.go:70 | 1 | 1 | 🟢 |
| `eventLoop` | internal/watcher/watcher.go:130 | 1 | 1 | 🟢 |
| `handleRequest` | internal/mcp/server.go:136 | 1 | 4 | 🟢 |
| `GetNodeByName` | internal/storage/queries.go:34 | 1 | 1 | 🟢 |
| `handleToolsList` | internal/mcp/server.go:165 | 1 | 1 | 🟢 |
| `serveCmd` | main.go:166 | 1 | 0 | 🟢 |
| `groupByPackage` | internal/export/exporter.go:388 | 1 | 0 | 🟢 |
| `watchCmd` | main.go:204 | 1 | 0 | 🟢 |
| `isProjectFunction` | internal/graph/builder.go:50 | 1 | 0 | 🟢 |
| `getPackageGroup` | internal/web/server.go:287 | 1 | 0 | 🟢 |
| `shortName` | internal/web/server.go:254 | 1 | 0 | 🟢 |
| `printhi1` | internal/storage/queries.go:240 | 1 | 0 | 🟢 |
| `getLayerDisplayName` | internal/export/exporter.go:425 | 1 | 0 | 🟢 |
| `exportCmd` | main.go:116 | 1 | 0 | 🟢 |
| `Run` | internal/mcp/server.go:113 | 1 | 2 | 🟢 |
| `getShortPackageName` | internal/export/exporter.go:444 | 1 | 0 | 🟢 |
| `hi` | internal/storage/db.go:58 | 1 | 0 | 🟢 |
| `addDirs` | internal/watcher/watcher.go:99 | 1 | 0 | 🟢 |
| `handleToolsCall` | internal/mcp/server.go:271 | 1 | 8 | 🟢 |
| `writeImpactTable` | internal/export/exporter.go:252 | 1 | 4 | 🟢 |
| `handleInitialize` | internal/mcp/server.go:151 | 1 | 1 | 🟢 |
| `DefaultExportOptions` | internal/export/exporter.go:34 | 1 | 0 | 🟢 |
| `toolList` | internal/mcp/server.go:425 | 1 | 1 | 🟢 |
| `categorizePackages` | internal/export/exporter.go:364 | 1 | 0 | 🟢 |
| `writeProjectStructure` | internal/export/exporter.go:82 | 1 | 0 | 🟢 |
| `getDocComment` | internal/graph/builder.go:162 | 1 | 0 | 🟢 |
| `getSortedPackageNames` | internal/export/exporter.go:396 | 1 | 0 | 🟢 |
| `NewServer` | internal/mcp/server.go:22 | 1 | 0 | 🟢 |
| `Start` | internal/watcher/watcher.go:119 | 1 | 1 | 🟢 |
| `GetAllEdges` | internal/storage/queries.go:246 | 1 | 0 | 🟢 |
| `NewServer` | internal/web/server.go:27 | 1 | 0 | 🟢 |
| `toolUpstream` | internal/mcp/server.go:320 | 1 | 2 | 🟢 |
| `Run` | internal/web/server.go:67 | 1 | 0 | 🟢 |
| `writeArchitectureDiagram` | internal/export/exporter.go:117 | 1 | 10 | 🟢 |
| `toolSearch` | internal/mcp/server.go:400 | 1 | 1 | 🟢 |
| `toolImpact` | internal/mcp/server.go:305 | 1 | 3 | 🟢 |
| `handleEvent` | internal/watcher/watcher.go:154 | 1 | 0 | 🟢 |
| `toolDownstream` | internal/mcp/server.go:360 | 1 | 2 | 🟢 |
| `runAnalysis` | internal/watcher/watcher.go:227 | 1 | 10 | 🟢 |
| `mcpCmd` | main.go:187 | 1 | 0 | 🟢 |
| `writePackageSection` | internal/export/exporter.go:170 | 1 | 12 | 🟢 |
| `createFunctionNode` | internal/graph/builder.go:130 | 1 | 1 | 🟢 |
| `toolMermaid` | internal/mcp/server.go:460 | 1 | 20 | 🟢 |
| `indexCmd` | main.go:46 | 1 | 0 | 🟢 |
| `NewExporter` | internal/export/exporter.go:21 | 1 | 0 | 🟢 |
| `NewAnalyzer` | internal/impact/analyzer.go:17 | 1 | 0 | 🟢 |
| `truncateDoc` | internal/export/exporter.go:559 | 1 | 0 | 🟢 |
| `Export` | internal/export/exporter.go:44 | 1 | 8 | 🟢 |
| `FormatMarkdown` | internal/impact/analyzer.go:109 | 1 | 0 | 🟢 |
