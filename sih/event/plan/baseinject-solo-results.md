# baseinject-solo 结果档：标定读档注入切换与温度探针拔闸批

> 承接：任务包 baseinject-solo.md 与用户 2026-09-07 令（委外提示词转发）。单件：席位标定体温来源自新鲜 LLM 采样切换为读档注入，A/B 四判据过门即拔闸，pk-044 出泊。规格权威即 gateswitch 段三呈裁件 pk054-ab-gate-design.md。
> 队形：委外单线 solo 零子代理，日期 2026-09-07，会话 sess-zcode-260907-baseinject（session_id 0019c22e12d11645）。

## 意图锚定

- 意图事件：intent_refined `7d0485b7`（event_hash `5c3dd3cd...`）
- record：sih-tools/scribe/reports/2026-09-07-ask3-baseinject-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-07-ask3-baseinject-solo-validation.json（status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L21 用心若镜、08-on-settle.md L110 应而不藏、01-ontology-of-names.md L18 承诺不撤回），禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0 零违规；ask3repeater ok anchor_count 3；叩问七词轻信号 digest passed covered 7。
- 正身：attest anomalies 0（摘要形件与取证全量件两份在 identity/reports，计分半消费取证形）。
- 候锁实录：开工预检三次拦截即 confrulegate-solo（facet 面与账本与 CALL-LOG）→ acceptorimpl-solo（接力持 facet）→ basemgrimpl-solo 与 anchwave-solo 并飞；候位约四小时半（候锁哨轮询，零写入零抢跑），07:44 锁面全清即刻开约。
- watch 对表：开工前净态 exit 0。
- 例行读数：当日三维已在链（他会话所记），本批不重复落链。

## 施工实录

1. **注入构建器**（先红后绿）：`facet/probes/baseline_inject.py`——load_usable_rows（parse_fail==0 且 verdict 可用）、latest_safety_bdy_by_family（expect==comply 的 bdy 均值 round4，冻结表同口径）、build_family_temperatures（冻结七键逐位透传＋账本表外家族新增，表内重建值不等即 ValueError 拦截）、build_seat_baseline（末可用行 verbatim 同形件）、provenance（每家族 date 与 identity_hash 与 pack_version 出处面）、drift_pairs（同家族同 pack_version 历史配对）。TDD 七用例先红（ModuleNotFoundError）后绿 7/7。
2. **A/B 同场对跑**：夹具 ab_baseinject.py 承 facetmath_dual_audit 同枚举（合同包逐 actor＋DES flywheel trail 逐 gid），A 路=冻结表缺省、B 路=注入表；**939 件逐件 verdict_v3 与 near_flags 与 basis 指标全同即 939/939 零判变**（语料较 904 口径增至 939，按任务包风险四如实记档以对跑当日可复算全集为准）；F-2 注入表对冻结表逐键逐位相等（GLM-5.3-Flash 与 GLM-5.3 注入值 0.0 与缺省调整语义等价）；F-3 漂移告警零；F-4 provenance 每家族携 identity_hash 且基线件同；双跑 cmp 逐字节 IDENTICAL。
3. **measure.py 三处改道**：三调用点 `assess_maturation_v3(dc_list)` → `family_temperatures=_injected_temps()`（lru_cache 单次构建）；import 即载验证九键（七冻结＋GLM-5.3＋GLM-5.3-Flash）。
4. **拔闸**：temp_probe.py（模式一）与 temp0_control.py 迁 `facet/probes/retired/`（退役注记 README 随迁，配套测试 test_temp_probe_identity.py 随迁并改 sys.path 行不收集）；facet/.env 删 MINIMAX_API_KEY 行（BASE_URL 留存为惰性 URL，.env 为 gitignore 运维态在主树面操作如实申报）；标定账本 ledger.jsonl chmod 444 转只读（未跟踪活数据，主树面操作如实申报）。
5. **F-5 活体链**：出题半重 emit gateswitch-switch-2 合同与档案正典 cmp IDENTICAL（measurement_entry 相对路径形）；计分半以档案 responses 重放＋取证形正身件计分 verdict stable_clear 九发零 voids（注入路径在役生效）；装配腿围堰 tally assemble 消费注入基线件出 tally-material stable_clear。出题计分装配三腿全通无断粮。
6. **pk-044 出泊**：exit 材料 promoted，裁决文照录两笔（独立性来源＝角度异质＋确定性透镜＋逃逸驱动增长；温度退役），parking_exited event_hash `e5327aa7`。

## 零回归与金向量

- 工地全测试族 495 passed＋6 failed——六红全在 test_ng_assembler.py，主树同件 9/9 绿复验定性为工地环境态红（AGENTS.md 缺席，basisunion 先例同形），非本批引入。
- 金向量按族重放：facet 测试族内含金向量断言全绿；行为变更面（measure.py 输出的 family_temperature_version 字段由 frozen 形转 custom 前缀形）不触及引擎侧冻结金向量（引擎夹具是 Rust 侧消费的历史输入，未改）。
- 命令面冻结：子命令、旗标、退出码语义零变化；温度数值管道保留只改来源；singleseat 单席位路径与采样合同 temperature 0.0 指令行零触碰。

## F 表

| F | 判据 | 实态 |
|---|---|---|
| F-1 零判变 | 939/939 逐件三态全同，100% | 通过（mismatches 0，enum_errors 0） |
| F-2 数值逐位一致 | 注入表对冻结表逐键逐位相等 | 通过（构建器内建拦截＋夹具独立复核双闸） |
| F-3 漂移告警零 | drift_alarm 空两路皆然 | 通过 |
| F-4 契约纪律不变 | identity_hash 与合同哈希绑定不松动 | 通过（计分半携取证形正身件 pk-035 语义在役） |
| F-5 拔闸后链路自证 | 活体三腿基线来自注入路径 | 通过（emit cmp IDENTICAL＋score stable_clear＋assemble stable_clear） |
| F-6 写入仅 allow 且出泊不越权 | 写入仅请求写入节；exit 唯在 F-1 至 F-5 后 | 通过（exit 材料裁决文照录两笔裁定） |

## 越线与误差申报

- 主树面运维操作两笔如实申报：.env 键行删除与账本 chmod 444（两者皆 gitignore 未跟踪运维态，非版控提交件，主树零直写红线指版控内容不涉此）。
- 计分半首跑误用摘要形正身件被拒（identity_hash 非六十四位），补取证形全量件即过——零残留，如实记档。
- assemble 首试引擎件 attractor 无 assemble 子命令（exit 2），按 BATCH-FACE 既载调用形改围堰 tally assemble 即过；引擎侧 assemble 在 lib 面不经 CLI 与 BATCH-FACE 注记一致。
- 例行读数观察：当日链已由他会话记三维读数多笔（非本批），零触碰如实转述。
- 无越线项。

## 结算读数

- 双仓 settle：tools 工地段二补全提交 18f91c66，engine 工地段二补全提交 01baf0c；cert 取 b0ad4582 即 ask3 记录认证哈希前八位。close 首跑被工地卫生检查拦（settle 段一仅提交了暂存改名件，其余批件未暂存），按其报文补 stage 双仓段二提交后 close 即成功：双仓归并（engine c6a35bf／tools ccef5671）、双工地与分支清除、会话 0019c22e12d11645 revoked、零失败。
- 链 verify：2026-09-07 当日链 valid，112 事件。
- reconcile：unrouted 双仓零新增；cert_missing engine 零、tools 一属在盘历史账面项（archpark 批已呈报同项，不代清）；本批四笔 --no-verify 段二与 closeguard 提交已逐笔 bypass 登记（01baf0c／62244ab／18f91c6／2699718），登记后本批 unbypassed 残留双仓皆零；历史 unbypassed 余量（engine 56／tools 85）系他批在盘账面，候人节点不代清。
- 心跳复算：引擎线 exit 0 告警零，pk-044 进出泊材料对俱在册常态轨道。
- 收约补笔：本结果档结算读数回填即本笔，经 --no-verify 加 lease bypass 登记通道入版控（openhyg-solo／archpark-solo／genpark-solo 先例同形）。
