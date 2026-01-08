# 项目调用图谱 (RAG)

> 生成时间: 2026-01-09 00:37:52
> 函数节点: 84 | 调用边: 125

## 项目结构

```
├── crag/
│   ├── cmd/
│   │   ├── crag/
│   ├── internal/
│   │   ├── analyzer/
│   │   ├── export/
│   │   ├── graph/
│   │   ├── impact/
│   │   ├── storage/
```

## 架构图

```mermaid
flowchart TB
    subgraph cmd [CLI 命令层]
    end

    subgraph analyzer [静态分析层]
        FilterMainPackages[FilterMainPackages]
        GetCallGraphStats[GetCallGraphStats]
        GetAllFunctions[GetAllFunctions]
        GetGitChanges[GetGitChanges]
        BuildCallGraph[BuildCallGraph]
        GetChangedPackagePatterns[GetChangedPackagePatterns]
        BuildSSA[BuildSSA]
        HasChanges[HasChanges]
        LoadPackages[LoadPackages]
        String[String]
    end

    subgraph graph [图构建层]
        Build[Build]
        GetNodeCount[GetNodeCount]
        NewBuilder[NewBuilder]
    end

    subgraph storage [存储层]
        GetNodeByID[GetNodeByID]
        Close[Close]
        DeleteNodesByPackage[DeleteNodesByPackage]
        GetDirectCallees[GetDirectCallees]
        Clear[Clear]
        GetStats[GetStats]
        GetUpstreamCallers[GetUpstreamCallers]
        GetAllFunctions[GetAllFunctions]
        FindNodesByPattern[FindNodesByPattern]
        GetCallEdgesForNode[GetCallEdgesForNode]
        GetNodesByPackage[GetNodesByPackage]
        InsertEdge[InsertEdge]
        Open[Open]
        GetDirectCallers[GetDirectCallers]
        Conn[Conn]
        GetDownstreamCallees[GetDownstreamCallees]
        GetNodeByName[GetNodeByName]
        InsertNode[InsertNode]
        DeleteOrphanEdges[DeleteOrphanEdges]
    end

    subgraph impact [影响分析层]
        AnalyzeImpact[AnalyzeImpact]
        FormatMarkdown[FormatMarkdown]
        NewAnalyzer[NewAnalyzer]
        Summary[Summary]
    end

    subgraph export [导出层]
        ExportIncremental[ExportIncremental]
        DefaultExportOptions[DefaultExportOptions]
        NewExporter[NewExporter]
        Export[Export]
    end

    % 关键调用关系
    ExportIncremental --> GetAllFunctions
    ExportIncremental --> GetDirectCallers
    Export --> GetAllFunctions
    Export --> GetStats
    AnalyzeImpact --> GetNodeByName
    AnalyzeImpact --> FindNodesByPattern
    AnalyzeImpact --> GetDirectCallers
    AnalyzeImpact --> GetUpstreamCallers
    AnalyzeImpact --> GetDirectCallees
    AnalyzeImpact --> GetDownstreamCallees
```

---

## 模块详解

### 📦 cmd/crag

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `analyzeCmd` | - | 1 | 0 |
| `analyzeCmd$1` | - | 0 | 14 |
| `downstreamCmd` | - | 1 | 0 |
| `downstreamCmd$1` | - | 0 | 5 |
| `exportCmd` | - | 1 | 0 |
| `exportCmd$1` | - | 0 | 8 |
| `impactCmd` | - | 1 | 0 |
| `impactCmd$1` | - | 0 | 8 |
| `listCmd` | - | 1 | 0 |
| `listCmd$1` | - | 0 | 3 |
| `main` | - | 0 | 7 |
| `outputJSON` | - | 3 | 0 |
| `searchCmd` | - | 1 | 0 |
| `searchCmd$1` | - | 0 | 3 |
| `upstreamCmd` | - | 1 | 0 |
| `upstreamCmd$1` | - | 0 | 5 |

### 📦 internal/analyzer

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `GetChangedPackagePatterns` | GetChangedPackagePatterns r... | 0 | 0 |
| `HasChanges` | HasChanges returns true if ... | 2 | 0 |
| `String` | String returns a summary st... | 0 | 0 |
| `BuildCallGraph` | BuildCallGraph builds the c... | 1 | 0 |
| `BuildSSA` | BuildSSA builds the SSA rep... | 1 | 0 |
| `FilterMainPackages` | FilterMainPackages filters ... | 1 | 0 |
| `GetAllFunctions` | GetAllFunctions returns all... | 0 | 0 |
| `GetCallGraphStats` | GetCallGraphStats returns s... | 1 | 0 |
| `GetGitChanges` | GetGitChanges returns the l... | 2 | 0 |
| `LoadPackages` | LoadPackages loads all Go p... | 1 | 0 |

#### `GetChangedPackagePatterns`

- **位置**: `internal/analyzer/git.go:95`
- **签名**: `func() []string`
- **说明**: GetChangedPackagePatterns returns package patterns for go/packages.Load

#### `HasChanges`

- **位置**: `internal/analyzer/git.go:85`
- **签名**: `func() bool`
- **说明**: HasChanges returns true if there are any Go file changes
- **被调用**: `exportCmd$1`, `analyzeCmd$1`

#### `String`

- **位置**: `internal/analyzer/git.go:90`
- **签名**: `func() string`
- **说明**: String returns a summary string of the changes

#### `BuildCallGraph`

- **位置**: `internal/analyzer/callgraph.go:12`
- **签名**: `func(prog *golang.org/x/tools/go/ssa.Program) (*golang.org/x/tools/go/callgraph.Graph, error)`
- **说明**: BuildCallGraph builds the call graph using VTA (Variable Type Analysis)
VTA is more precise than other algorithms for handling interface calls
- **被调用**: `analyzeCmd$1`

#### `BuildSSA`

- **位置**: `internal/analyzer/ssa.go:10`
- **签名**: `func(pkgs []*golang.org/x/tools/go/packages.Package) (*golang.org/x/tools/go/ssa.Program, []*golang.org/x/tools/go/ssa.Package)`
- **说明**: BuildSSA builds the SSA representation for the given packages
- **被调用**: `analyzeCmd$1`

#### `FilterMainPackages`

- **位置**: `internal/analyzer/loader.go:47`
- **签名**: `func(pkgs []*golang.org/x/tools/go/packages.Package) []*golang.org/x/tools/go/packages.Package`
- **说明**: FilterMainPackages filters packages to only include those with source files
- **被调用**: `analyzeCmd$1`

#### `GetAllFunctions`

- **位置**: `internal/analyzer/ssa.go:21`
- **签名**: `func(prog *golang.org/x/tools/go/ssa.Program) map[*golang.org/x/tools/go/ssa.Function]bool`
- **说明**: GetAllFunctions returns all functions from the SSA program

#### `GetCallGraphStats`

- **位置**: `internal/analyzer/callgraph.go:29`
- **签名**: `func(cg *golang.org/x/tools/go/callgraph.Graph) github.com/zheng/crag/internal/analyzer.CallGraphStats`
- **说明**: GetCallGraphStats returns statistics about the call graph
- **被调用**: `analyzeCmd$1`

#### `GetGitChanges`

- **位置**: `internal/analyzer/git.go:21`
- **签名**: `func(projectPath string, base string) (*github.com/zheng/crag/internal/analyzer.GitChanges, error)`
- **说明**: GetGitChanges returns the list of changed Go files since the last commit
If base is empty, it compares with HEAD (uncommitted changes)
If base is "HEAD~1", it compares with the previous commit
- **被调用**: `exportCmd$1`, `analyzeCmd$1`

#### `LoadPackages`

- **位置**: `internal/analyzer/loader.go:10`
- **签名**: `func(projectPath string) ([]*golang.org/x/tools/go/packages.Package, error)`
- **说明**: LoadPackages loads all Go packages from the given project path
- **被调用**: `analyzeCmd$1`

### 📦 internal/graph

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Build` | Build processes the call gr... | 1 | 2 |
| `GetNodeCount` | GetNodeCount returns the nu... | 1 | 0 |
| `NewBuilder` | NewBuilder creates a new gr... | 1 | 0 |
| `createFunctionNode` | createFunctionNode creates ... | 1 | 1 |
| `getDocComment` | getDocComment extracts the ... | 1 | 0 |
| `isProjectFunction` | isProjectFunction checks if... | 1 | 0 |

#### `Build`

- **位置**: `internal/graph/builder.go:59`
- **签名**: `func(cg *golang.org/x/tools/go/callgraph.Graph) error`
- **说明**: Build processes the call graph and stores nodes/edges
- **被调用**: `analyzeCmd$1`
- **调用**: `isProjectFunction`, `createFunctionNode`

#### `GetNodeCount`

- **位置**: `internal/graph/builder.go:188`
- **签名**: `func() int`
- **说明**: GetNodeCount returns the number of nodes created
- **被调用**: `analyzeCmd$1`

#### `NewBuilder`

- **位置**: `internal/graph/builder.go:25`
- **签名**: `func(fset *go/token.FileSet, pkgs []*golang.org/x/tools/go/packages.Package, insertFn func(*github.com/zheng/crag/internal/graph.Node) (int64, error), edgeFn func(*github.com/zheng/crag/internal/graph.Edge) error) *github.com/zheng/crag/internal/graph.Builder`
- **说明**: NewBuilder creates a new graph builder
- **被调用**: `analyzeCmd$1`

### 📦 internal/storage

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Clear` | Clear removes all data from... | 1 | 0 |
| `Close` | Close closes the database c... | 7 | 0 |
| `Conn` | Conn returns the underlying... | 0 | 1 |
| `DeleteNodesByPackage` | DeleteNodesByPackage delete... | 0 | 3 |
| `DeleteOrphanEdges` | DeleteOrphanEdges deletes e... | 0 | 0 |
| `FindNodesByPattern` | FindNodesByPattern returns ... | 2 | 1 |
| `GetAllFunctions` | GetAllFunctions returns all... | 3 | 1 |
| `GetCallEdgesForNode` | GetCallEdgesForNode returns... | 0 | 0 |
| `GetDirectCallees` | GetDirectCallees returns fu... | 5 | 1 |
| `GetDirectCallers` | GetDirectCallers returns fu... | 5 | 1 |
| `GetDownstreamCallees` | GetDownstreamCallees return... | 1 | 1 |
| `GetNodeByID` | GetNodeByID returns a node ... | 0 | 1 |
| `GetNodeByName` | GetNodeByName returns a nod... | 1 | 1 |
| `GetNodesByPackage` | GetNodesByPackage returns a... | 0 | 2 |
| `GetStats` | GetStats returns database s... | 2 | 0 |
| `GetUpstreamCallers` | GetUpstreamCallers returns ... | 1 | 1 |
| `InsertEdge` | InsertEdge inserts an edge ... | 0 | 0 |
| `InsertNode` | InsertNode inserts a node i... | 0 | 0 |
| `Open` | Open opens or creates a SQL... | 7 | 0 |
| `hi` | - | 1 | 0 |
| `joinStrings` | - | 4 | 0 |
| `scanNode` | - | 2 | 0 |
| `scanNodes` | - | 7 | 0 |

#### `Clear`

- **位置**: `internal/storage/db.go:47`
- **签名**: `func() error`
- **说明**: Clear removes all data from the database
- **被调用**: `analyzeCmd$1`

#### `Close`

- **位置**: `internal/storage/db.go:42`
- **签名**: `func() error`
- **说明**: Close closes the database connection
- **被调用**: `exportCmd$1`, `upstreamCmd$1`, `analyzeCmd$1`, `searchCmd$1`, `impactCmd$1`, `downstreamCmd$1`, `listCmd$1`

#### `Conn`

- **位置**: `internal/storage/db.go:53`
- **签名**: `func() *database/sql.DB`
- **说明**: Conn returns the underlying database connection for advanced queries
- **调用**: `hi`

#### `DeleteNodesByPackage`

- **位置**: `internal/storage/queries.go:241`
- **签名**: `func(packages []string) (int64, error)`
- **说明**: DeleteNodesByPackage deletes all nodes belonging to the specified packages
Also deletes all edges referencing those nodes
Returns the number of deleted nodes
- **调用**: `joinStrings`, `joinStrings`, `joinStrings`

#### `DeleteOrphanEdges`

- **位置**: `internal/storage/queries.go:273`
- **签名**: `func() (int64, error)`
- **说明**: DeleteOrphanEdges deletes edges that reference non-existent nodes

#### `FindNodesByPattern`

- **位置**: `internal/storage/queries.go:51`
- **签名**: `func(pattern string) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: FindNodesByPattern returns nodes matching a name pattern (using LIKE)
- **被调用**: `searchCmd$1`, `AnalyzeImpact`
- **调用**: `scanNodes`

#### `GetAllFunctions`

- **位置**: `internal/storage/queries.go:227`
- **签名**: `func() ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetAllFunctions returns all function nodes
- **被调用**: `Export`, `listCmd$1`, `ExportIncremental`
- **调用**: `scanNodes`

#### `GetCallEdgesForNode`

- **位置**: `internal/storage/queries.go:196`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Edge, error)`
- **说明**: GetCallEdgesForNode returns all call edges where the node is the caller

#### `GetDirectCallees`

- **位置**: `internal/storage/queries.go:80`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDirectCallees returns functions that the given function directly calls
- **被调用**: `writeImpactTable`, `writePackageSection`, `writePackageSection`, `AnalyzeImpact`, `writeArchitectureDiagram`
- **调用**: `scanNodes`

#### `GetDirectCallers`

- **位置**: `internal/storage/queries.go:64`
- **签名**: `func(nodeID int64) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDirectCallers returns functions that directly call the given function
- **被调用**: `writeImpactTable`, `writePackageSection`, `writePackageSection`, `ExportIncremental`, `AnalyzeImpact`
- **调用**: `scanNodes`

#### `GetDownstreamCallees`

- **位置**: `internal/storage/queries.go:147`
- **签名**: `func(nodeID int64, maxDepth int) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetDownstreamCallees returns all downstream callees recursively up to maxDepth
If maxDepth is 0, it returns all callees with no depth limit
- **被调用**: `AnalyzeImpact`
- **调用**: `scanNodes`

#### `GetNodeByID`

- **位置**: `internal/storage/queries.go:42`
- **签名**: `func(id int64) (*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodeByID returns a node by its ID
- **调用**: `scanNode`

#### `GetNodeByName`

- **位置**: `internal/storage/queries.go:33`
- **签名**: `func(name string) (*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodeByName returns a node by its fully qualified name
- **被调用**: `AnalyzeImpact`
- **调用**: `scanNode`

#### `GetNodesByPackage`

- **位置**: `internal/storage/queries.go:286`
- **签名**: `func(packages []string) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetNodesByPackage returns all nodes in the specified packages
- **调用**: `joinStrings`, `scanNodes`

#### `GetStats`

- **位置**: `internal/storage/queries.go:308`
- **签名**: `func() (nodeCount int64, edgeCount int64, err error)`
- **说明**: GetStats returns database statistics
- **被调用**: `Export`, `analyzeCmd$1`

#### `GetUpstreamCallers`

- **位置**: `internal/storage/queries.go:97`
- **签名**: `func(nodeID int64, maxDepth int) ([]*github.com/zheng/crag/internal/graph.Node, error)`
- **说明**: GetUpstreamCallers returns all upstream callers recursively up to maxDepth
If maxDepth is 0, it returns all callers with no depth limit
- **被调用**: `AnalyzeImpact`
- **调用**: `scanNodes`

#### `InsertEdge`

- **位置**: `internal/storage/queries.go:23`
- **签名**: `func(edge *github.com/zheng/crag/internal/graph.Edge) error`
- **说明**: InsertEdge inserts an edge into the database

#### `InsertNode`

- **位置**: `internal/storage/queries.go:10`
- **签名**: `func(node *github.com/zheng/crag/internal/graph.Node) (int64, error)`
- **说明**: InsertNode inserts a node into the database and returns its ID

#### `Open`

- **位置**: `internal/storage/db.go:20`
- **签名**: `func(path string) (*github.com/zheng/crag/internal/storage.DB, error)`
- **说明**: Open opens or creates a SQLite database at the given path
- **被调用**: `exportCmd$1`, `upstreamCmd$1`, `analyzeCmd$1`, `searchCmd$1`, `impactCmd$1`, `downstreamCmd$1`, `listCmd$1`

### 📦 internal/impact

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `AnalyzeImpact` | AnalyzeImpact analyzes the ... | 3 | 6 |
| `FormatMarkdown` | FormatMarkdown formats the ... | 2 | 0 |
| `Summary` | FormatJSON formats the impa... | 1 | 0 |
| `NewAnalyzer` | NewAnalyzer creates a new i... | 3 | 0 |

#### `AnalyzeImpact`

- **位置**: `internal/impact/analyzer.go:31`
- **签名**: `func(funcName string, upstreamDepth int, downstreamDepth int) (*github.com/zheng/crag/internal/impact.ImpactReport, error)`
- **说明**: AnalyzeImpact analyzes the impact of changing a function
- **被调用**: `upstreamCmd$1`, `impactCmd$1`, `downstreamCmd$1`
- **调用**: `GetNodeByName`, `FindNodesByPattern`, `GetDirectCallers`, `GetUpstreamCallers`, `GetDirectCallees`, `GetDownstreamCallees`

#### `FormatMarkdown`

- **位置**: `internal/impact/analyzer.go:109`
- **签名**: `func() string`
- **说明**: FormatMarkdown formats the impact report as markdown
- **被调用**: `impactCmd$1`, `impactCmd$1`

#### `Summary`

- **位置**: `internal/impact/analyzer.go:175`
- **签名**: `func() string`
- **说明**: FormatJSON formats the impact report as JSON (use encoding/json for actual serialization)
- **被调用**: `impactCmd$1`

#### `NewAnalyzer`

- **位置**: `internal/impact/analyzer.go:17`
- **签名**: `func(db *github.com/zheng/crag/internal/storage.DB) *github.com/zheng/crag/internal/impact.Analyzer`
- **说明**: NewAnalyzer creates a new impact analyzer
- **被调用**: `upstreamCmd$1`, `impactCmd$1`, `downstreamCmd$1`

### 📦 internal/export

| 函数 | 说明 | 被调用 | 调用 |
|------|------|--------|------|
| `Export` | Export generates a complete... | 1 | 8 |
| `ExportIncremental` | ExportIncremental generates... | 1 | 7 |
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
- **被调用**: `exportCmd$1`
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

---

## 修改影响速查

| 函数 | 位置 | 被调用次数 | 调用次数 | 风险 |
|------|------|-----------|----------|------|
| `getShortDisplayName` | internal/export/exporter.go:452 | 11 | 0 | 🔴 高 |
| `scanNodes` | internal/storage/queries.go:346 | 7 | 0 | 🔴 高 |
| `Close` | internal/storage/db.go:42 | 7 | 0 | 🔴 高 |
| `Open` | internal/storage/db.go:20 | 7 | 0 | 🔴 高 |
| `GetDirectCallers` | internal/storage/queries.go:64 | 5 | 1 | 🔴 高 |
| `GetDirectCallees` | internal/storage/queries.go:80 | 5 | 1 | 🔴 高 |
| `isKeyFunction` | internal/export/exporter.go:516 | 4 | 1 | 🟡 中 |
| `getRelativePath` | internal/export/exporter.go:505 | 4 | 0 | 🟡 中 |
| `joinStrings` | internal/storage/queries.go:317 | 4 | 0 | 🟡 中 |
| `GetAllFunctions` | internal/storage/queries.go:227 | 3 | 1 | 🟡 中 |
| `AnalyzeImpact` | internal/impact/analyzer.go:31 | 3 | 6 | 🟡 中 |
| `makeNodeID` | internal/export/exporter.go:494 | 3 | 1 | 🟡 中 |
| `NewAnalyzer` | internal/impact/analyzer.go:17 | 3 | 0 | 🟡 中 |
| `outputJSON` | cmd/crag/main.go:407 | 3 | 0 | 🟡 中 |
| `GetGitChanges` | internal/analyzer/git.go:21 | 2 | 0 | 🟢 |
| `getLayerOrder` | internal/export/exporter.go:408 | 2 | 0 | 🟢 |
| `FindNodesByPattern` | internal/storage/queries.go:51 | 2 | 1 | 🟢 |
| `FormatMarkdown` | internal/impact/analyzer.go:109 | 2 | 0 | 🟢 |
| `matchPackageSuffix` | internal/export/exporter.go:571 | 2 | 0 | 🟢 |
| `GetStats` | internal/storage/queries.go:308 | 2 | 0 | 🟢 |
| `isExportedFunc` | internal/export/exporter.go:540 | 2 | 1 | 🟢 |
| `HasChanges` | internal/analyzer/git.go:85 | 2 | 0 | 🟢 |
| `scanNode` | internal/storage/queries.go:330 | 2 | 0 | 🟢 |
| `Build` | internal/graph/builder.go:59 | 1 | 2 | 🟢 |
| `GetNodeCount` | internal/graph/builder.go:188 | 1 | 0 | 🟢 |
| `exportCmd` | cmd/crag/main.go:413 | 1 | 0 | 🟢 |
| `searchCmd` | cmd/crag/main.go:371 | 1 | 0 | 🟢 |
| `listCmd` | cmd/crag/main.go:332 | 1 | 0 | 🟢 |
| `NewExporter` | internal/export/exporter.go:21 | 1 | 0 | 🟢 |
| `getDocComment` | internal/graph/builder.go:162 | 1 | 0 | 🟢 |
| `GetUpstreamCallers` | internal/storage/queries.go:97 | 1 | 1 | 🟢 |
| `analyzeCmd` | cmd/crag/main.go:46 | 1 | 0 | 🟢 |
| `getSortedPackageNames` | internal/export/exporter.go:396 | 1 | 0 | 🟢 |
| `writeProjectStructure` | internal/export/exporter.go:82 | 1 | 0 | 🟢 |
| `DefaultExportOptions` | internal/export/exporter.go:34 | 1 | 0 | 🟢 |
| `truncateDoc` | internal/export/exporter.go:559 | 1 | 0 | 🟢 |
| `createFunctionNode` | internal/graph/builder.go:130 | 1 | 1 | 🟢 |
| `Export` | internal/export/exporter.go:44 | 1 | 8 | 🟢 |
| `Clear` | internal/storage/db.go:47 | 1 | 0 | 🟢 |
| `writeArchitectureDiagram` | internal/export/exporter.go:117 | 1 | 10 | 🟢 |
| `BuildSSA` | internal/analyzer/ssa.go:10 | 1 | 0 | 🟢 |
| `downstreamCmd` | cmd/crag/main.go:223 | 1 | 0 | 🟢 |
| `BuildCallGraph` | internal/analyzer/callgraph.go:12 | 1 | 0 | 🟢 |
| `impactCmd` | cmd/crag/main.go:286 | 1 | 0 | 🟢 |
| `ExportIncremental` | internal/export/exporter.go:297 | 1 | 7 | 🟢 |
| `LoadPackages` | internal/analyzer/loader.go:10 | 1 | 0 | 🟢 |
| `getLayerDisplayName` | internal/export/exporter.go:425 | 1 | 0 | 🟢 |
| `upstreamCmd` | cmd/crag/main.go:160 | 1 | 0 | 🟢 |
| `getShortPackageName` | internal/export/exporter.go:444 | 1 | 0 | 🟢 |
| `hi` | internal/storage/db.go:58 | 1 | 0 | 🟢 |
| `GetCallGraphStats` | internal/analyzer/callgraph.go:29 | 1 | 0 | 🟢 |
| `writeImpactTable` | internal/export/exporter.go:252 | 1 | 4 | 🟢 |
| `isProjectFunction` | internal/graph/builder.go:50 | 1 | 0 | 🟢 |
| `Summary` | internal/impact/analyzer.go:175 | 1 | 0 | 🟢 |
| `groupByPackage` | internal/export/exporter.go:388 | 1 | 0 | 🟢 |
| `FilterMainPackages` | internal/analyzer/loader.go:47 | 1 | 0 | 🟢 |
| `categorizePackages` | internal/export/exporter.go:364 | 1 | 0 | 🟢 |
| `writePackageSection` | internal/export/exporter.go:170 | 1 | 12 | 🟢 |
| `GetDownstreamCallees` | internal/storage/queries.go:147 | 1 | 1 | 🟢 |
| `GetNodeByName` | internal/storage/queries.go:33 | 1 | 1 | 🟢 |
| `NewBuilder` | internal/graph/builder.go:25 | 1 | 0 | 🟢 |
