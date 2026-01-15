import json
from openai import OpenAI # 假设使用兼容 OpenAI 接口的模型

class Orchestrator:
    def __init__(self, runner, blackboard, model_api_key):
        self.runner = runner
        self.blackboard = blackboard
        self.client = OpenAI(api_key=model_api_key)
        self.modules = self._load_module_definitions() # 加载所有 YAML 的描述信息

    def _load_module_definitions(self):
        # 读取 modules/ 目录下所有 YAML，提取 ID 和描述给 AI 看
        # 只需要给 AI 看名字和描述，不需要给它看复杂的请求细节
        return [
            {"id": "user_login", "desc": "用户登录，产出 token"},
            {"id": "get_room_list", "desc": "获取直播间列表，产出 room_id_list"},
            {"id": "mute_user", "desc": "禁言指定用户，需要 room_id"}
        ]

    def run_goal(self, user_goal):
        max_steps = 10
        for i in range(max_steps):
            # 1. 准备上下文给 AI
            context = {
                "module_list": self.modules,
                "blackboard": self.blackboard.get_all_summary(), # 只给描述和键名
                "goal": user_goal
            }

            # 2. 询问 AI 下一步做什么
            response = self.ask_ai(context)
            decision = json.loads(response)

            print(f"🧠 AI 思考: {decision['thought']}")

            if decision['is_finished']:
                print("🎉 任务圆满完成！")
                break

            # 3. 调度 Runner 执行具体的 YAML
            module_id = decision['next_step']
            print(f"🚀 执行模块: {module_id}")

            # Runner 内部会自动结合黑板进行变量替换和提取
            success = self.runner.execute_by_id(module_id)

            if not success:
                print("❌ 执行失败，交给 AI 重新决策。")

    def ask_ai(self, context):
        # 调用大模型 (GPT-4, Claude, DeepSeek 等)
        # 这里填入上面的 Prompt 逻辑
        pass
