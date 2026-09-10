# 司衡路径写死只读排查报告（簇C，2026-09-11）

范围：sih-engine/src 与 sih-engine/src/bin（Rust）、sih-tools 各工具 src 与工具顶模块（Python）。
排除：target、.venv、node_modules、worktrees、dist、__pycache__、静态资产。
定级：真患=域外场景会错行为；有界=现状正确但假设未声明或未测；误报=正典常量或测试专用。

## 一、方法

1. `grep -rn` 全量扫描八类候选（/Users 绝对路径、仓名常量、sih-engine/sih-tools 字面 join、sih/ 布局、端口主机、SIH_* env、tests/probes/reports 真_root），先扫后读。
2. 每处候选逐处读上下文（读函数体与 docstring 与测试边界）判级，不凭 grep 行定患。
3. 基线件逐件重读核验封闭性（见第二节）。
4. 实存核验：`ls` 验 sih-tools/scribe/trail（老家，现存 2026-08-28.ndjson 一件）、sih-engine/sih/event/trail（新家，多件）、<根>/sih（仅含 event，无 ledger，即主根非 canonical 形）。

## 二、基线排除与封闭性核验（只验不改）

- critsweep derive_root（sweep.py:31-42 arg>walk>env 三级）与 detect_layout/domain_faces 双形（sweep.py:44-87）：**仍封闭**，first_domain 泊界路径逐字节不变、canonical 面 sih/ledger|sih/event/trail 齐备。
- lease rootanchor 域感知（core.py discover_workspace_root 标记上溯 + is_canonical_domain + detect_domain_context canonical 先判）：**仍封闭**；_stem_gate_first_domain（core.py:628-631）为已知设计位，本批仅标注不计新患。
- gauge 域件（cli.py 根解析 canonical 先判、_central_code_root 双仓上溯、_guard_same_domain 跨域卫 fail-closed）：**仍封闭**。
- mcpline 两根分离（runtime.py:21-39 resolve_root SIH_ROOT>parents[4]、code_root SIH_MCPLINE_CODE_ROOT>同源回落）：**仍封闭**。
- 引擎 retriever derive_root（worktrees 层恒跳过）+ layout_form 双形判（mod.rs:390-407）：**仍封闭**。

## 三、分级清单

### A. 真患（2 件）

A1. `sih-tools/locks/src/locks/cli.py:27-30` —— `trail_dir = Path(root) / "sih-tools" / "scribe" / "trail"`（_default_trails 只枚举老家单居所）。
患义：lease 同名函数（lease/cli.py:214-219）显式双居所并集并注明"引擎新家加工具侧老家，迁链路标机械位承 pk-031"，locks check 漏新家 sih-engine/sih/event/trail；check_baseline 对 unknown_target 不拦（core.py:333-338 fail-open），新家链证（2026-08-28 后全部）游离于乐观锁基线外，脏上游漏拦。
修法：locks/cli.py _default_trails 改双居所并集，与 lease 对齐（或收單一域感知函数）。

A2. `sih-tools/mcpline/src/mcpline/runtime.py:97-103` —— `trail_path`: `root / "sih-engine" / "sih" / "event" / "trail"`；`scribe_bin`: `code_root() / "sih-engine" / "target" / "debug" / "scribe"`。
患义：mcpline 读/写面缺省路无视 DES-015 canonical 两形——writeface/domains.py 有 DomainLayout 但仅域令牌绑定注入，server.py 读函数 layout=None 缺省分支零 canonical 探测（grep 证）；SIH_ROOT 指 canonical 域根部署时读面默认路全指 <域>/sih-engine/... 错位，scribe_bin 另绑 target/debug 构建档。有界化条件：若部署合同明文限定 mcpline 中央面不直服 canonical 域，则降有界。
修法：缺省分支接 detect_layout 同构判定（critsweep detect_layout 已有先例），canonical 形走 <域>/sih/event/trail；scribe_bin 走 profile 无关解析或报文如实。

### B. 有界（前五）

B1. `sih-engine/src/retriever/locator_bridge.rs:39-52,178` —— canonical 形码根自祖先上溯取 `sih-tools/locator`；query_word_entries/query_by_id 以祖先仓 `packs/memory/pack.json` 为 --pack，而 build 用内嵌 CANONICAL_MEMORY_PACK。患义：两包无一致性声明，漂移即建查不一致；祖先上溯亦可能拾取异工作区 locator。修法：canonical 查询统一用内嵌表或加 pack 摘要对表。

B2. `sih-engine/src/scrutinator/rule.rs:214-215` + `bin/scrutinator.rs:36-44` —— `WORKSPACE_ROOT_MARKER = "sih-tools/lease/ledger/sessions.ndjson"` 上溯判根。患义：标记件绑 first-domain 台账位，canonical 域零标记即判域外（纵深守卫 fail-closed 是有意的），但 canonical 域内文件永远无法入域清单判定，此边界未测。修法：标记件族补 canonical 形（sih/ledger 目录）第二判。

B3. `sih-engine/src/event_stream/tdd_tests.rs:55-59` —— T2 生产 trail 测试硬编码 `["../sih-tools/scribe/trail", "../../../sih-tools/scribe/trail"]`。患义：first-domain 共存 + 老家单居所双假设，canonical/纯新家形下 cargo test 红。修法：改双居所并集或 --ignored 化。

B4. `sih-tools/cascade/src/cascade/core.py:43` —— `root = Path(__file__).resolve().parents[3] / "parser"`（同型：lease/core.py:630 `_nomenclator_project_root` parents[3]/"nomenclator"）。患义：sih-tools 兄弟仓字面 join，pip 装机/异地部署即断（PARSER_ROOT 可注入、缺席 fail-closed 报文如实，故有界）。修法：接两根分离式码根 env（承 SIH_MCPLINE_CODE_ROOT 先例）。

B5. `sih-tools/mcpline/tests/conftest.py:17-18` —— `_ROOT = Path(os.environ.get("SIH_ROOT", "/Users/moc/workspaces/SiHankor")); os.environ["SIH_ROOT"] = str(_ROOT)`。患义：alpha 面缺省硬指真仓并在 import 期覆写进程 env，换机 CI 即红（docstring 自报"固定于真工作区根"，声明在）。修法：缺省回落仓相对推导（git toplevel 或 env 必填校验）。

### C. 有界（其余，逐条）

- C1 `sih-tools/locks/src/locks/cli.py:23` _default_root=parents[4] 定深，无 walk 无 env（--root 可覆写）；:74,78-80,102-104 worktree ledger/registry(si-engine/doc/CASCADE.json)/doc_root/reports_root/cascade 兄弟仓 first-domain joins——canonical 域缺件走 LocksError fail-closed。修法：缺省根接 walk。
- C2 `sih-tools/locks/src/locks/core.py:22` `DOC_PREFIX = "sih-engine/doc/"` 域硬绑——合同限定引擎 doc 语料，canonical 域不可用但报文如实（:327-328）。
- C3 `sih-tools/lease/src/lease/cli.py:1240-1245`（check/commit 同款）registry/doc_root/reports_root/cascade first-domain joins——canonical fail-closed。修法：接 detect_domain_context 分形。
- C4 `sih-tools/lease/src/lease/core.py:100-127` resolve_root/_gate_root 双仓标记（sih-engine/Cargo.toml+sih-tools/pyproject.toml）上溯 + parents 定深回落（回落已声明可预测失败）——rootanchor 基线余部。
- C5 `sih-tools/lease/src/lease/core.py:129-135,217-222` worktrees 字面段判工地（src/cwd 含 "worktrees"）——worktree 目录名字面假设（rootanchor 设计件，rootanchor_disable env 有）。
- C6 `sih-tools/attnanchor/anchor.py:23-29` root() 标记件上溯 > ZCODE_PROJECT_DIR/CLAUDE_PROJECT_DIR > parents[2] 三级回落；:46 inflight 读 sih-tools/lease/ledger——scrutinator rule.rs 自认先例位；canonical 形落到 env/定深级。
- C7 `sih-tools/watchcheck/src/watchcheck/cli.py:18-27` + `core.py:25` `REPOS = ("sih-tools","sih-engine")`——双仓前提写进 --root help 与常量，trail 缺席 ToolError fail-closed（core.py:93）；声明在、canonical 无分支。
- C8 `sih-engine/src/retriever/mod.rs:668-672` load_chain `_ =>` 回落 root/sih-engine/sih/event/trail——unknown 形根误指 first-domain 形，但读缺报 TargetUnreadable 如实。
- C9 `sih-tools/proposition/DES/leaseopt-audit/leaseopt-census.py:16-29` walk-up 标记件（sih-tools/lease/ledger/sessions.ndjson）+ 全套 first-domain joins——一次性审计脚本，重放绑主树形。
- C10 `sih-tools/critsweep/make_registry.py:21` `ROOT = Path("/Users/moc/workspaces/SiHankor")`——registry 再生器缺省绝对路径（--gov/--out 参化已备）。
- C11 `sih-engine/src/attractor/route.rs:976` 测试 fixture 真 root "/Users/moc/workspaces/SiHankor/sih-engine"（#[cfg(test)] 内，fail-closed 测试，路径值不敏感）。
- C12 `sih-tools/facet/src/cli.py:10` PROJECT_ROOT=parents[3] 定深（=sih-tools 仓根）；`sih-tools/tally/src/tally/cli.py:281` repo_root=des_root.parent.parent DES 材料两级行距假设（合同内）。
- C13 类7 测试真仓真册绝对路径（14 行）：mcpline tests/{conftest,fixture_root,test_stdio_smoke,test_write_gates}、lease tests/{test_stemgate REAL_ROOT,test_dogfood_crossroot,test_calllog}、parser tests/{test_f2_xref,f2_support LOCATOR_DIR,test_tokencap}、watchcheck tests/test_leaseup_watch（真 trail 真锁册）、acceptor tests/test_engine、basemgr tests/test_engine——多数为域绑定集成测试的既定形态；mcpline 已自报，parser/watchcheck 未声明。
- C14 类2 仓名常量入逻辑：lease tests/test_dogfood_crossroot.py:66 断言根含 "/SiHankor" 段；lease/core.py discover_workspace_root 标记常量 ".sihankor-workspace"（rootanchor 基线件内）。
- C15 类6 env 回退余部：anchor.py env 第三级回落（跨 workspace 会话 env 污染为理论患，walk 先行护住）；lease/cli.py:131 dogfood 双 env（SIHANKOR_CALLLOG_DOGFOOD_ROOT > SIHANKOR_WORKSPACE_ROOT）自担申报设计。

### D. 误报（看似路径，实为正典常量/测试专用）

- 端口/主机全数正典位：mcpline web.py:50 DEFAULT_HOST="127.0.0.1"（DES-015）、init.py:308 CLIENT_FACE_URL 8765、httpface.py:647 与 server.py:59 使用说明文案 8765；facet/tests port 9 测试专用。类 5 新患：**零**。
- mcpline/runtime.py:17-18 CANON_SPEC_023/CANON_LINE_PKG 文档指针常量。
- sih-engine/src/event_stream/lockgate.rs:55-56 台账上四级定根——入参台账形不变量、文档已声明、路径参驱动自洽。
- 包内相对 parents：meter/cli.py:27 counts、facet paradigm_loader/llm_client/thinking_resolver/env_loader、lease claimcore:48/commitcore:372 hooks、locator carriers/code.py:12 packs、mcpline web.py:115 说明书、gauge cli parents[2] 仓根——皆工具包内正典。
- 引擎 bins 零 env/端口/root 推导（ask3repeater --root 必参拒缺省、retriever 用 derive_root 基线）——类 6 引擎侧：零命中。
- scribe/reports 68 件一次性报表生成器真根绝对路径——历史批材料非工具运行面（同时计入 C 类批量，不计患）。

## 四、八类命中总账

1. /Users 绝对路径：96 行（scribe/reports 68 件、tests 14 行、probes/contracts/DES 批材料 8 行、make_registry 1、route.rs 测试 1、其余同上）——真患 0，运行面零硬编码。
2. 仓名常量入逻辑：3 处有界（C14），余为 docstring/提示词文案非逻辑。
3. first-domain 共存：真患 1（A1 双记本类）、有界 9；_stem_gate_first_domain 已知设计位已标注。
4. sih/ 布局无视 canonical：真患 1（A2）、有界 2（B3、C8）。
5. 端口主机越典：零命中。
6. env 回退交叉域：真患 0、有界 2（C15）；基线两根分离封闭。
7. 测试/fixture 真仓真册：14 行有界（C13）+ route.rs 1（C11）。
8. 其他路径假设：真患 1（A1 主记）、有界 6（B1、B2、C1、C12、C15、lease worktrees 字面判）。

误报计数：约 13 位（端口 5、指针常量 2、lockgate 不变量 1、包内 parents 5）。

## 五、总体判词

域感知基线五件（critsweep derive_root/detect_layout、lease rootanchor、gauge 域件、mcpline 两根分离、引擎 retriever derive_root/layout_form）经重读全部仍封闭。真患仅 2 件，同属两族："trail 迁新家后旧缺省未跟"（locks check 老家单居所，与新家链证实际并存，现势已漏）与"canonical 域缺省读面未分形"（mcpline runtime 默认路 + target/debug 构建档绑死）。有界 15 位集中于测试真仓依赖、一次性批脚本与 fail-closed 的兄弟仓 join。绝对路径患全部停在一次性/测试材料，工具运行面零 /Users；主机端口全在 DES-015 正典位。建议修序：A1（现势漏拦，最小改动双居所并集）→ A2（分形判定接缺省分支）→ B1（建查包一致性）→ B2（canonical 标记第二判）。
