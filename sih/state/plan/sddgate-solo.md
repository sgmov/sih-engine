# sddgate-solo 批任务包：SDD/TDD 完备度判据正典与四层落位（文规线复活）

> 线：文规线（regula-line-tools-2026-09-06 孵化登记件）复活 + 四层消费面
> 令源：用户 2026-09-12 三令——「sdd是司衡引擎事前优化 ai（agent、llm）代码生成的重要功能之一，应该作为上线前的重要指标」、「AB走完不走CD了？另外所有的 sdd 、tdd的文档标准都是要走司衡确定性程序落，而不是交给llm自由发挥！」、「多子代理并行跑。」
> 形：solo 批四簇编组，丁簇（正典面）先行典在码前，甲乙丙三簇并行；本批自身即判据第一个受试批

## 一、使命

SDD/TDD 完备度从原则落为可机械校验的上线前判据：判据正典 DEC-024 先行（判定命令形，零 LLM 判词位），四层消费面全落——A 层 DEC-022 第四晋升判据、B 层 lease 两道门（逐批执法）、C 层 critsweep 第六判据（扫描召回）、D 层 gauge 维度（长程度量）；文规检查器实装复活（先红后绿，吃 sdd-v1 五件格式包与 golden 夹具），SDD 文档标准由机械判定承载。

## 二、四簇分工

- 丁簇（主线）：DEC-024 判据正典 + DEC-022 修订（第四晋升判据）+ GOV-002 修订（v2.5 第六判据），T6 三闸。
- 甲簇：文规检查器实装。工作名运行（孵化登记件命名次序节已批），落 sih-tools/incubation/checker/（围堰位，正式立名后迁）；吃 sdd-v1 包对目标文档族逐条谓词判定，包 schema 三面（识别加判定加出处），三态失败定位（missing、violation、broken_ref），三值退出码（零过、一违规、二异常），JSON 报告双版本戳，先红后绿（golden 夹具先绿、畸形体先红），基线向量回填出处面 D-4 锚位（sih-tools/basemgr/vectors/checker-golden/）。
- 乙簇：两道门落 lease close 轴。SDDG 四判据判定命令形实现（典在码前：链笔时序查规格物先于首个实装笔；规格束齐备：settle diff 新增源码头注正典指针扫描；偏差有承载：结果档偏差条目指向正典位；测试同批：diff 含测试件与对表材料在档），适用界条款（不溯往，落地后新开批受试），显式绕行通道（close --bypass-sddgate 事由留痕 bypass 台账），CONTRACT 修订，测试含本批自身为受试批的反身用例。
- 丙簇：critsweep 接 GOV-002 v2.5 第六判据（SDDG 完备度在档，token_scope 与扫描面循既有形）+ gauge 第四维度（完备度趋势读数，复用 lease 台账与链面数据，零新写点）。

## 三、红线

1. 零 LLM 判词位：全部判据判定命令形（DEC-022 先例：每条判据载判定命令或材料指针，禁印象语），校验机械可重放
2. 文档标准走确定性程序：DEC-024 与 GOV-002 修订走 T6 三闸（化格加核阅加检词），SDD 文档形校验归文规检查器机械承载
3. 先红后绿：检查器与两道门的测试先于或同笔于实装，golden 夹具绿加畸形体红的判词在档
4. 典在码前：丁簇正典面落笔先于甲乙丙实装落笔
5. 适用界：两道门不溯往，落地时点后新开批受试；本批 close 为门的反身受试批（门缺陷即批内修复）
6. 单源四表：判据定义只在 DEC-024，A/B/C/D 四层引用不复制

## 四、验收判据（可证伪）

- G-1 检查器：golden 五件包全绿（退出码零），畸形体样例全红（退出码一，报告带三态定位），`--help` 与 JSON 报告形与契约草案六件对表
- G-2 两道门：SDDG 四判定命令各有确定性实现与单元测试（含正反用例），绕行通道落 bypass 台账留痕可查
- G-3 第六判据：critsweep 输出含 GOV2-C6 完备度在档读数，扫描零异常
- G-4 gauge：第四维读数出数且与台账对表一致
- G-5 本批 close 过门（反身受试）且链 verify valid
- G-6 全文档 T6 三闸绿

## 五、写入面（allow 清单）

- sih-engine/doc/decision/024-sdd-completeness-gates.md（DEC-024）
- sih-engine/doc/decision/022-semver-release-v1.md（修订）
- sih-engine/doc/governance/GOV-002-mainline-lock-v1.md（修订 v2.5）
- sih-engine/sih/state/plan/sddgate-solo.md
- sih-engine/sih/event/plan/sddgate-solo/（结果档与材料）
- sih-engine/sih/event/trail/2026-09-12.ndjson（链笔）
- sih-tools/incubation/（检查器围堰位与夹具）
- sih-tools/lease/（两道门）
- sih-tools/critsweep/（第六判据）
- sih-tools/gauge/（第四维）
- sih-tools/identity/reports/ 与 sih-tools/scribe/reports/（批机械链侧写）

## 六、明确不做

- 检查器正式立名与迁移（候立名会话，围堰位运行）
- TDD 验收器与基线管实装（文规线后继批，本批只落检查器与门）
- 语义层检查（manifest out_of_scope 照录）
- 包容器格式统一（pk-061 候裁维持）
