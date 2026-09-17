# mcpnomgate-solo 批任务包：检词上 MCP 写面——lease open stem 查册闸与两具投影与开闸三毛边

> 令源：用户 2026-09-10 令「同意，这个是既有功能没上mcp」承 DEC-017 修订四常设纪律（工程命名动作前查命名集与检词，2026-08-28 立令）兑现定性；问题源即司梦侧 M1/M3 源码级报告（对表判词在当日会话：检词零引用、闸链零词典位、意图标签错指、空 allow 静默）
> 形：solo 批独立立约独立收约；第二批（M2 域感知：判据扫认新城形、rootanchor 域感知、close 回执 gates_skipped、gauge 域件）候令另开
> version: v1

## 一、使命

把 2026-08-28 既裁常设纪律「工程命名动作前查命名集与检词」从 skill 文档载体搬进机械闸位，并把检词查询核查两具投影上 MCP 工具面；顺手修开闸三处毛边。判定语义零新增——闸的每格判词都循 nomenclator 既有六态，新加的只有执行位与认领旗标。

## 二、关键设计

**件一 stem 查册闸（lease open 新闸位，core.py）**

- 触发：lease open，对包 stem 生效；仅本域会话拒，新城正典域只教学（域判别承静态规则：root 下 sih-engine 与 sih-tools 两目录俱在即本域，余即新城形——与 writeface/domains.py 判别同构）
- 分段：stem 按 - 与 _ 切分，弃结构词（solo、纯数字、pkNNN 形），余段逐段过 nomenclator query（子进程 uv run --project <root>/sih-tools/nomenclator nomenclator query --pack <root>/sih-tools/nomenclator/packs/core --word <段>；包位经根锚 root 解析）
- 判词（循 query 既有 state 六态，零新造判定）：
  - established → 过
  - dead → 拒（死档在档，翻案须显式推翻原死因）
  - lazy → 拒教学（懒波在档：升格走登记或另择名）
  - candidate → 拒教学（候补在档：呈报既有条目阻断重推）
  - unknown（零命中）→ 本域拒教学 + `--new-stem` 认领旗标可过（认领即铸名声明，出参载 stem_check: new_coinage_acknowledged 入回执）
  -新城域 → 任一态只教学不拒（DES-016 中性化裁定同形：外部域零代强制），教学语指中央词典与立名程序
- 闸失败形态：nomenclator 工具缺席或 query 异常即退出码二 fail-closed 教学——闸不静默跳（M2 病灶防复发）
- 出参：open 回执载 stem_check 字段（checked、segments、verdicts、disposition：established_pass 或 collision_rejected 或 new_coinage_acknowledged 或 external_teaching）
- 闸位次：插包校验之后、正身闸之前（命名是包的属性，最早拒最省）；既有六位闸序位次顺延申报

**件二 MCP 面两具投影（mcpline server.py 与 httpface.py 与 writeface）**

- nomenclator_query（入参 word）与 nomenclator_check（入参 target 文档路径）：只读 CLI 透传，出参透传，退出码透传
- 面计数：stdio 17 升 19；HTTP 15 升 17（alpha 只读面五具扩七具）；assert_http_face 组成断言同步；授权矩阵两行（local 与 external 俱可调，只读形）
- register 不投影（登记=立名入口，外部面不给写词典位——红线）
- SPEC-023 修订一：alpha 只读面契约增两具登记

**件三 开闸三毛边（lease core.py 与 cli.py）**

- intent 先验：意图件存在性、可读、JSON 合法、对象形的验前移到身份与血统闸之前，报文正标签「意图记录不可读：<路径>」「意图记录非合法 JSON：<路径>」（现形「identity file unreadable」错指对象）
- 空 allow 教学：allow 空数组时回执载 allow_empty_teaching 字段（写入范围空：逐锁将 scope_violation；重开带 --allow），不拒——read-only 会话形仍有合法空
- scope_violation 拒透传报文补一句指向（重开带 --allow）

**件四 版本与文档**

- lease 1.41.0 升 1.42.0（pyproject 与 __init__ 与 CONTRACT 同步）；CONTRACT 修订五十八（stem 闸六格判词表与 --new-stem 与域分派与 intent 先验与空 allow 教学与闸位次顺延申报）
- mcpline 0.7.0 升 0.8.0（两新具与面计数；pyproject 与 __init__ 同步）
- DEC-017 修订五：常设纪律执行位落闸记录（载体自 skill 文档扩至 lease open 闸与 MCP 工具面；本域拒加认领、外部域教学；纪律判定语义零变更）
- AGENTS.md：MCP 节工具面计数与清单同步（十五具升十七具、stdio 十七升十九）

**件五 测试**

- lease 族（fixture 根夹具化：fixture_root 建 fixture 词典包 packs/core 最小五件，terms.json 播 mcpbeta established；既有 open 测试经 established 命中零破坏）：
  - S-1 established 过（stem_check: established_pass）
  - S-2 dead 拒教学零副作用
  - S-3 lazy 拒教学
  - S-4 candidate 拒教学
  - S-5 zero-hit 拒教学，--new-stem 过且回执载 new_coinage_acknowledged
  - S-6 新城域教学不拒
  - S-7 nomenclator 缺席退出码二 fail-closed
  - intent 先验：缺文件与非 JSON 两例报文正标签且先于身份闸
  - 空 allow 回执教学字段在
- mcpline 族：两具单元透传、矩阵两行、面计数断言（stdio 19、HTTP 17）、既有族零回归

## 三、工作清单

- [ ] T-1 lease stem 闸与 intent 先验与空 allow 教学
- [ ] T-2 mcpline 两具投影与面计数
- [ ] T-3 两族测试
- [ ] T-4 SPEC-023 修订一与 CONTRACT 修订五十八与 DEC-017 修订五与 AGENTS.md
- [ ] T-5 版本双升与 README
- [ ] T-6 结果档与认证上链与双仓 settle 与收约对账

## 四、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F-1 | 闸 | S-1..S-7 七态俱有实录，拒面教学语在档 |
| F-2 | 先验 | intent 两错形报文正标签且闸序先于身份 |
| F-3 | 投影 | 两具透传出参与退出码与 CLI 一致，register 零投影 |
| F-4 | 面计数 | stdio 19、HTTP 17 断言绿，既有面零回归 |
| F-5 | 回归 | lease 族与 mcpline 族主树复跑全绿 |
| F-6 | 文档 | 四档修订在档，SPEC/CONTRACT 走管线三步 |

## 五、必读文件

- sih-engine/doc/decision/017-wengu-naming.md（既裁令源）
- sih-tools/nomenclator/src（query/check 出参契约）
- sih-tools/lease/src/lease/core.py（open 闸序现场）
- sih-tools/mcpline/src/mcpline/server.py 与 httpface.py 与 writeface/matrix.py
- sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md

## 六、约束（红线）

1. DES-016 中性化裁定：外部域零哲学代强制，stem 闸新城域只教学
2. register 不投影 MCP 面；nomenclator 工具本体与包零改动（闸是消费方）
3. lease 五验与链闸与 closeguard 与 git hooks 零触碰；既有闸序位次只顺延申报不重排
4. 禁触：sih-visual、中央登记册实测数据、InferServer 与司梦工作区、引擎 Rust 零改动零重编
5. 主树零直写（AGENTS.md 例外：不在 git 仓内，原地改，归档件入 engine 工地材料——agentslim 先例）
6. 每条命令立即取退出码失败即停整链禁管道掩码；先红留痕禁清洗
7. scribe 二进制一律主树 target/debug/scribe

## 七、验收标准

- 两族测试主树复跑全绿；七态闸实录齐；面计数断言绿
- SPEC-023 与 CONTRACT 走化格核阅检词管线（AGENTS.md 与 DEC-017 亦走）
- 认证上链、双仓 settle、reconcile 双零、scribe verify 全文 valid

## 八、风险点

- 面计数变更有连锁断言（matrix 行数、httpface 组成、AGENTS.md 计数、README 表）——漏一处即红，逐处对表
- fixture 词典包与真包分叉风险：闸的包位解析经根锚 root，fixture root 自然隔离；真跑冒烟用真根 --new-stem 形验收
- mcpline 0.8.0 与并行窗版本位：mcpboot 刚落 0.7.0，des016impl 批曾拟 0.6.0→0.7.0 未落码，撞面风险低；开工时实查

## 九、范式偏离声明

无（用户三裁：同意分两拨、既有功能没上 MCP 定性、本域直接拒承 08-28 既裁）。

## 十、关联文件

- DEC-017 修订三/四（纪律出生地）；DES-016（中性化裁定边界）；司梦 M1/M3 报告（问题源）；mcpboot-solo（bootstrap 先例与 mcpline 现状）

## 十一、请求写入（逐路径分行）

sih-tools/lease/src/lease/core.py
sih-tools/lease/src/lease/cli.py
sih-tools/lease/src/lease/__init__.py
sih-tools/lease/pyproject.toml
sih-tools/lease/CONTRACT.md
sih-tools/lease/tests/
（目录级理由：stem 闸测试新件与 fixture 词典包工场件出生地）
sih-tools/mcpline/src/mcpline/server.py
sih-tools/mcpline/src/mcpline/httpface.py
sih-tools/mcpline/src/mcpline/writeface/passthrough.py
sih-tools/mcpline/src/mcpline/writeface/matrix.py
sih-tools/mcpline/src/mcpline/__init__.py
sih-tools/mcpline/pyproject.toml
sih-tools/mcpline/README.md
sih-tools/mcpline/tests/
（目录级理由：两具测试新件与面计数断言更新地）
sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md
sih-engine/doc/decision/017-wengu-naming.md
sih-engine/sih/state/plan/mcpnomgate-solo.md
sih-engine/sih/event/plan/mcpnomgate-solo-results.md
sih-engine/sih/event/plan/mcpnomgate-solo-materials/
（目录级理由：批材料出生地，先红留痕与管线读数入档）
sih-engine/sih/event/trail/2026-09-10.ndjson
sih-tools/scribe/reports/
（目录级理由：ask3 双门与叩问与认证报告出生地）
sih-tools/identity/reports/
（目录级理由：正身件出生地）
AGENTS.md
（工作区根文件不在 git 仓内，原地改归档入 engine 工地——agentslim 先例）
