我们正在以 AI 时代驱动的项目开发方式：

1. 产品初期，确认主要功能（人工）
2. 技术方案粗稿，proto 定义 API 核心字段
3. 根据 2，AI 撰写主流程测试用例，以 restfull、rpc 调用形式
4. AI 根据 2、3 撰写主流程代码，保证 3 全部通过
5. 前后端技术方案对齐（人工），技术精细化，补全 proto
6. AI 根据完整技术文档撰写代码，同时保证新的代码不破坏 3 的主流程

让 AI 在不同的阶段扮演不同的角色（架构师、QA、开发者），并通过“测试结果”作为唯一的质量反馈标准。

# 如何使用

### 方案 A：IDE 增强型（推荐初学者）

工具： Cursor (Agent Mode) 或 GitHub Copilot + Terminal。

做法：

1. 在 Cursor 中开启 Composer 或 Agent 模式。
2. 将上述“全流程 Prompt 框架”作为 .cursorrules 文件存入项目根目录。这样 AI 每次对话都会遵循这个开发范式。

### 方案 B：自动化流水线型（推荐进阶）

工具： 写一个简单的 Python 脚本，调用 OpenAI/Claude API。

做法： 编写一个 dev_agent.py，依次调用：

读取 api/user.proto -> 调用 LLM 生成 tests/main_flow_test.go。

执行测试（预期失败）。

调用 LLM 生成 internal/service/ 代码。

循环直到 go test 通过。

## 分阶段实战演练

1. “锚定”契约 (Step 2-3) 当你写完初步的 Proto 后，绝对不要先写业务代码。

动作： 把 Proto 丢给 AI，下令：“请根据这个契约，在 /tests 目录下编写完全独立的 REST/RPC 测试。这些测试现在应该运行失败。这组测试将作为后续所有代码的唯一验收标准，未经许可不得修改。”

关键点： 这组测试必须涵盖所有的核心业务链路。

2. 引导 AI “填空” (Step 4)
   动作： 告诉 AI：“现在，请在 /internal 下实现业务逻辑，直到 /tests 中的所有用例全部 Pass。”

AI 行为： 如果它写错了，报错信息（Compile error 或 Assert fail）就是它最好的导师。你可以直接把错误贴给它，让它自愈。

3. 应对变更与重构 (Step 5-6)
   这是最体现价值的一步。

动作： 当需求变更（如补全 Proto 字段）时，先更新 Proto，然后告诉 AI：“需求已更新，请在不破坏 /tests/integration 中已有测试的前提下，实现新功能。”

价值： 即使 AI 在重构时改动了底层逻辑，只要这组“主流程用例”还是绿色的，你就拥有了极高的信心。

## 影子模式

### 配合此 Makefile 的 AI 开发指令 (Prompt)

由于代码不再放在 ai-derive 目录下，你需要调整给 AI 的指令：

关于生成位置： “请在现有的业务代码目录下（如 internal/service/）生成对应的实现文件，文件名必须以 \_ai.go 结尾。”

关于编译标签： “在所有生成的 \_ai.go 文件第一行，必须添加 // +build ai_shadow。”

关于测试逻辑： “请在 .ai-derive/tests/ 目录下生成 REST 测试代码。通过 HTTP 调用 localhost:8080 验证接口。”

### 方案一的工作流示例

Step 1: AI 读取 internal/service/user.go。

Step 2: AI 生成 internal/service/user_ai.go（带有 // +build ai_shadow）。

Step 3: 你打开一个终端运行 make ai-up。

Step 4: 你打开另一个终端运行 make ai-test。

Step 5: 如果你想知道新逻辑是否搞坏了旧功能，运行 make ai-test-all。

Step 6: 满意后，运行 make ai-merge，它会自动删除 \_ai.go 的标签并覆盖原文件。

### 核心优势提示

使用 sed -i '1d' "$$target_file" 在合并时会自动删除 // +build ai_shadow 那一行，这样合并后的代码就变成了正式的、不带特殊标签的业务代码。

下一步： 建议先手动创建一个 \*\_ai.go 文件并运行 make ai-up，确认你的 main.go 入口是否能正确识别标签并加载影子逻辑。
