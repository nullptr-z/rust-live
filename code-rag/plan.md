1. 静态分析层 (Graph Construction)
你不能只靠 AI 来猜调用关系，必须结合抽象语法树 (AST)。

提取节点： 扫描项目，提取所有类、函数、接口。

建立边： 识别 call (调用)、inherit (继承)、import (引入) 和 implement (实现)。

工具推荐： 使用 Tree-sitter 或 LSP (Language Server Protocol)。它们能极其精确地告诉你：函数 A 到底在何处被调用。


2. 静态分析与拓扑提取
不要直接把源码喂给 AI 去理解逻辑，要利用 Go 的官方工具包构建确定的依赖图：

解析器： 使用 golang.org/x/tools/go/packages 加载整个项目。它能处理 go.mod 依赖，确保你拿到的类型信息是全量的。

中间表示 (SSA)： 使用 golang.org/x/tools/go/ssa 将代码转为静态单赋值形式。这是构建精确调用图的基础。

构建调用图 (Call Graph)： 使用 golang.org/x/tools/go/callgraph/vta (Variable Type Analysis)。相比传统的算法，VTA 在处理 Go 的 interface 调用时更加精准。

3构建“代码知识图谱”
将提取到的信息存入一个轻量级的图数据库（如内存中的 gonum/graph 或持久化的 BadgerDB），节点和边包含以下元数据：

Node (节点)：

类型：Function, Struct, Interface, Package

属性：源码位置（文件+行号）、签名（参数/返回值）、文档注释（Doc comments）。

Edge (边)：

关系：Calls (调用), Implements (实现接口), References (引用成员), Returns (作为返回值)。
