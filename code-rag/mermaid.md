# Code-RAG 项目架构图

## 1. 项目整体架构图

```mermaid
flowchart TB
    subgraph CLI["CLI 命令层"]
        main["main()"]
        analyzeCmd["analyzeCmd"]
        upstreamCmd["upstreamCmd"]
        downstreamCmd["downstreamCmd"]
        impactCmd["impactCmd"]
        listCmd["listCmd"]
        searchCmd["searchCmd"]
        exportCmd["exportCmd"]
        mcpCmd["mcpCmd"]
    end

    subgraph Analyzer["analyzer 分析层"]
        LoadPackages["LoadPackages"]
        BuildSSA["BuildSSA"]
        BuildCallGraph["BuildCallGraph"]
        GetCallGraphStats["GetCallGraphStats"]
        FilterMainPackages["FilterMainPackages"]
        GetGitChanges["GetGitChanges"]
    end

    subgraph Graph["graph 图构建层"]
        NewBuilder["NewBuilder"]
        Build["Build"]
        isProjectFunction["isProjectFunction"]
        createFunctionNode["createFunctionNode"]
        getDocComment["getDocComment"]
    end

    subgraph Impact["impact 影响分析层"]
        NewAnalyzer["NewAnalyzer"]
        AnalyzeImpact["AnalyzeImpact"]
        FormatMarkdown["FormatMarkdown"]
        Summary["Summary"]
    end

    subgraph Export["export 导出层"]
        NewExporter["NewExporter"]
        Export["Export"]
        ExportIncremental["ExportIncremental"]
    end

    subgraph MCP["mcp 服务层"]
        NewServer["NewServer"]
        Run["Run"]
        handleRequest["handleRequest"]
        handleToolsCall["handleToolsCall"]
        toolImpact["toolImpact"]
        toolUpstream["toolUpstream"]
        toolDownstream["toolDownstream"]
        toolSearch["toolSearch"]
        toolList["toolList"]
        toolMermaid["toolMermaid"]
    end

    subgraph Storage["storage 存储层"]
        Open["Open"]
        InsertNode["InsertNode"]
        InsertEdge["InsertEdge"]
        GetNodeByName["GetNodeByName"]
        FindNodesByPattern["FindNodesByPattern"]
        GetDirectCallers["GetDirectCallers"]
        GetUpstreamCallers["GetUpstreamCallers"]
        GetDirectCallees["GetDirectCallees"]
        GetDownstreamCallees["GetDownstreamCallees"]
        GetAllFunctions["GetAllFunctions"]
    end

    main --> analyzeCmd & upstreamCmd & downstreamCmd & impactCmd & listCmd & searchCmd & exportCmd & mcpCmd

    analyzeCmd --> LoadPackages --> BuildSSA --> BuildCallGraph
    analyzeCmd --> GetGitChanges
    analyzeCmd --> NewBuilder --> Build
    Build --> isProjectFunction & createFunctionNode
    createFunctionNode --> getDocComment

    impactCmd & upstreamCmd & downstreamCmd --> NewAnalyzer --> AnalyzeImpact
    AnalyzeImpact --> GetNodeByName & FindNodesByPattern & GetDirectCallers & GetUpstreamCallers & GetDirectCallees & GetDownstreamCallees

    exportCmd --> NewExporter --> Export

    mcpCmd --> NewServer --> Run --> handleRequest --> handleToolsCall
    handleToolsCall --> toolImpact & toolUpstream & toolDownstream & toolSearch & toolList & toolMermaid

    toolImpact --> AnalyzeImpact
    toolUpstream --> GetUpstreamCallers
    toolDownstream --> GetDownstreamCallees
    toolSearch --> FindNodesByPattern
    toolList --> GetAllFunctions
```

## 2. MCP 服务器调用流程图

```mermaid
flowchart TB
    Run["🎯 Server.Run"]
    style Run fill:#f96,stroke:#333,stroke-width:2px

    sendError["sendError"]
    handleRequest["handleRequest"]
    send["send"]
    handleInitialize["handleInitialize"]
    handleToolsList["handleToolsList"]
    handleToolsCall["handleToolsCall"]
    sendResult["sendResult"]

    toolImpact["toolImpact"]
    toolUpstream["toolUpstream"]
    toolDownstream["toolDownstream"]
    toolSearch["toolSearch"]
    toolList["toolList"]
    toolMermaid["toolMermaid"]

    Run --> sendError
    Run --> handleRequest
    sendError --> send

    handleRequest --> handleInitialize
    handleRequest --> handleToolsList
    handleRequest --> handleToolsCall
    handleRequest --> sendError

    handleToolsCall --> sendError
    handleToolsCall --> sendResult
    handleToolsCall --> toolImpact
    handleToolsCall --> toolUpstream
    handleToolsCall --> toolDownstream
    handleToolsCall --> toolSearch
    handleToolsCall --> toolList
    handleToolsCall --> toolMermaid

    sendResult --> send
```

## 3. 分析流程图 (analyze 命令)

```mermaid
flowchart TB
    analyzeCmd["🎯 analyzeCmd"]
    style analyzeCmd fill:#f96,stroke:#333,stroke-width:2px

    subgraph Git["Git 检测"]
        GetGitChanges["GetGitChanges"]
        HasChanges["HasChanges"]
    end

    subgraph Load["包加载"]
        LoadPackages["LoadPackages"]
        FilterMainPackages["FilterMainPackages"]
    end

    subgraph SSA["SSA 构建"]
        BuildSSA["BuildSSA"]
        BuildCallGraph["BuildCallGraph"]
        GetCallGraphStats["GetCallGraphStats"]
    end

    subgraph DB["数据库操作"]
        Open["Open"]
        Clear["Clear"]
        Close["Close"]
        GetStats["GetStats"]
    end

    subgraph Graph["图构建"]
        NewBuilder["NewBuilder"]
        Build["Build"]
        GetNodeCount["GetNodeCount"]
        isProjectFunction["isProjectFunction"]
        createFunctionNode["createFunctionNode"]
        getDocComment["getDocComment"]
    end

    analyzeCmd --> GetGitChanges --> HasChanges
    analyzeCmd --> LoadPackages --> FilterMainPackages
    analyzeCmd --> BuildSSA --> BuildCallGraph --> GetCallGraphStats
    analyzeCmd --> Open --> Clear
    analyzeCmd --> NewBuilder --> Build --> GetNodeCount
    Build --> isProjectFunction
    Build --> createFunctionNode --> getDocComment
    analyzeCmd --> GetStats
    analyzeCmd --> Close
```

## 4. 影响分析调用图

```mermaid
flowchart TB
    AnalyzeImpact["🎯 AnalyzeImpact"]
    style AnalyzeImpact fill:#f96,stroke:#333,stroke-width:2px

    GetNodeByName["GetNodeByName"]
    FindNodesByPattern["FindNodesByPattern"]
    GetDirectCallers["GetDirectCallers"]
    GetUpstreamCallers["GetUpstreamCallers"]
    GetDirectCallees["GetDirectCallees"]
    GetDownstreamCallees["GetDownstreamCallees"]
    scanNode["scanNode"]
    scanNodes["scanNodes"]

    AnalyzeImpact --> GetNodeByName
    AnalyzeImpact --> FindNodesByPattern
    AnalyzeImpact --> GetDirectCallers
    AnalyzeImpact --> GetUpstreamCallers
    AnalyzeImpact --> GetDirectCallees
    AnalyzeImpact --> GetDownstreamCallees

    GetNodeByName --> scanNode
    FindNodesByPattern --> scanNodes
    GetDirectCallers --> scanNodes
    GetUpstreamCallers --> scanNodes
    GetDirectCallees --> scanNodes
    GetDownstreamCallees --> scanNodes
```

## 5. 项目模块依赖图

```mermaid
flowchart LR
    subgraph cmd["cmd/crag"]
        main["main.go"]
    end

    subgraph internal["internal/"]
        analyzer["analyzer/"]
        graph["graph/"]
        impact["impact/"]
        export["export/"]
        mcp["mcp/"]
        storage["storage/"]
    end

    main --> analyzer
    main --> graph
    main --> impact
    main --> export
    main --> mcp
    main --> storage

    analyzer --> storage
    graph --> storage
    impact --> storage
    export --> storage
    mcp --> storage
    mcp --> impact
```

---

## 项目说明

这是一个 **Go 代码调用图分析工具 (crag)**，主要功能包括：

| 模块 | 功能 |
|------|------|
| `analyzer` | 使用 SSA 分析 Go 代码，构建函数调用图 |
| `graph` | 将调用图转换为节点和边的数据结构 |
| `storage` | SQLite 数据库存储，支持调用关系查询 |
| `impact` | 分析函数变更的影响范围（上下游调用者） |
| `export` | 导出 Markdown 格式的 RAG 文档 |
| `mcp` | MCP 服务器，允许 AI 助手直接查询调用图 |

