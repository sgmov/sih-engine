# SPEC-023 契约对表单（mcpcold-solo 批材料）

> 对表正典：sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md 工具契约节与红线节
> 被对面：sih-tools/mcpline 服务器（源码 server.py 与 runtime.py 加 stdio 面实测出参，读数实录 stdio-five-tool-readings.json）
> 对表日：2026-09-09；判定词：符合／超集（出参字段多于契约而核心字段全符）／差异

## 逐工具对表

### chain_query

- 工具名：SPEC `chain_query` ＝ 实测 `mcp__sih__chain_query`。符合
- 入参：SPEC `date 即 YYYY-MM-DD 缺省实日；event_type 可选过滤` ＝ schema `properties {date, event_type}` 俱可选。符合
- 出参：SPEC `当日链事件清单即哈希、事件型、主体字段`；实测含 `events[]（event_hash 与 event_type 与 subject 与 doc_id 与 timestamp）` 外加信封字段 `date/trail/event_type_filter/matches/returned`。超集（清单三核心字段全在，信封为定位与计数补充）
- 承接 CLI 只读：SPEC `scribe query --trail <date>` ＝ 实测 scribe_bin 主树 target/debug 只读调用。符合

### chain_verify

- 工具名：SPEC `chain_verify` ＝ 实测同名。符合
- 入参：SPEC `date` ＝ schema `required [date]`。符合
- 出参：SPEC `逐笔校验与整链 valid 判词`；实测 `status 与 valid 与 events 数与 first_hash/last_hash`。符合（判词形齐）
- 承接 CLI 只读：SPEC `scribe verify --trail` ＝ 实测同。符合

### critsweep

- 工具名：SPEC `critsweep` ＝ 实测同名。符合
- 入参：SPEC `date` ＝ schema `required [date]`。符合
- 出参：SPEC `五判据三态与泊界路由与两账本在飞，严格 JSON 单对象` ＝ 实测 sweep.py stdout 原样投影单对象（criteria 五键与 parking 双线与 inflight 全在）。符合
- 承接 CLI 只读：SPEC `python3 sweep.py --at <date> --root <根>，stdout 捕获` ＝ 实测 run_readonly stdout 捕获。符合

### heartbeat

- 工具名：SPEC `heartbeat` ＝ 实测同名。符合
- 入参：SPEC `无` ＝ schema `properties 空 required 空`。符合
- 出参：SPEC `秤星三维最新读数与距上快照间隔日` ＝ 实测 `readings{convergence/adoption/mergeback}` 加 `days_since_last_snapshot`。符合
- 承接 CLI 只读：SPEC `gauge.cli read 只读不落链` ＝ 实测同（零链笔增量旁证：调用前后链 74 笔不变）。符合

### locks_read

- 工具名：SPEC `locks_read` ＝ 实测同名。符合
- 入参：SPEC `无` ＝ schema 空。符合
- 出参：SPEC `未释放锁成对核算与活跃会话数` ＝ 实测 `held_count 与 held_locks[path,session_id] 与 active_sessions 与 active_session_ids`。符合（实测 held_count 10 恰对本批十锁，成对核算语义实证）
- 承接 CLI 只读：SPEC `lease status 查册` ＝ 实测同。符合

## 错误语义与描述自足对表

- 错误载荷四字段：SPEC `error 与 what_this_tool_does 与 valid_params 与 canonical_pointers` ＝ 实测两错误探针（chain_query date=`2026-9-9`、chain_verify date=`not-a-date`）载荷键集恰四字段。符合
- 工具描述自足：SPEC `双语一句话加正典指针` ＝ 实测五工具 description 俱中英双语句加 SPEC-023 与线程序包指针。符合

## 红线对表

1. alpha 零写入零治理语义：五承接路径全只读子命令；冷跑前后双仓 git status 逐字节全等（gitstatus-*-before/after 四件），链 74 笔不变，`.zcode/config.json` 哈希不变。符合
2. critsweep stdout 捕获零落盘：源码 run_readonly 捕获不落盘；仓内 sweep-latest.json 唯一一件系 2026-09-08 21:32 critsweep 开发期遗留跟踪件，本批前后零变化（mcpserv 零写入测试同款白名单事实），非本线产生。符合（遗留件如实申明）
3. beta 写面不在范围：五工具外无写面暴露（toolNames 面实证）。符合
4. 自有运行时归 pk-079：服务器 stdio 拉式，零自有对话循环。符合

## 差异清单

- 零实质差异。申报两条超集与遗留事实：一 chain_query 出参信封字段系超集补充（核心三字段全符）；二 sweep-latest.json 系开发期遗留跟踪件与本线无关（时点与 git 归属实证在案）
