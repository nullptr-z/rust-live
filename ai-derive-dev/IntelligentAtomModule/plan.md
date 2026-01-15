---------------
如何给 Claude 发送指令？
你可以复制以下 Prompt 发给 Claude：

“我正在开发一个基于 Python 的智能 API 自动化测试平台。请阅读以下 PRD（需求文档），然后帮我完成 Phase 1: 核心骨架 的代码编写。

要求：

使用 Pydantic 来定义 AtomicModule 和 BlackboardItem 的数据结构。

编写一个 Runner 类，能够接受模块列表并按顺序执行。

暂时不要连接真实的 LLM，AI 部分先用 Print 占位或 Mock 返回。

代码要清晰、有注释，符合生产级规范。
---------------



# PRD: 智能 API 编排与自动化测试平台 (Agentic API Runner)

## 1. 项目概述 (Project Overview)

构建一个非侵入式、配置驱动的 API 自动化测试平台。该平台不依赖额外开发的测试接口，而是将现有的业务 API 封装为原子模块 (Atomic Modules)，通过黑板模式 (Blackboard Pattern) 管理状态，并利用 LLM (大语言模型) 实现参数的自动语义推断与注入。

#### 1.1 核心设计哲学

- One Node = One Request: 一个测试节点严格对应一个 API 请求。
- CURL as Kernel: 底层执行逻辑基于 HTTP/CURL 标准，保证通用性。
- Semantic Wiring: 模块间不再通过硬编码连接（step1.output -> step2.input），而是通过“生产者声明输出、消费者声明需求 + AI 语义匹配”实现动态连接。-- 每个模块都是一个独立的智能体，能够自主寻找所需数据。

#### MVP(Minimum Viable Product)

@blackboard.py

---

### 2. 核心概念与术语 (Core Concepts)

| 术语                           | 定义                                                                                                         |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------ |
| Atomic Module (原子模块)       | 最小执行单元。包含 API 请求模板、输入契约 (Inputs)、输出契约 (Outputs) 和 AI 推理逻辑。                      |
| Blackboard (黑板)              | 全局运行时上下文 (Runtime Context)。用于存储所有模块产生的输出数据。数据以 Key-Value + Metadata 的形式存储。 |
| Workflow (工作流)              | 一组按顺序排列的原子模块配置，描述了一个完整的业务链路（如：登录 -> 浏览 -> 下单）。                         |
| Semantic Resolution (语义解析) | 当显式参数缺失时，利用 LLM 分析黑板中的元数据，自动寻找最佳参数值的过程。                                    |

## 3. 系统架构 (System Architecture)

#### 3.1 技术栈

语言: Python 3.10+

核心库: requests (HTTP 请求), pydantic (数据校验), jinja2 (模板渲染), openai (AI 推理)

配置格式: YAML (定义模块和流程)

#### 3.2 数据结构定义 (Data Schemas)

**A. 原子模块配置 (Module Schema)**

@demo-modul.yaml

```yaml
name: "Create_Order_API"
description: "创建订单接口"
method: "POST"
url: "https://api.domain.com/v1/orders"

# 请求模版 (支持 Jinja2 语法 {{variable}})
headers:
  Authorization: "Bearer {{token}}"
  Content-Type: "application/json"
body:
  user_id: "{{uid}}"
  item_id: "{{product_id}}"
  count: 1

# 输入契约：我需要什么数据才能运行？
inputs:
  - name: "token"
    description: "用户登录凭证"
    required: true
  - name: "uid"
    description: "用户唯一标识"
  - name: "product_id"
    description: "商品ID"

# 输出契约：运行完我会产生什么数据？
outputs:
  - name: "order_id"
    selector: "json.data.order_no" # 支持 JSONPath
    description: "生成的订单编号"
    type: "string"

# 策略配置
strategies:
  ai_fallback: true # 是否开启 AI 自动参数推断
```

**B. 黑板数据项 (Blackboard Item)**

```yaml
class BlackboardItem:
key: str # 变量名 (如 "order_id")
value: Any # 实际值 (如 "ORD_12345")
description: str # 语义描述 (如 "创建成功的订单编号")
source: str # 来源模块名
timestamp: float # 产生时间
```

## 4. 功能需求 (Functional Requirements)

#### 4.1 核心执行引擎 (Execution Engine)

引擎需按顺序遍历 Workflow 中的模块，对每个模块执行标准的 R-R-E-E 循环：

Resolve (参数解析):

- 检查 inputs 定义。
- Level 1 - 精确匹配: 在黑板中查找同名 Key。
- Level 2 - AI 推理 (Fallback): 若未找到且开启 ai_fallback，收集黑板中所有数据的描述，构建 Prompt 询问 LLM：“我需要‘用户 ID’，黑板里哪个值最像？”。
- 若无法解析，抛出 MissingParameterError。

Render (模板渲染):

- 使用解析出的参数字典，渲染 URL、Headers 和 Body 模板。

Execute (执行请求):

- 发送真实的 HTTP 请求。
- 支持 Mock Mode (仅返回预设数据，不发网络请求)。

Extract (结果提取):

- 根据 outputs 中的 selector (JSONPath) 解析 Response。
- 将提取的数据封装为 BlackboardItem 存入黑板。

#### 4.2 AI 智能模块 (AI Logic)

Trigger: 仅在参数缺失时触发。

Prompt 构造逻辑:

- Input: 当前模块需要的参数描述 (target_desc)。
- Context: 黑板中所有可用变量的 key 和 description 列表。
- Output: 只要返回匹配的 Key 名称。

#### 4.3 模块库管理 (Module Repository)

支持从 YAML 文件加载原子模块。

支持将 CURL 命令文本直接转换为原子模块结构（后续功能）。

## 5. 接口与交互设计 (Interface Design)

#### 5.1 CLI 运行器

```bash
# 运行一个定义好的流程
python runner.py run --workflow ./workflows/order_test.yaml --env staging

# 单独调试一个模块
python runner.py debug --module ./modules/login.yaml
```

#### 配置文件示例

```yaml
workflow_name: "电商下单全链路测试"
steps:
  - uses: "./modules/login.yaml"
  - uses: "./modules/get_product.yaml"
  - uses: "./modules/create_order.yaml"
    # 允许人工强制覆盖参数，优先级最高
    with:
      count: 99
```

## 6. 开发路线图 (Roadmap for Claude)

请按以下顺序生成代码：

1. Phase 1: 核心骨架

- 定义 Blackboard 类和 AtomicModule 类。
- 实现 JSONPath 提取逻辑。
- 实现简单的 Jinja2 模板渲染。

2. Phase 2: AI 集成

- 集成 Claude CLI SDK。
- 实现 \_resolve_params 中的 AI Fallback 逻辑。

3. Phase 3: YAML 驱动

- 实现 YAML 读取器，能够解析 Workflow 文件并实例化模块列表。

4. Phase 4: 健壮性增强

- 添加异常处理、重试机制和详细的执行日志。
