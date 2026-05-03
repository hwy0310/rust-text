# API 调用模板（新手友好 + 便于维护）

> 目标：给新手一个可以直接复制使用的模板，同时保证后续维护成本低。

---

## 1. 目录建议

```text
project/
  ├─ api/
  │   ├─ client.py          # 统一请求入口（超时、重试、日志、鉴权）
  │   ├─ endpoints.py       # 按业务拆分 API 方法
  │   └─ models.py          # 请求/响应数据结构（可选）
  ├─ config/
  │   └─ settings.py        # 环境变量读取
  ├─ tests/
  │   └─ test_api.py        # 基础测试
  ├─ .env.example           # 环境变量示例
  └─ README.md
```

---

## 2. 环境变量模板（`.env.example`）

```env
API_BASE_URL=https://api.example.com
API_KEY=replace_me
API_TIMEOUT=15
API_RETRY=2
```

---

## 3. 通用 API Client（`api/client.py`）

```python
import os
import time
import requests
from typing import Any, Dict, Optional


class ApiError(Exception):
    """统一 API 异常类型，方便上层统一处理。"""


class ApiClient:
    def __init__(
        self,
        base_url: Optional[str] = None,
        api_key: Optional[str] = None,
        timeout: Optional[int] = None,
        max_retries: Optional[int] = None,
    ):
        self.base_url = (base_url or os.getenv("API_BASE_URL", "")).rstrip("/")
        self.api_key = api_key or os.getenv("API_KEY", "")
        self.timeout = timeout or int(os.getenv("API_TIMEOUT", "15"))
        self.max_retries = max_retries or int(os.getenv("API_RETRY", "2"))

        if not self.base_url:
            raise ValueError("API_BASE_URL 未配置")
        if not self.api_key:
            raise ValueError("API_KEY 未配置")

        self.session = requests.Session()
        self.session.headers.update(
            {
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
                "Accept": "application/json",
                "User-Agent": "demo-api-client/1.0",
            }
        )

    def _request(
        self,
        method: str,
        path: str,
        *,
        params: Optional[Dict[str, Any]] = None,
        json: Optional[Dict[str, Any]] = None,
    ) -> Dict[str, Any]:
        url = f"{self.base_url}/{path.lstrip('/')}"
        last_error = None

        for attempt in range(self.max_retries + 1):
            try:
                resp = self.session.request(
                    method=method.upper(),
                    url=url,
                    params=params,
                    json=json,
                    timeout=self.timeout,
                )

                # 统一错误处理
                if resp.status_code >= 400:
                    raise ApiError(
                        f"HTTP {resp.status_code} | url={url} | body={resp.text[:300]}"
                    )

                # 尝试按 JSON 返回
                try:
                    return resp.json()
                except ValueError:
                    raise ApiError(f"响应不是 JSON：url={url} | text={resp.text[:300]}")

            except (requests.RequestException, ApiError) as err:
                last_error = err
                if attempt < self.max_retries:
                    time.sleep(0.5 * (attempt + 1))  # 简单退避
                else:
                    raise ApiError(f"请求失败（重试后）: {err}") from err

        # 理论不会到这里，仅为了类型完整性
        raise ApiError(str(last_error))

    # 对外暴露的语义方法（建议按业务继续扩展）
    def get(self, path: str, params: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self._request("GET", path, params=params)

    def post(self, path: str, json: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        return self._request("POST", path, json=json)
```

---

## 4. 业务接口封装（`api/endpoints.py`）

```python
from typing import Dict, Any
from .client import ApiClient


class UserApi:
    def __init__(self, client: ApiClient):
        self.client = client

    def get_user_profile(self, user_id: str) -> Dict[str, Any]:
        # GET /v1/users/{id}
        return self.client.get(f"/v1/users/{user_id}")

    def create_user(self, name: str, email: str) -> Dict[str, Any]:
        # POST /v1/users
        payload = {"name": name, "email": email}
        return self.client.post("/v1/users", json=payload)
```

---

## 5. 使用示例（`example.py`）

```python
from api.client import ApiClient
from api.endpoints import UserApi


def main():
    client = ApiClient()
    user_api = UserApi(client)

    profile = user_api.get_user_profile("123")
    print("用户信息:", profile)

    created = user_api.create_user("Alice", "alice@example.com")
    print("创建结果:", created)


if __name__ == "__main__":
    main()
```

---

## 6. 新手必看（避免踩坑）

1. **先用 `.env.example` 复制出 `.env`**，再填真实密钥。  
2. **所有请求都走 `ApiClient`**，不要在业务代码里直接 `requests.get/post`。  
3. 遇到报错先看：
   - URL 是否正确
   - API Key 是否过期
   - 请求参数是否符合接口文档
4. 先跑通一个最小接口（如 `get_user_profile`），再逐步扩展。

---

## 7. 维护规范（团队协作）

- 每新增一个接口：
  - 在 `endpoints.py` 增加一个清晰命名的方法
  - 写明对应 HTTP 方法和路径注释
  - 补一个基础测试用例
- 每个 API 变更要记录：
  - 变更日期
  - 变更内容
  - 影响范围
- 固定版本依赖（`requirements.txt`），避免“我这能跑你那不行”。

---

## 8. 可选增强（后续迭代）

- 增加结构化日志（trace_id / request_id）
- 增加限流与熔断
- 用 `pydantic` 做请求响应校验
- 把重试策略改为按错误类型区分（超时重试，参数错误不重试）

---

## 9. 快速复制版（最小模板）

```python
import requests

BASE_URL = "https://api.example.com"
API_KEY = "replace_me"

headers = {
    "Authorization": f"Bearer {API_KEY}",
    "Content-Type": "application/json",
}

resp = requests.get(f"{BASE_URL}/v1/ping", headers=headers, timeout=10)
resp.raise_for_status()
print(resp.json())
```

适用场景：先验证网络/密钥可用；验证通过后，尽快迁移到上面的 `ApiClient` 结构。
