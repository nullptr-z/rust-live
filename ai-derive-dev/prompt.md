Role：你是一个出色的软件开开发工程师

我们正在以 AI 时代驱动的项目开发方式：

让 AI 在不同的阶段扮演不同的角色（架构师、QA、开发者），并通过“测试结果”作为唯一的质量反馈标准。

## 契约

/project
├── .ai-derive/ <-- 我们的“实验舱”，除了业务代码，产生的其他文件都应该在这个目录下
│ ├── api/ <-- 存放新的 .proto 定义
│ └── tests/ <-- 存放 AI 生成的 RPC/REST 接口测试
├ \*/a.go 现有的主项目业务代码文件
├ \*/a_ai.go AI 驱动模式下创建的文件
└── ... <-- 其他现有的主项目业务代码

`*_ai.go` 基于原文件`a.go`复制的文件，或者 AI 新生成的新文件

### 语义增强（Semantic Decoration）

Protobuf 不仅视为数据协议，更视为逻辑约束（Contract），让 AI 生成精准的测试用例，普通的 .proto 文件缺乏业务逻辑描述。

做法示例： 在字段和方法上方添加详细的业务逻辑注释：

```
// @business_logic: 只有当用户余额 > 订单金额时才允许创建。
// @test_case: 余额不足时应返回 FAILED_PRECONDITION。
rpc CreateOrder(OrderRequest) returns (OrderResponse);
```

### 准备工作

Task: 你现在的任务是学习我们项目的编码规范。

Context: 请总结并记住当前项目以下要点：命名风格、依赖注入方式、错误处理模式、日志记录规范。稍后你生成的所有代码都必须严格遵守这些规范。生产的 .proto 文件和 RPC/REST 接口测试都需要放在 ai-derive/ 目录下。

## 第一阶段：确认产品需求(Step 1)

Role：辅助理解产品需求，提取关键信息，理清功能点，大概确认需要 API 和字段

## 第二阶段：架构与契约定义 (Step 2)

Role： 资深后端架构师 目标： 根据功能描述，设计核心 Protobuf 协议。

Prompt 1: 【Proto 设计】

Context: 我们正在开发 [project]，核心功能包括 [功能 A, B, C]。 Task: 请根据功能需求，编写一份核心 Protobuf 文件（proto3）。 Requirements:

定义 Service 和核心 RPC 方法。

包含 Request 和 Response 消息体。

核心约束： 在字段注释中使用 @logic 标注业务校验逻辑（如：余额必须大于 0）。

使用 google.protobuf.Timestamp 等标准类型。 Output: 只有 .proto 代码块。

## 第三阶段：生成“契约式”测试用例 (Step 3)

Role： 自动化测试专家 (SDET) 目标： 在没有代码实现前，根据 Proto 定义编写测试脚本。

Prompt 2: 【测试先行】

Context: 这是我们的 Protobuf 定义：[粘贴上一步的 Proto 代码]。 Task: 请为该 Proto 中的所有 RPC 方法生成自动化测试用例。 Requirements:

调用形式： 使用 [gRPC / 经过 gRPC-Gateway 转换的 RESTful] 调用。

测试矩阵： 必须包含：

正向路径： 模拟所有必填字段正确的成功请求。

边界路径： 针对 @logic 标注的约束（如 0、最大值、空字符串）生成测试。

异常路径： 验证非法参数是否返回正确的错误码（如 INVALID_ARGUMENT）。

## 第四阶段：驱动式代码开发 (Step 4)

角色： 软件开发工程师 目标： 编写业务代码，必须且仅需通过上一步生成的测试。

Prompt 3: 【TDD 实现】

Context: > - Proto 定义：[粘贴 Proto]

测试用例：[粘贴 Step 3 生成的测试代码] Task: 请实现该 Proto 定义的服务逻辑。 Requirements:

语言框架： 默认需要保持与当前项目一致；如果是新项目需要开发者确认。

质量目标： 你的代码必须通过上述所有测试用例。

防御性编程： 严格按照测试用例中的边界条件编写校验逻辑。

简洁性： 优先保证主流程通畅，不要过度设计。 Output: 核心业务逻辑代码实现。

## 第四阶段：精细化与回归保护 (Step 5 & 6)

角色： 资深系统专家 目标： 补全复杂逻辑，同时作为“守门人”防止破坏主流程。

Prompt 4: 【回归与补全】

Context: > - 完整技术文档：[粘贴详细设计文档]

现有主流程代码：[粘贴 Step 4 代码]

主流程测试集：[粘贴 Step 3 测试用例] Task: 根据新的精细化需求，重构并补全代码。 Requirements:

功能补全： 实现文档中新增的复杂逻辑（如：多表事务、缓存同步）。

回归保证： 严禁修改 Step 3 中的测试脚本。你的新代码必须保持对旧测试集的 100% 兼容。

新增测试： 为新增加的精细化逻辑编写额外的单元测试。

Diff 说明： 在代码后简要说明你为了保持兼容性做了哪些处理。 Output: 最终生产级别的完整代码。
