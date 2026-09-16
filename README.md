# 司衡引擎 sih-engine

司衡引擎是治理 LLM 参与开发的确定性执行层。LLM 只生成符号材料，不拥有写入权；治理操作全部由确定性程序执行；人只把注意力投向异常信号。本仓是引擎本体，哲学权威源在 sih-philosophy 仓，数学形式化桥梁在 sih-math 仓。

## 为什么可以信它

- 写操作全部经租约与文件锁，无锁写入会被机械闸拦下
- 每个治理动作落哈希链即 sih/event/trail 每日一文件，一条命令可整链复算
- 判断不认人不认 LLM 只认材料：裁决类产出须经得一测量闸门与执契终签才生效
- 一切断言携带链锚，红证文化即对己不利的读数逐件留档不清洗

## 五条工程基线，大白话版

1. 确定性程序是治理的唯一执行者，LLM 产出只是待校验材料
2. 人类注意力是稀缺资源，系统按此假设设计
3. 人只在视图告警时介入，不看原始日志
4. 所有写入可追溯、可机械校验、历史不可篡改
5. 治理延伸是减少 LLM 参与而非增加

## 五分钟跑起来

从引擎仓根起跑，命令全部走引擎 bin 位即 `target/debug/<工具名>`（2026-09-14 起生产调用面唯一形态）：

```bash
cargo build

# 验链：最近一日治理链整链复算，status valid 即完整未篡改
latest=$(find sih/event/trail -name '*.ndjson' | sort | tail -1)
target/debug/scribe verify --trail "$latest"

# 判据扫：GOV-002 六判据现态（达成／在飞／沉底三态）与泊界路由与两账本在飞
# --root 指工作区根（引擎仓与 sih-tools 双仓形）；单仓克隆时在飞面降级如实可见
target/debug/critsweep --at $(date +%F) --root ..
```

## 接入你自己的项目

三行电梯：

1. 司衡引擎治理你的项目而不接管你的代码：LLM 产出只是待校验材料，写入权在确定性程序与租约闸
2. 三步接入：`sih init` 落 sih 树 → 客户端注册一行 stdio 配置 → 会话启动五件照模版裁剪，全程无后台服务、无端口、无令牌
3. 缺省 stdio 单域一块配置（DEC-026 裁定）；HTTP 多域与管理台是显式选用的进阶档

动线正典三件：

- 接入指南：`doc/guide/adoption-guide-v1.md`，域自举、客户端配置、会话启动五件、租约治理首件的单页走读
- AGENTS 模版：`doc/guide/AGENTS-template.md`，最小可配形，照裁即用
- 决策档：`doc/decision/026-adoption-default-stdio-v1.md`，缺省面裁 stdio，HTTP 面转显式选用（只裁缺省面，不裁通道存废）

## MCP 接入：sihmcp

引擎自带 MCP 服务器（Rust 载体，DEC-023；二进制 `target/debug/sihmcp`）。缺省 stdio 形，一进程一连接一域；在你客户端的 MCP 配置里加一块（下形为通用形，路径换成你机器上的绝对路径）：

```json
{
  "mcp": {
    "servers": {
      "sih": {
        "type": "stdio",
        "command": "<引擎仓绝对路径>/target/debug/sihmcp",
        "env": {
          "SIH_ROOT": "<你要治理的项目根>"
        },
        "enabled": true,
        "timeoutMs": 60000
      }
    }
  }
}
```

- `SIH_ROOT` 显式指域根最稳（缺省自当前目录向上找标记）
- stdio 缺省分支即第一域映射形：一个客户端配置对一个项目根，多项目即多块配置
- 注意：`bootstrap --client-config` 自动写入的是 HTTP 形 payload（指向 8765 面），stdio 缺省档请照上形手写，两形不可混写

在役工具面十九具：

- 读面九具（alpha，SPEC-023）：`chain_query`、`chain_verify`、`critsweep`、`heartbeat`、`locks_read`、`retriever_recall`、`naming_guide`、`nomenclator_query`、`nomenclator_check`
- 写面十具（beta，DES-014 外部缺省分级）：`lease_open`、`record_intent`、`record_append`、`record_park`、`lease_lock`、`lease_unlock`、`lease_wait_turn`、`lease_claim`、`lease_commit`、`lease_close`；本地可信分级另开 `record_direct` 与 `lease_unclaim`（全集十二行）
- 写面受会话闸约束：一连接恰好一会话，先 `lease_open` 立会话，未立会话一切写拒

进阶档（显式选用）：8765 HTTP 面承载 streamable HTTP 识别写面与 `/tokens` 令牌管理台（DES-015），多域接入与令牌生命周期见 `doc/design/DES-015-mcp-http-multitenant-auth-v1.md`。

## 两个入口

- 只想用：读 `doc/guide/user-guide-v1.md` 即使用者入门，三十分钟含一个真实批的完整走读
- 想参与引擎开发：读 `doc/guide/contributor-guide-v1.md` 即贡献者入门，含五仓地图、批机械链与提交变更的完整路径

## 仓内一图

| 位 | 职能 |
|---|---|
| src/ | 引擎库：治理六席即三问 ask3repeater、参验 scrutinator、书简 event_stream、判定器 attractor、视图 view、温故 retriever，加 MCP 载体 mcpserver、共享谓词基座 snapline 等 |
| src/bin/ | 三十四个命令行工具位，生产调用面即 `target/debug/<工具名>` |
| sih/event/trail/ | 治理哈希链，每日一文件，只经书简（scribe）写位 |
| sih/state/plan/ 与 sih/event/plan/ | 任务包与结果档 |
| sih/state/parking/ | 泊界材料，未决事项的有界停靠地 |
| critsweep/registry.json | GOV-002 六判据单源，判据文本逐字节内嵌自正典 |
| packs/ | 规则与格式与路由纯数据包（formatter、nomenclator、parser、selector） |
| doc/ | 治理文档九类，doc/guide/ 是入门面，doc/decision/ 是决策档 |
| tools 侧 | sih-tools 围堰冻结只读（2026-09-14 起生产调用面全切引擎 bin 位） |
