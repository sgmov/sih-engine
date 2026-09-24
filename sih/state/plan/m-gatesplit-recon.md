# m-gatesplit 侦察：src/attractor 三处落点改动面清单

- 侦察范围：worktree `/Users/moc/workspaces/SiHankor/worktrees/sih-engine/m-gatesplit`（分支 msh/m-gatesplit，HEAD d371930）
- 性质：只读侦察，零改码零写入（本档除外）
- 日期：2026-09-24

## 一、裁判提示词生成处

提示词模板本体不在 Rust 源码内，是外部 atom.yaml 数据文件；引擎侧只做模板装载与标记替换。构造链四件：

| 件 | 位置 | 职能 |
|---|---|---|
| 模板装载 | `src/attractor/paradigm_loader.rs:23-26` `load_atoms` | 读 atom.yaml，返回 atom 名→配置映射（serde_yaml） |
| 提示词构造 | `src/attractor/contract_mode.rs:64-95` `build_integrator_prompts` | atom `integrator` 第一方向（judge）出 system/user；user 先 `{topic_content}` 后 `{proposition}` 双替换（L74-76）；system 按围堰序替换七标记（L82-90）：`{round}`→1、`{prev_round}`→0、`{prior_facets}`→空、`{normative_guidance}`→ng 文本、`{divergence_shape}`→`EMPTY_SHAPE_MARKER`、`{role}`/`{role_prompt}`→空 |
| 空发散发谱锚 | `src/attractor/contract_mode.rs:28` `EMPTY_SHAPE_MARKER` | 现文「（无前置微分数据，发散发谱为空）」，合同提示词防漂移锚 |
| shots 与落盘 | `src/attractor/contract_mode.rs:98-110` `make_shots`；`113-140` `emit_contract` | 同一提示词 n 发展开为 shots（key 形 `<gid>#r<k>`）；合同确定性落盘（canonical_json，同输入逐字节一致） |

CLI 入口：`src/bin/attractor.rs` —— USAGE L23-26（`emit-contract: --topic --ng-file --seat --gid --title --shots --atoms <atom.yaml> --ng-label --out [--paradigm-id] [--atom integrator] [--direction judge]`）；atoms 路径取参 L71、装载 L101、构造调用 L105、落盘 L114。

模板本体（搜索字符串命中处，仓库内范本即金向量夹具同件）：
`src/attractor/fixtures/golden/contract-emit/atom.yaml`（773 行）——
- L719 `integrator:` atom；L731 `- id: judge`；L733-751 `system_prompt_template` 现文（「你是治理规约裁定者。严格依据以下规约判定命题：…」）
- L738「高发散意味着本命题可能落在规范边界，须更谨慎判定」；L751「- boundary_flag: 本命题是否落在规范边界（基线不直接回答、须推理适用）」
- L745 输出格式四键 JSON schema 声明；L754-760 validator `json_schema` required_fields 四键
- 生产调用位 atom.yaml 由调用方 `--atoms` 显式传位（paradigm_loader 头注 L5-6：路径由调用方显式给位，围堰缺省路径语义不迁）

改动面提示：动 atom.yaml 模板文本即动合同哈希；合同哈希绑定响应哈希绑定裁决材料（contract_mode.rs 头注 L9-11），且金向量 `contract-emit`、`*-net/expected/contract.json` 全部哈希冻结（见第四节），牵一发动冻结面。

## 二、响应解析处（裁判 JSON 四键）

两条同语义合成路径：

1. 合同模式主解析（单席位采样腿）：`src/attractor/contract_mode.rs`
   - `parse_raw_answer` L253-290：围栏剥离（``` 剥壳）→ 花括号切片（首 `{` 至尾 `}`）→ 四键提取 `decision`（缺省 no_answer）/`basis_regulation`（缺省空串）/`boundary_flag`（缺省 false）/`reason`（缺省空串），附 `void` 标记
   - 解析失败不静默：L283-289 落 `no_answer + boundary_flag=true + reason="non-JSON output: …" + void=true`（空转标记，复现即作废语义）
   - `dc_from_responses` L293-318：响应列表 → per_actor 七字段（actor_id/model_id/family=Seat/decision/basis_regulation/boundary_flag/reason）+ voids 明细
2. 多 integrator 聚合路径：`src/attractor/compiler.rs`
   - `extract_decision_fields` L592-607：四键 required 数组 L597 `["decision","basis_regulation","reason","boundary_flag"]`，任一缺席返回 None；boundary_flag 强转 bool 缺省 false（L605）；依赖 `parse_payload` L50
   - 上游严格对表：`load_responses`（contract_mode.rs L171-249）——每行恰三键 key/shot/raw、key 全在合同、无重复、shot 对齐、零缺失，违例拒收整批

## 三、成熟度判据处（三态判定）

关键事实：**引擎侧不产三态判词**。`src/attractor/contract_mode.rs:455-456` 现文自述「gate_verdict 为闸上游产出（判据 v3 闸留围堰，机械腿只装配不判闸）」；`score_pipeline` L457-485 只把 `gate-verdict` 字符串参数透传进计分材料（L482）。

三态计算正典在围堰（工作区主仓，非本 worktree）：`/Users/moc/workspaces/SiHankor/sih-tools/facet/probes/maturation_gate.py` `assess_maturation_v3` ——
- 阈值常量（L109-111）：`decision_flip_threshold=0.2`、`boundary_rate_threshold=0.34`、`near_margin=0.10`
- boundary_rate 计算 L136-140：`boundary_rate = round(sum(bflags)/len(bflags), 4)`；`boundary_low = rate < 0.34`；`boundary_near = 0.24 <= rate < 0.34`
- 三态裁决 L172-182：任一子判据明确挂 → `boundary`；全过但有 near → `near_threshold`；全过无 near → `stable_clear`
- 附体温参数化（V3-T，L264 起）：boundary 判据相对席位家族体温，`effective_threshold = auto_baseline × user_posture_coefficient`（strict 0.8 / lenient 1.2）

引擎侧相关机械面（本 worktree 改动落点）：

| 位置 | 内容 |
|---|---|
| `src/attractor/compiler.rs:610-719` `compute_decision_convergence` | 单 run 聚合：`declared_boundary_rate = boundary_count/n`（L669）；阈值 0.5/0.5（L670-671 `is_boundary`/`is_converged`）；四格 cell L672-678：`boundary_converged / boundary_diverged / clear_converged / clear_diverged`（注意这是 cell 四分格，非闸三态） |
| `src/attractor/tally.rs:33` | `CRITERIA_VERSION = "v3"`；L35 `BUDGET_PER_GID = 9` |
| `src/attractor/tally.rs:216-219` | R3 校验 gate_verdict ∈ {stable_clear, near_threshold, boundary} 三值集合，出集即 R3 失败 |
| `src/attractor/tally.rs:375-387` | 三态映射四值处置（DES-011 优先级）：席位 suspend/abnormal→挂起 先于一切；规则失败→材料退回；`boundary`→打回重作；`near_threshold`→挂起；`stable_clear`+全过→裁决通过（方向沿用席位众数 `modal_direction` L113-140） |
| `src/bin/tally.rs:38` | `GATE_VALUES: [&str; 3] = ["stable_clear","near_threshold","boundary"]`；L786 near_threshold 同构处置 |
| `src/pendline/route.rs:159-165`、`src/pendline/dispatch.rs:177-186` | 判词直读 gate_verdict（三值闸词表）优先、回落 verdict 字段，零计算零判定 |

若本批要把判据 v3 闸从围堰融回引擎，新计算件落点即 contract_mode.rs L455 注释所指的空位，消费面为 tally.rs R3 与三态映射、pendline 两处直读。

## 四、测试与夹具

测试（本 worktree tests/）：
- `tests/attractor_contract.rs`：`t4_assemble_field_compat_then_check_passes` L46、`t4_engine_signcheck_passes_guard_crosscheck_unchanged` L101、`t4_multi_gid_no_cross_contamination` L144、`t4_leg_split_manifest` L166、`t6_rules_version_shape` L219、`t6_tri_state_mapping_four_dispositions` L238（三态→四处置正测）、`t6_idempotent_replay_no_history_rewrite` L270
- `tests/attractor_golden.rs`：`t1_golden_frozen_with_dirty_targets` L60、`t1_contract_and_score_golden_frozen` L107、`t2_check_reports_byte_identical_all_scenarios` L124、`t2_signcheck/score_material/contract_emit/rejection_envelopes_byte_identical` L146/162/192/229、`t2_frozen_golden_zero_drift_after_freeze` L247、`t2_stats_conclusion_equivalence` L265
- `tests/attractor_cli.rs`：`t3_check_exit_codes` L54、`t3_sign_tri_state` L80、`t3_verify/watch_two_values` L130/150、零网络零 LLM 三扫 L185/205/218
- `tests/attractor_route.rs`：金向量冻结、退出码表、零网络源扫、双模对表 L343

金向量（`src/attractor/fixtures/golden/`，`golden-manifest.json` 全件 sha256 冻结，baseline 声明「围堰 Python 原件（freeze_golden.py 驱动，唯一基准禁自造）」）三态覆盖矩阵：

| 场景 | gate_verdict | disposition |
|---|---|---|
| adisp-net / p3xcarr-net / dirty-alarm-r7 | stable_clear | 裁决通过 |
| dirty-return-r2 | stable_clear | 材料退回（R2 哈希不符） |
| dirty-suspend-near | near_threshold | 挂起 |
| adisp-guard-1-net / m-p3xcarr-net | （score-material 场景，无 check 腿） | — |
| reject-* 四件 | — | 响应对表拒绝信封 |

覆盖结论：stable_clear 与 near_threshold 两态有金向量；**boundary 态（→打回重作）无金向量场景**，判据闸若融回需补 boundary 场景冻结件。提示词文本与解析改动均落 contract-emit/expected/contract.json 及各 net 场景哈希冻结面，改后须围堰 freeze_golden.py 重冻并双跑对表。

## 范畴排除声明

本侦察只读不改码，未跑 build/test，未覆盖围堰 facet runner 侧执行腿（sih-tools/facet 出题执行与 maturation_gate.py 逐行为只作定位引用未逐行核验）；sddgate（src/bin/lease/sddgate.rs）同名词 gate 属租约收约闸，与本三态闸不同物，未展开。
