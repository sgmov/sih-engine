# pendline 批子代理施工归报

工地：w/pendline（分支 w/pendline，基点 baa1868）。全部新建件，git 零写动作。

## 模块布局与 CLI 面

落位：src/bin/pendline.rs（CLI 入口，经 #[path = "../pendline/mod.rs"] 私有引入模块树，lib.rs 零改动）加 src/pendline/（mod.rs 档位类型 Gear 3/6/9/off 与参数解析与退出码信封、collect.rs、config.rs、dispatch.rs 含 backfill、route.rs）加 tests/pendline.rs（顶部 canon 指针头注指 DES-017 加任务包加 mcpdual 第九节）。

子命令五件：collect（--out 加 --critsweep-json/--parking-json/--parking-trail/--covenant 皆可重复）；dispatch（--candidates --out [--gears 3|6|9|off] [--config] [--seat 框架:模型]）；backfill（--contract --responses 加 --verdict 或 --verdict-material --out）；route（--materials... --scribe-binary --trail [--out] [--ttl-days] [--identity-report] [--session/--sessions/--locks 或 --no-session-reason] [--dry-run]）；config（--config --get 或 --set）。退出码 0/1/2。

## 三段实现要点与对表注记

- collect（pl-01）：判据扫出参只吃文件零内调进程，取 status=sunk 沉底判据入候选（判据沉底机械召回对表）；泊界 trail 只读重放 parking_entered 与 parking_exited 取末态在泊者的 entry_id 与 title 与 exit_condition；候裁单 md 取列表项 json 取数组。输出 kind=pendline-candidates 统一清单，每项带 source 与 id 与 title 与 summary 与 origin。
- dispatch（pl-02）：档位解析序 CLI 旗标优先配置件次之缺省 3（令源原字），off 即零合同落 mode=manual 全人工清单。每候选项经 attractor contract_mode::emit_contract 真内核落盘 kind=facet-sampling-contract 合同（确定性排序键盘），shots=gear 值。回填插拔：backfill 用 attractor load_contract 与 load_responses 严格对表（缺发即拒 exit 1），判词上游直读零判定。
- route（pl-03）：判词三态机械判据直读字段——stable_clear 即过、boundary 即 boundary、其余即未过。过者：输出建议通道字段 channel batch 或 direct_edit，落链走 scribe direct 直改笔（subject=execution_marked）；boundary 者：subject=human_rewrite_marked，代码结构保证不触 park 不触重测；未过者：scribe park 停泊事件（title 与 exit_condition 与 ttl_days 与 context 照录判词要害）。写链全经 scribe 子进程零直接写 trail；park 无会话闸、marks 无会话形走 --no-session-reason；有过者或 boundary 时前置校验正身件与会话席（exit 2 零链写入）。

## 测试绿证

15 件全绿（cargo test --test pendline：15 passed 0 failed 退出码 0）：三态全链各一件加混批三态同链加档位 3/6/9 shots 断言加缺省 3 加 off 零采样加非法值 exit 2 加配置件读写两面（含 CLI 压配置件与值域外双面 exit 2）加 collect 三源加 collect 非法出参 exit 2 加 backfill 缺发拒收加过者缺正身件或会话席 exit 2 零写入加 dry-run 零链写加零触碰断言。全链测试断言链上事件类型与字段（parking_entered 四要害；direct_edit_completed 的 subject 与 pen_kind 与 files）并 scribe verify valid。cargo build 退出码 0，pendline 自有文件零警告。

## 零触碰断言读数

git status --porcelain 对 src/bin/critsweep.rs、src/attractor、src/bin/lease.rs、src/bin/lease、src/retriever、src/mcpserver 全空（零 diff）；测试内 zero_touch_critsweep_attractor_lease 绿。全仓改动面恰为四许可位。

## 偏差申报（v1 边界）

1. 链上标记事件型：scribe 二进制事件型固定（zero-touch 约束下不可加型），execution_marked 与 human_rewrite_marked 承载于 scribe direct 直改笔 details.subject（链上 event_type=direct_edit_completed，断言 event_type 加 subject 双键）——正典依据即任务包直改笔两通道留痕。
2. 判词读序：gate_verdict 优先回落 verdict（兼容 attractor score 材料与 check 报告形）；backfill 材料双字段同值落盘。
3. backfill 判词源：批令最小形之上补 --verdict 与 --verdict-material 二择一（判词上游直读）；不写飞轮 trail（runs_written=0 如实申报）。
4. route 增补旗标：--dry-run（零链写预演）、--ttl-days（缺省 30）、--out 路由报告落盘。
5. collect 范围：判据扫源仅 sunk 入候选（unknown 不入）；多件旗标取重复旗标形。
6. 合同细节：v1 零模型，system prompt 固定最小框架串，seat 缺省 pendline:unbound（--seat 可覆写）。
7. 测试发现：scribe park 泊界重放面解析 trail 同目录全部 ndjson，测试链移入 chains/ 子目录与夹具隔离；scribe 错误信封出 stdout，route 错误附录双通道原文。
8. v1 边界：collect 不内调 critsweep 进程、回填腿不调模型、挂点不接会话启动例程——三项均照批令边界未越。pl-06 留结算腿。
