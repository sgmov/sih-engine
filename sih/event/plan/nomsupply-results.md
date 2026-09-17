# nomsupply 批结果档

> 清账并联批 debtclear-parallel 簇A执行代理落档，2026-09-11。令源 debtclear-parallel.md 簇A行与 pk-090 出泊条件，正典输入 pk-090 全文含用户 2026-09-11 两轮裁定（三要求与甲乙结合定案）。批机械链全序承 sih-tools/BATCH-FACE.md。

## 一、批身份与链证 {#identity}

- 批名：nomsupply，查册 unknown，--new-stem 认领；概念锚申报 zh 立名供给，派生 nom 即 mcpnomgate 批名既承 nom- 语素加 supply 即供给，既裁 code 形显式申报无承。甲表申报以意图件承载（本批开约时甲表机械闸尚未落地，闸机械形随本批件五生效，申报自洽于批序）
- 会话号：75ec5eb9f9931788（已 revoked，双工地拆除，双支归并删支）
- 意图笔：dbf31027 ea3cac30 事件族在链，plain 形 validation 豁免（DES-016）
- 认证七笔在链（2026-09-11 trail）：1a00036b15555cf5（dec017-scrutinator）与 02d764f16bf10432（spec023-scrutinator）与 6ea83aa75bb419a8（dec017-nomenclator）与 e9d697961529555a（spec023-nomenclator）与 2a96be351a0ae631（tdd-red-green-cert 封装）与 f2fe4cfa0ab23b59（suites-report）共六笔 append 加意图笔，trail 全链 verify valid（42 events）
- settle 提交：tools 仓 89aac1e1（段1 in-scope）加 b75b3958（段1 越界四件 --no-verify 加 bypass 双行在册）；engine 仓 19211fb；归并 merge tools 0f0c0b21 加 engine 1574de6
- reconcile 双零：tools unrouted 0 加 cert_missing 0；engine unrouted 0 加 cert_missing 0

## 二、F1–F8 逐条实跑 {#f-results}

- F1 裸 --new-stem 开约拒且教学载荷含甲表三件：过。test_s9b_cli_bare_claim_rejected 实跑，rc 1 加 reason_code stem_gate_rejected 加报文含 --claim-zh 与 --claim-code 与 --claim-derivation 三旗标加 CLAIM_TEACHING 乙前注入指针，会话台账零落盘断言过
- F2 全甲表开约过且回执载甲表：过。test_s9_cli_claim_open_full 实跑，rc 0 加回执 stem_check.new_coinage_claim 载 zh 与 code 加 code_verdict declared_none 加 derivation 逐段 kind 加 verified_state
- F3 unknown 查词返回含立名摘要：过。test_unknown_query_carries_naming_teaching 实跑，naming_teaching 载五步序摘要加 next_action 乙前注入指 map --concept；错误载荷同载（test_error_payload_carries_naming_teaching）；established 出参零加字段（test_established_query_untouched）
- F4 fixture canonical 域检词解析域包且 first-domain 中央包路径零变：过。test_pack_resolution_canonical_prefers_domain 实跑，域册懒波词经域包查得 lazy 加 _nomenclator_pack(None) 与第一域形俱承 code_root 中央路径加 envelope 缺席回退中央
- F5 map 报告四段齐：过。test_map_four_sections_complete 实跑，六态查得与既裁 code 形与近邻词（机械子串双向）与出泊指针四段齐加只报不判免责声明；unknown 概念报告照出零拒（test_map_unknown_concept_report_only）
- F6 naming_guide 两面在役且计数 19 与 19：过。test_http_face_nineteen_with_naming_guide 加 test_face_counts_stdio_twentyone_http_nineteen 加 stdio 冒烟 EXPECTED_TOOLS_EXTERNAL 19 具实跑
- F7 mcpline 全套绿含 heartbeat：过。工地铁 151 passed 加主树重跑 151 passed，test_heartbeat_happy 与 test_heartbeat_days_since_snapshot_spans_history 俱绿
- F8 lease 全套绿：过。工地铁 344 passed 加主树重跑 344 passed

## 三、六件落形 {#six-items}

- 件一 naming_guide MCP 只读具：alpha 第九具两面在役，内容五段静态教学即立名五步形与 DEC-017 修订四五六指针与检词六态语义与 stem 闸拒教认领三语义与死档禁条，零裁决零 LLM 零写入；stdio 外部面 18 升 19 与 HTTP 18 升 19，计数同步位八处全列：EXPECTED_TOOLS_EXTERNAL 与 HTTP_ALPHA_TOOLS 与 test_write_matrix（21 加 19）与 test_web_smoke（19）与 test_stdio_smoke（19）与 test_wengu_tools（21 加 19）与 test_nomenclator_tools（19）与 ALPHA_READ_ROWS 矩阵读面行；SPEC-023 修订三与 README 与 AI-MANUAL 同步；mcpline 0.9.0 升 0.10.0 双点位（pyproject 加 __init__）。AGENTS.md MCP 节计数为主线同步位本批零写如实转述候主线
- 件二 nomenclator_query unknown 教学扩载：NAMING_GUIDE_SUMMARY 立名程序摘要附 unknown 与错误两态载荷，stdio 与 HTTP 两面 core 共用（_nomenclator_query 单点承载）
- 件三 检词包按域解析：_nomenclator_project_dir 码根 sibling 解析（循 lease stem 闸先例）加 _nomenclator_pack canonical 域根优先域内包 envelope 在场即认加缺席回退中央包；first_domain 形与中央形逐字节零变；HTTP 面检词两具包装位附 layout 域形式
- 件四 nomenclator map --concept：语义映射报告只报不判，四段即六态查得与既裁 code 形与近邻词与出泊指针；CLI 形，MCP 投影候令不并入；nomenclator 0.2.0 升 0.3.0 三源对齐
- 件五 lease stem 闸甲兜底：--claim-zh 与 --claim-code 与 --claim-derivation 三旗标，裸认领拒教学载甲表三件，指称完整与派生对表机械核（派生对表直接对闸既算段判词零重复查询），填不圆即拒，零 LLM 判词位不动；回执 stem_check.new_coinage_claim additive；lease 1.43.0 升 1.44.0 三源对齐；乙前注入使用侧纪律写进 DEC-017 修订六与 AI-MANUAL
- 件六 test_heartbeat_happy 根因修复：根因即 domaware 后 gauge read 只认显式传入 trail 而心跳只传当日链，history 窗口空窗致 days_since_last_snapshot 返 None；修法即 _heartbeat 域链目录全日 sorted glob 展开传 gauge read（镜像 gauge --domain-root 展开语义），间隔日回其本义；断言零弱化加新钉精确值断言（间隔三日 == 3）；工具显式教形路线，夹具播快照路线未取

## 四、套件数字（主树真跑） {#suites}

- mcpline 151 passed 0 failed（批前基线 143 passed 1 failed，红即件六，净增八测全绿）
- lease 344 passed 0 failed（批前基线 340，净增四测）
- nomenclator 34 passed 0 failed（批前基线 30，净增四测）
- gauge 48 passed 3 skipped（基线零回归，本批零触碰）
- critsweep 23 passed 0 failed（基线零回归，本批零触碰）

## 五、改动文件清单 {#changed-files}

- sih-tools/mcpline：src/mcpline/server.py 加 httpface.py 加 writeface/matrix.py 加 __init__.py 加 ../pyproject.toml；tests/test_tools_unit.py 加 test_stdio_smoke.py 加 test_write_matrix.py 加 test_web_smoke.py 加 test_wengu_tools.py 加 test_nomenclator_tools.py 加 test_naming_tools.py 新件；README.md 加 AI-MANUAL.md
- sih-tools/lease：src/lease/core.py 加 cli.py 加 __init__.py 加 ../pyproject.toml；tests/test_stemgate.py 加 tests/frozen/sweepjson/golden-report.json 版本位重冻；CONTRACT.md 修订六十
- sih-tools/nomenclator：src/nomenclator/map.py 新件加 cli.py 加 __init__.py 加 ../pyproject.toml；tests/test_map.py 新件；CONTRACT.md 修订五
- sih-engine：doc/decision/017-wengu-naming.md 修订六加修订五段 C006 存量七笔全角括号修复；doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md 修订三与概览与红线与工具契约 naming_guide 条目；sih/state/plan/nomsupply.md 批任务包；sih/event/plan/nomsupply-materials/ 七件
- CALL-LOG 三笔：mcpline 加 lease 加 nomenclator（权威腿 calls.ndjson 加索引腿已落，mcpline 册投影腿留主树候主线 commit， bypass-calllog 事由在档）

## 六、误差申报与未决项 {#deviations}

- 越界补笔一笔：tools 仓 b75b3958 四路径（三具包根 pyproject.toml 加 nomenclator/tests/test_map.py）经 --no-verify 加 lease bypass 留痕，事由即任务书写入清单漏列包根版本点位与 tests 目录而批义务必需；bypass 台账该 sha 双行（首行全事由有效加次行 retry 冗余，reconcile unbypassed 判定不受扰，冗余行如实申报不清洗）
- 无主闸 bypass-orphan 一笔：facet/probes 两件加 mcpline/ledger/tokens.ndjson 三件为主树批前遗留改件（mtime 早于本会话签发），非本批所改零触碰原样保留，候人节点按 watchcheck 协议二值裁决
- 金向量让位归并：主树未提交 1.43.0 冻结（他批遗留）让位本批 1.44.0 归并，备份存 /tmp/nomsupply-golden-backup-1.43.0.json，差异即版本位单字段，supersede 语义在档
- results 档差集认领一笔：本件即收约后交付件，--ack-uncommitted 事由在档，落主树候主线 campaign commit
- 环境性红两族如实申报（非本批码坏，主树同参对照在案）：write_gates 十四红为本会话持 sih-engine/doc 两锁与夹具 allow 前缀交叠渗读（mcpnomgate 隔离病族，放锁复绿），根治候隔离批；zero_write 套件窗证明红为并行簇窗口内落真台账行（前后全等判据按条呈报）
- 未决项：AGENTS.md MCP 节计数十八具待主线升十九具；mcpline CALL-LOG 投影腿候主线入版控；mcpline 测试隔离病族（夹具预检渗读真锁面）候根治批；map --concept MCP 投影候令；批词汇（nomsupply 加 naming_guide 加甲表加乙前注入）候立名程序人节点终裁登记，本批零写 terms.json
