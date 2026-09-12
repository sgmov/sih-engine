# DEC-022 发布版本定约决策

本决策成立司衡引擎发布线的版本语义与 1.0.0 机械晋升判据与治理语义 SemVer 定约，令源即用户 2026-09-09 前令接受 0.9.0 首发布与 1.0.0 机械晋升判据方案。版本号是治理承诺的载体：0.9.0 立测试味首发布位，1.0.0 立稳定位，晋升由机械可验判据承载，不由印象承载。本令同时立 MAJOR 与 MINOR 与 PATCH 三位的治理语义，版本位升级条件自本令起可对表可复算。相名转写声明：本令承域字符集闸一律拉丁转写相名，alpha 相与 beta 相指称希腊字母原名的同一相，承 SPEC-023 转写先例。

## 概览 {#overview}

- 0.9.0 首发布语义即测试味版本，alpha 相只读 MCP 面在列，稳定兼容承诺零::[0.9.0 语义](#rel-090)
- 1.0.0 晋升三判据俱机械可验，每条载判定命令或材料指针，判定读数以晋升批当批重放为准::[晋升判据](#promotion)
- 治理语义 SemVer 定约三位即 MAJOR 治理语义破坏、MINOR 判定语义增量、PATCH 执法面修补::[定约](#agreement)
- 版本载体即 git tag 与 Cargo 版本位双载体同批对表，失对表即发布无效::[版本载体](#carrier)
- 适用界自 0.9.0 起算不溯往，判据立约后不放宽，工具线版本面不入辖域::[适用界](#scope)
- 备选两案即感觉成熟定版与日期定版俱拒绝::[备选方案](#alternative)

## 0.9.0 语义 {#rel-090}

0.9.0 是司衡引擎的首发布版本，定位测试味版本：对外可用，向后兼容承诺零，接口与行为面可在 1.0.0 前随 MINOR 位调整且零预告废弃义务。0.9.0 发布内容面以发布清单为正典，见 `sih-engine/doc/plan/release-0.9.0-v1.md`。alpha 相只读 MCP 面在列：mcpline 五读数工具按 SPEC-023 契约随 0.9.0 对外暴露，冷 agent 线级验收已成立在案，见 `sih-engine/sih/event/plan/mcpline-alpha-settlement-v1.md`。beta 相写面不在列：写面候会话与租约映射安全模型设计批过闸，未过不开工，承 SPEC-023 红线。

## 1.0.0 机械晋升判据 {#promotion}

1.0.0 晋升当且仅当三判据同时成立。每条判据载判定命令或材料指针，判定动作零裁量；判定读数以晋升批当批重放为准，历史读数不代判。禁感觉成熟形：任何判据不得以趋稳、成熟、质量良好类印象语作答。

### 判据一 冷 agent MCP 面零辅助跑通 {#crit-cold}

零上下文外部 agent 经 MCP 面零辅助完成一次被治理交互且链 verify valid。判定材料指针：冷 agent 起跑记录与 transcript 归档，形承 mcpcold 先例 `sih-engine/sih/event/plan/mcpcold-solo-materials/cold-launch-record.json` 与 `cold-transcript-extract.json`。判定命令：晋升批重放冷 agent 场景后跑 `sih-engine/target/debug/scribe verify --trail sih-engine/sih/event/trail/<实日>.ndjson` 退出码零，且 transcript 含 chain_query 与 chain_verify 调用各至少一笔，判定三要素照 `mcpcold-solo-results.md` 第一节即零求助、零越界写、判词 valid，任一缺即判不成立。

### 判据二 陌生人零帮助指南走查 {#crit-walkthrough}

零语境陌生人仅凭库内指南零帮助走通入门到首次被治理交互全程。判定材料指针：走查底本 `sih-engine/doc/guide/user-guide-v1.md` 与 `sih-engine/doc/guide/contributor-guide-v1.md`；走查记录件落晋升批材料，ndjson 形每步一行载 step 与 cmd 与 exit 三字段。判定命令：`grep -c '"exit": 0' <走查记录件>` 计数与总步数对表相等，且 `grep -cE '"exit": [1-9]' <走查记录件>` 输出零。走查过程零场外求助零口头补提示，求助即判不成立；走查人不限于 agent，库外志愿者与外部用户走查记录同效。

### 判据三 发布后修复节奏建立 {#crit-fixcycle}

0.9.0 发布后修复动作有制度位可循且实测走通。节奏制度位即本定约 PATCH 位与发布清单回滚法；实测成立判据即发布 tag 之后存在至少一笔修复类批全链收约。判定材料指针：`sih-tools/lease/ledger/sessions.ndjson` 与当日链。判定命令：对 sessions 台账 `grep -c '"package":"<修复批名>"'` 输出大于零，且该批会话号在链上的 certification_completed 笔经 `sih-engine/target/debug/scribe confirm --trail <链> --hash <该笔哈希>` 确认在档；首笔修复批收约即节奏建立，后续修复按定约 PATCH 位递增。

## 治理语义 SemVer 定约 {#agreement}

SemVer 三位各承治理语义，版本位升级由内容面判定不由数字偏好判定，位判定争议归批令留痕裁量，不由发布执行人临场定夺。

MAJOR
: 治理语义破坏。治理语义即链面事件语义与判据语义与执法语义与工具调用契约：MCP 面工具契约破坏、链 verify 语义破坏、租约与锁语义破坏、des-001 类规则包语义破坏，任一在即 MAJOR 位递增，兼容迁移路径必附。

MINOR
: 判定语义增量。判定面新增或收紧即 MINOR 位递增：新判据、新读数维度、新只读工具、新执法面检查项，向后兼容。

PATCH
: 执法面修补。执法面即既有判定语义的执行载体：缺陷修复、报文修正、性能修补、文档勘误，判定语义零变化，PATCH 位递增。

## 版本载体 {#carrier}

双载体同批对表：git tag `v<三位版本>` 落 sih-engine 仓，`sih-engine/Cargo.toml` version 位同批改至同值。判定命令：`git -C sih-engine tag -l "v*"` 与 `grep "^version" sih-engine/Cargo.toml` 输出对表相等。两载体失对表即发布无效，处置归发布清单回滚法。sih-tools 各工具自带版本面如 lease 1.39.0 不入本定约，本定约只辖 sih-engine 发布线。

## 适用界 {#scope}

本定约自 0.9.0 起算，既往版本实践零追溯。1.0.0 判据立约后不放宽：放宽即定约破坏，须显式决策留痕，承命名承诺不撤回之义。工具线版本面与数学仓版本面不在本定约辖域。

## 备选方案 {#alternative}

感觉成熟定版
: 已拒绝
: 以趋稳、质量良好类印象语判 1.0.0，判据零机械可验性，主窗独立复算零落点，违可验证性约束。

日期定版
: 已拒绝
: 以日历日判晋升，版本位与内容面脱钩，破坏定约的治理语义承载。

## 与既有治理件的关系 {#relation}

SPEC-023 承 alpha 与 beta 分相红线，本定约 0.9.0 内容面照录其相界；GOV-002 判据面变化按 MINOR 位对表；融回门与视图组件按各正典运行不受本定约影响。1.0.0 晋升批照批机械链全序执行，三判据材料落批材料并上链认证，主窗独立复算。

## 修订记录 {#revisions}

2026-09-12 修订一：随 sddgate-solo 批增第四晋升判据，令源即用户 2026-09-12 判词「sdd是司衡引擎事前优化 ai 即 agent 与 llm 代码生成的重要功能之一，应该作为上线前的重要指标」。判据四 SDD/TDD 完备度闸在役：格式包检查器工作名件在役吃 sdd-v1 五件格式包对 SDD 文档族机械判定三值退出码，lease close 轴两道门 SDDG 四判据执法在役，判据正典 DEC-024 在典。判定命令：晋升批当批重放 sddgate 判定命令四条俱过即 close 门零绕行或绕行有显式事由在 bypass 台账，且格式包检查器对晋升批 SDD 文档族退出码零；判定材料即 close 链闸记录与 bypass 台账与检查器报告，重放零裁量。判据正典单源 DEC-024，本条只作引用不复制。原三判据语义零变化。
