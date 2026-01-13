# 变量定义
PROJECT_NAME  := my-project
AI_DIR        := .ai-derive
AI_TESTS      := $(AI_DIR)/tests
MAIN_INTERNAL := internal
# 用于匹配 AI 生成的影子代码文件
AI_PATTERN    := *_ai.go
# 影子测试运行时的基础 URL (REST/HTTP)
TEST_URL      := http://localhost:8080

.PHONY: all help ai-init ai-test ai-test-all ai-up ai-merge ai-clean

help: ## 显示帮助信息
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

# --- 第一阶段：环境准备 ---

ai-init: ## 初始化实验舱 (仅存放 API 和测试，代码将生成在 internal)
	mkdir -p $(AI_DIR)/api $(AI_DIR)/tests
	@echo "✅ AI 实验舱初始化完成。请让 AI 在 $(MAIN_INTERNAL) 下生成以 _ai.go 结尾的文件。"

# --- 第二阶段：运行与验证 ---

ai-up: ## 启动影子服务 (激活所有带有 ai_shadow 标签的代码)
	@echo "🌐 正在以 [影子模式] 启动服务..."
	MODE=shadow go run -tags ai_shadow ./cmd/server/main.go

ai-test: ## 运行 AI 生成的 REST 集成测试
	@echo "🧪 正在执行影子 REST 测试: $(TEST_URL)"
	# 影子测试本身不需要编译标签，因为它通过 HTTP 访问
	go test -v ./$(AI_TESTS)/... -args -base-url=$(TEST_URL)

ai-test-all: ## 全量回归：验证影子实现是否通过了主项目原有的测试
	@echo "🛡️ 正在进行全量回归验证 (影子代码 vs 原有测试)..."
	# 启动影子代码时运行原有测试，确保旧功能不坏
	go test -v -tags ai_shadow ./$(MAIN_INTERNAL)/...

# --- 第三阶段：代码合并与清理 ---

ai-diff: ## 对比 AI 代码与原代码
	@echo "📊 正在扫描目录中的差异文件..."
	@find $(MAIN_INTERNAL) -name "$(AI_PATTERN)" -exec sh -c 'echo "Diffing: {} with $${1%_ai.go}.go"; diff -u "$${1%_ai.go}.go" "{}"' _ {} \;

ai-merge: ## 【执行合并】将 _ai.go 替换原文件
	@echo "⚠️ 准备正式合并 AI 代码..."
	@read -p "确认已通过所有测试? 此操作将覆盖原有的 .go 文件 [y/N] " ans && [ $${ans:-N} = y ]
	@find $(MAIN_INTERNAL) -name "$(AI_PATTERN)" | while read shadow_file; do \
		target_file="$${shadow_file%_ai.go}.go"; \
		echo "Merging $$shadow_file -> $$target_file"; \
		cp "$$shadow_file" "$$target_file"; \
		sed -i '1d' "$$target_file"; \
	done
	@echo "🎉 合并完成。请手动检查 package 下的引用。"

ai-clean: ## 清理所有带有 _ai.go 后缀的文件及实验舱
	@echo "🧹 清理影子代码..."
	find . -name "$(AI_PATTERN)" -delete
	rm -rf $(AI_DIR)/tests/*
	@echo "✨ 清理完成"
