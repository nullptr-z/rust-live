import requests
import json
import yaml
import re
import difflib
import sys
from string import Template
from typing import Dict, List, Any, Optional

# ==========================================
# 1. 基础设施：增强版黑板 (Smart Blackboard)
# ==========================================
class Blackboard:
    def __init__(self):
        self._store = {} # 存储值
        self._meta = {}  # 存储描述元数据

    def set(self, key: str, value: Any, description: str = ""):
        print(f"    💡 [黑板更新] {key} = {value} ({description})")
        self._store[key] = value
        self._meta[key] = description

    def get(self, key: str) -> Any:
        return self._store.get(key)

    def get_all_metadata(self) -> Dict[str, str]:
        """获取所有变量的 key 和 description，供 AI 分析"""
        return self._meta

    def dump(self):
        return self._store

# ==========================================
# 2. 模拟 AI 引擎 (Simulated AI Engine)
# ==========================================
class AIEngine:
    """
    模拟 LLM 的行为。
    在真实场景中，这里会调用 OpenAI/Claude API。
    在这里，我们使用 'difflib' 进行模糊匹配来模拟语义推断。
    """
    @staticmethod
    def infer_parameter(target_name: str, target_desc: str, blackboard: Blackboard) -> Any:
        print(f"    🤖 [AI 介入] 正在为 '{target_name}' 寻找最佳匹配...")

        candidates = blackboard.get_all_metadata().keys()
        if not candidates:
            return None

        # 1. 简单的模糊匹配模拟 (寻找最相似的 Key)
        # 例如: 'user_id' 和 'uid' 相似度较高
        matches = difflib.get_close_matches(target_name, candidates, n=1, cutoff=0.4)

        if matches:
            best_match = matches[0]
            print(f"    ✨ [AI 发现] 语义关联: '{target_name}' ≈ '{best_match}'")
            return blackboard.get(best_match)

        return None

# ==========================================
# 3. 核心组件：增强版原子模块
# ==========================================
class AtomicModule:
    def __init__(self, config: Dict):
        self.config = config
        self.name = config.get('name', 'Unnamed_Module')
        self.request_conf = config.get('request', {})
        self.inputs = config.get('inputs', [])
        self.outputs = config.get('outputs', [])
        self.strategies = config.get('strategies', {})
        self.default_params = config.get('DefaultPramas', {})

    def _resolve_params(self, blackboard: Blackboard) -> Dict[str, Any]:
        resolved = {}
        missing = []

        for input_def in self.inputs:
            key = input_def['name']
            desc = input_def.get('description', '')

            # A. 尝试直接获取
            val = blackboard.get(key)

            # B. 尝试获取默认参数
            if val is None:
                val = self.default_params.get(key)

            # C. 尝试 AI 推断 (如果开启了 fallback)
            if val is None and self.strategies.get('ai_fallback', True):
                val = AIEngine.infer_parameter(key, desc, blackboard)

            if val is not None:
                resolved[key] = val
            else:
                missing.append(key)

        if missing:
            print(f"    ⚠️ [警告] 缺失参数: {missing}")

        return resolved

    def _smart_template_render(self, template_obj: Any, params: Dict) -> Any:
        """
        增强版渲染：支持将 "123" 字符串回转为 int/float
        """
        if isinstance(template_obj, str):
            # 1. 替换变量
            rendered = Template(template_obj).safe_substitute(params)
            # 2. 尝试类型恢复 (避免 ID 被变成字符串)
            # 如果是纯数字，且原模版不是显式的字符串结构(这个判断比较简单)，转为数字
            if rendered.isdigit():
                return int(rendered)
            return rendered
        elif isinstance(template_obj, dict):
            return {k: self._smart_template_render(v, params) for k, v in template_obj.items()}
        elif isinstance(template_obj, list):
            return [self._smart_template_render(i, params) for i in template_obj]
        return template_obj

    def _extract_outputs(self, response_json: Dict, blackboard: Blackboard):
        def flatten_schema(schema_dict):
            flat_list = []
            for name, definition in schema_dict.items():
                if 'selector' in definition:
                    def_copy = definition.copy()
                    def_copy['name'] = name
                    flat_list.append(def_copy)

                if 'schema' in definition and isinstance(definition['schema'], dict):
                    flat_list.extend(flatten_schema(definition['schema']))
            return flat_list

        if not isinstance(self.outputs, dict) or 'schema' not in self.outputs:
            return

        outputs_to_process = flatten_schema(self.outputs['schema'])

        for output_def in outputs_to_process:
            key_name = output_def['name']
            json_path = output_def['selector']
            keys = json_path.replace("$.", "").split(".")
            val = response_json
            try:
                for i, k in enumerate(keys):
                    if '[*]' in k:
                        array_name = k.split('[*]')[0]
                        if array_name in val and isinstance(val[array_name], list):
                            remaining_keys = keys[i + 1:]
                            results = []
                            for item in val[array_name]:
                                sub_val = item
                                for sub_k in remaining_keys:
                                    if isinstance(sub_val, dict):
                                        sub_val = sub_val.get(sub_k)
                                    else:
                                        sub_val = None
                                        break
                                if sub_val is not None:
                                    results.append(sub_val)
                            val = results
                            break 
                        else:
                            val = None
                            break
                    elif isinstance(val, list):
                        val = val[int(k)]
                    elif isinstance(val, dict):
                        val = val.get(k)
                    else:
                        val = None
                        break
                
                if val is not None:
                    blackboard.set(key_name, val, output_def.get('description', ''))
            except (KeyError, ValueError, IndexError, TypeError) as e:
                print(f"    ❌ [提取失败] {key_name} (Path: {json_path}) - {e}")

    def _mock_router(self, url, method):
        """增强的 Mock 路由，不区分大小写"""
        url_lower = url.lower()
        if "login" in url_lower:
            return {"code": 200, "data": {"access_token": "MOCK_TOKEN_XYZ", "user_info": {"uid": "U_9527"}}}
        if "order" in url_lower:
            return {"code": 200, "data": {"order_no": "ORD_2026_8888", "status": "created"}}
        return {}

    def run(self, blackboard: Blackboard, mock_mode=False):
        print(f"\n🚀 [执行] {self.name} ({self.request_conf['method']} {self.request_conf['url']})")

        # 1. 准备参数
        params = self._resolve_params(blackboard)

        # 2. 渲染请求
        url = self._smart_template_render(self.request_conf['url'], params)
        headers = self._smart_template_render(self.request_conf.get('headers', {}), params)
        body = self._smart_template_render(self.request_conf.get('body', {}), params)

        print(f"    📤 Payload: {json.dumps(body, ensure_ascii=False)}")

        # 3. 发送/Mock
        response_json = {}
        if mock_mode:
            response_json = self._mock_router(url, self.request_conf['method'])
            print(f"    🔙 Response (Mock): {json.dumps(response_json)}")
        else:
            try:
                resp = requests.request(self.request_conf['method'], url, json=body, headers=headers, verify=False)
                response_json = resp.json()
                print(f"    🔙 Response: Status {resp.status_code}")
            except Exception as e:
                print(f"    💥 请求异常: {e}")
                return

        # 4. 提取输出
        self._extract_outputs(response_json, blackboard)

# ==========================================
# 4. 编排运行器 (The Runner)
# ==========================================
class Runner:
    def __init__(self):
        self.blackboard = Blackboard()

    def load_module_from_yaml(self, file_path: str) -> AtomicModule:
        with open(file_path, 'r', encoding='utf-8') as f:
            config = yaml.safe_load(f)
        return AtomicModule(config)

    def run_workflow(self, module_files: List[str], initial_vars: Dict = None, mock=True):
        # 预置环境变量
        if initial_vars:
            for k, v in initial_vars.items():
                self.blackboard.set(k, v, "Initial Config")

        # 顺序执行
        for file_path in module_files:
            try:
                module = self.load_module_from_yaml(file_path)
                module.run(self.blackboard, mock_mode=mock)
            except FileNotFoundError:
                print(f"❌ 找不到文件: {file_path}")

        print("\n" + "="*30)
        print("🏁 最终黑板状态:")
        print(json.dumps(self.blackboard.dump(), indent=2, ensure_ascii=False))

# ==========================================
# 5. 演示入口
# ==========================================
if __name__ == "__main__":
    # 检查是否有传入文件名
    if len(sys.argv) < 2:
        print("❌ 用法错误。请指定 YAML 文件:")
        sys.exit(1)

    # 获取命令行传入的所有文件名
    yaml_files = sys.argv[1:]

    print(f"📂 准备加载流程: {yaml_files}")

    runner = Runner()

    # 这里我们默认关闭 Mock，走真实请求
    runner.run_workflow(
        module_files=yaml_files,
        initial_vars={"user": "admin"}, # 这里可以放一些默认通用的变量
        mock=False
    )
