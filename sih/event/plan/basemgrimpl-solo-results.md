# basemgrimpl-solo 结果档：基线向量管理工具实装与双机围堰向量迁入批

> 批：basemgrimpl-solo（文规向界工程批，委外单线 solo，零子代理）
> 会话：e850598a7109d5ae（lease 1.30.0，scope_source package）｜日期：2026-09-07
> 承接：用户 2026-09-07 令转发委外提示词；契约权威即孵化登记件基线向量管理工具节（CLI 三子命令冻结形不改文）

## 意图锚定

- 意图事件：intent_refined（event_hash `197c1c05`）
- record／validation：sih-tools/scribe/reports/2026-09-07-ask3-basemgrimpl-solo-record.json 与同名 validation（三锚：03-on-second-tao.md L15、07-on-assay.md L69、08-on-settle.md L52 程序切片）
- 前置：叩问 10 信号全轻 digest passed 10/10；正身 anomalies 0（identity `4fa19078`／core `82f460c2`）；watch 净态；心跳两线零告警；gauge 落链
- 排队候位：open 首跑被 preflight 拦（facepatch-solo 持 CALL-LOG 锁），候位约 4 分钟零写入后转至

## 件读数：三子命令与向量家

- 工具落位 `sih-tools/basemgr/`（uv src 布局：src/basemgr/engine.py 加 cli.py、tests 七测、fixtures 投影脚本加红样例加漂移样本、CONTRACT.md、pyproject、vectors/ 向量家）
- **向量件 schema**：冻结四元组（input_desc、context_desc、pathing、expected_file 加 command_cwd）加环境指纹（cwd_root、path_resolution、env_whitelist 最小空集起步、clock declared-fixed、T11 不完备声明）加版本三元积加六冻结对象声明位加 impact_set 加 expected_sha256
- **重放只读零写入**：指纹前置探针（cwd 与白名单逐键对表）不符 exit 2 拦截实测
- **重冻分种两通道**：基线种漂移查 impact_set——缺失即未分类 refuse exit 1 不重冻；规约种无新鲜红证 refuse exit 1（缺件态）；六冻结对象 --six-now 对表任一变更零豁免 refuse 须全量重算
- **甲乙红线**：报告 declaration claim=unchanged-only；现役逐字节门优先条款入 CONTRACT
- **freeze rc 校验**：基线种拒收失败命令输出（rc 非 0 或空输出）；规约种放行红输出（exit 1）

## 五子模块设计申报节（登记件授权）

1. 存储与 manifest：向量件两件组（元数据 JSON 加期望 .bin）家位 vectors/，index.json 目录索引
2. 冻结（freeze）：命令 capture、指纹实测、版本三元积显式传参
3. 重放比对（replay）：前置探针、逐字节比对、原始事实报文
4. 漂移归因（refreeze 内嵌）：impact_set 存在性判定映射（v0 简化，细分属后继）
5. 重冻分种（refreeze）：基线种归因前置、规约种先红前置、六对象变更零豁免

## v0 范围申报节（诚实边界，双申报）

- 覆盖机（探针跑、分块矩阵、覆盖率报告）属后继批：六冻结对象仅数据位在，消费逻辑除变更检测外未建
- 三态归因细分（真回归／工装非决定论判别）与申报影响集包含判定的全量形属基线向量管理工具覆盖机后继；v0 只做 impact_set 存在性判定
- 环境指纹白名单最小空集起步，不完备性承 SPEC-021 T11

## 双机迁移与锚位回填（F-2）

- 检查器 golden 六件：投影管道捕获迁入 vectors/checker-golden/，**迁移保真证 sem_eq 6/6**（git 原件 3a3dc342 逐件对表，findings 与 verdict 语义全等）；旧位 retired 桩留档
- TDD 验收工具 frozen 件：全报告捕获迁入 vectors/acceptor-frozen/，replay 零漂移证；旧位 retired 桩留档
- 寻径更新与版本进位逐件申报：检查器 manifest 0.2.0→**0.3.0**（baseline_vectors 节改迁移完成申报）；验收判定包 0.1.0→**0.2.0**（TC-002 frozen 改指新家 .bin）
- **D-4 锚位回填翻真**：五包 provenance.d4_anchor_slot.required false→true，vectors_home 指向新家（checkerimpl 批申报的激活路径就此走通）

## 自托管（F-3）

basemgr 自身 CLI --help 金向量冻结入 vectors/self/（指纹 cwd_root=工作区根），replay 零漂移跑通——首个真家位的自托管成立。

## 登记件验收判据对表节

| 登记件判据 | 实测 | 结果 |
|---|---|---|
| 判据一重放零漂移 | 冻结向量件同参重放逐字节一致 exit 0 | 过 |
| 判据二环境指纹 | 未声明环境变更的重放 cwd 不符样本前置探针拦截 exit 2 | 过 |
| 判据三重冻分种 | 基线种未分类漂移 refuse exit 1；规约种无红证 refuse exit 1；两通道逐一对表 | 过 |
| 判据四六冻结对象 | --six-now 变更样本零豁免 refuse exit 1 须全量重算 | 过 |
| 判据五双跑一致 | 冻结程序（本工具 replay）同参两跑逐字节一致 | 过 |

## 一裁读数

- 命题 m-basemgr-impl-1：facet 九发 9/9 comply、变卦 0%；同席采样谱系双声明载 topic authored 行
- tally assemble stable_clear；attractor check 裁决通过 failed 0；verify identical；**sign b1c925a7**（doc_id crosscheck-m-basemgr-impl-1）

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 三查实测 | 工程 | 重放零、指纹拦二、重冻分种两通道对表 | 通过（逐件实测） |
| F-2 迁移零漂移 | 工程 | 双机迁入重放零漂移、旧位留档、进位申报 | 通过（sem_eq 6/6 加 replay 证） |
| F-3 自托管 | 工程 | 自身金向量冻结入自身家重放零 | 通过 |
| F-4 先红后绿 | 治理 | 红证在档后全绿 | 通过（7 failed 归档→7/7 绿） |
| F-5 空腹证明 | 工程 | grep 零项目字面零命令字面 | 通过（test_empty_stomach） |
| F-6 终签在链 | 治理 | stable_clear 终签入链 verify valid | 通过（b1c925a7） |
| F-7 写入仅 allow | 治理 | 零租约零引擎零 sih-math；对表入档；遗留零触碰 | 通过 |

## 越线与误差申报

1. **冻结工序失序一笔（已补救）**：checker golden 原位桩化先于保真证执行（应证后桩化）——原字节从 git 提交 3a3dc342 恢复对表补救，sem_eq 6/6 证在档；流程教训申报
2. 投影形两轮返工：首版捕获成全报告与旧精简形 form drift（sem False 信号触发诊断），改 sh -c 投影管道保真后 sem_eq 全过；全报告版一度误冻（freeze rc 校验拒空输出拦截在先）
3. 开发期中途红多笔（向量名形、测试路径、--command nargs 旗标截断改 --command-json as-built、argparse --root 双注册、元数据 input_desc 与管道符冲突改结构化 command 位、self 指纹空变量）均在实装至绿窗口修复；首跑红证以空实现归档件为准
4. 化格 19 件批跑 exit 2 改逐件跑全过——工具批面异常如实申报候查（不属本批处置面）
5. 检词批含一件不存在路径误参（basemgr/packs 笔误）exit 2，剔除后九件全绿零违例
6. migrate 后 checker tests 的 golden 精简形引用为连带改写面（属 checker 工具后继批寻径更新，本批 allow 面不含 checker/），如实申报
7. TC-003 类 uv 运行时缓存同前批申报同形

## 收口读数

- 双仓 settle：tools 段1（归并在档）、engine 段1 加结果档（差集闸在位声明件先提交）；cert 取 `eb7ab5b8`
- 放锁收约：十三锁 unlock 全放；close exit 0 会话 e850598a7109d5ae revoked
- 链 verify：2026-09-07 当日链 valid
- reconcile：双仓 unrouted 0，cert_missing 相比批前零新增
- 心跳复算：两线 exit 0 零告警
- 本结果档经收约补笔 bypass 通道回填结算读数入版控（先例同形）
