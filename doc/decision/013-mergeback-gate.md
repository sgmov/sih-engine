# DEC-013 融回门机制

本决策成立引擎侧融回门机制，承接 2026-08-27 用户第一批融回批令、doc/governance/GOV-002-mainline-lock-v1.md 退出标准、sih-tools/COURSE-v2.md 融回门判据、孵化环第四步三查。首件即书简。

## 概览 {#overview}

- 门开判据即贡献度检验判据、达标由检验判据承载不由用户裁量、承 v1.4 数学化，融回评估启动仍走批令留痕与三查::[决策内容](#content)
- 执行三步即引擎开发、双模并存切换、工具侧退役::[决策内容](#content)
- 三查承孵化环升格即验收判据全过、接口契约未变、回迁债已评估::[三查](#checks)
- 名随物走即家位承 DEC-001 映射不重建::[决策内容](#content)
- 此后每件融回复用本机制不再另立::[复用](#reuse)

## 决策内容 {#content}

门开
: 融回门开关归治理程序与用户批令留痕。开门判据即贡献度检验判据：贡献度为承重事件计数测度、承重集由认证引用与闸门改流与材料引用三判据并集承载、v1 均匀权重、可加性恒等式承 PROB-016；达标为三件合同参数合取即 union_threshold 至少 N 件承重与 sustained_min_days 至少连续 D 天每日承重计数大于零与 monotonicity 序列形态承诺即 non_decreasing 与 non_increasing 与 median_stable 与 any 四枚举，由 gauge contrib 只读子命令即 formula gd-2 承载，三参数缺省值以 constclear 登记册行指针随引即 GD_UNION_THRESHOLD_DEFAULT 与 GD_SUSTAINED_MIN_DAYS_DEFAULT 见 sih-math/docs/constclear2b-routing-2026-09-08.md 工程实践三件套态表与 GD_MONOTONICITY_DEFAULT 系账面外常数族候后继清账批收编，证人面即 union_count 与 sustained_run 与 monotonicity_satisfied 承 P3.2.1 构造性可达性；gd-1 二项检验与控制图两腿即 PROB-010 与 PROB-013 为理论史登记不参与新判据面；2026-09-04 用户裁定「融合贡献度应该走数学公理，不走人节点」，达标由检验判据承载不由用户裁量；原体量与时间两重门槛降级为贡献度的观察窗参数。评估材料由 gauge contrib 读数与链上认证面供，融回评估启动仍走批令留痕与融回门三查，不以判据全绿代裁融回动作本身。

第一步引擎开发
: 引擎侧是开发非移植。SDD 即规格先行，落差规格钉死待建面与验收判据后才开始实现。TDD 即失败测试先行，逐判据先红后绿，红转绿留痕于实现批结果档。组件以 Rust 落 src，哈希公式单源承既有 compute_event_hash。

第二步双模并存切换
: T6 管线认证位改指引擎件，工具件转兼容只读即仍可 verify 与 query 供回看，不再作为唯一写入位。并存期以生产 trail 全量复验双跑一致为切换判据。并存不是常态，切换完成即进第三步。

第三步工具侧退役
: 工具契约标注退役与切换日期，调用册补尾行，融回完成档记录三查对表结果。工具代码不删除即沉默参考，失败经验已提炼者不再继承。

名随物走
: 融回件家位承 DEC-001 映射。书简家位即 sih/event/trail 与引擎 event_stream，接事件流不重建。命名不另起即书简之名随承诺回家。

占位承载
: 凡决策文declared占位批或切流或远期融回义务，其承载位必须机械在册即 pk 泊位或后继任务包号两者其一，未承载即该决策不得以收约闭项。义务散文不承载义务，占位蒸发即意图静默降级，judouwire-solo 批判因在案即 DEC-016 第三占位批从未开立致重接债带伤运行。已declared未承载的存量义务随本条生效一次性盘点入泊。

## 三查 {#checks}

验收判据全过
: 融回件的验收判据逐条机械判定，全过才进切换。

接口契约未变
: 工具线契约与引擎规格对表，接口语义不因搬迁而变，变更须先修规格。

回迁债已评估
: 研究档与落差规格所列回迁债逐项处置或显式挂起，不默写。

三查出处为孵化环第四步，本决策将其升格为引擎侧机制，此后每件融回按本节执行，不再依赖孵化 skill 文本。

## 复用 {#reuse}

本机制为可复用规程。下一件融回即租约，再后即管线三件齐议。每件复用即新开 SDD 批与 TDD 批与切换批，不复用旧规格文本，落差按件重钉。

## 决策理由 {#reason}

融回是双向动作即工具侧交权与引擎侧承权，缺机制则每件融回即兴。三查出自孵化环已被围堰期验证，升格入 DEC 使机制随引擎治理域固化。双模并存防切换即断档，退役不删代码防历史失证。

## 版本与固定 {#version}

v1 即 2026-08-27 承用户第一批融回批令成立。修订须走修订记录节逐条留痕。

v1.1 修订一，2026-09-01 随 scrutmerge-switch-solo 批核阅融回三步曲第三步执行记录：核阅融回三步曲前两步已毕即 SPEC-013 规格与引擎侧实装加测试守卫；本步即切换批以同参形条款为切换成立判据执行实录完成：双跑判据 cargo build 1 warning 来自主树已存在 retriever 集成根 dead code 非本批触发红线守住、cargo test --lib 107 passed 含金向量 12 件逐字节断言全过、三目标 SPEC-013 DEC-020 GOV-002 同参形双跑 cmp 零差与退出码一致；核阅位换旗即根 AGENTS.md 三处核阅位与 BATCH-FACE.md 核阅腿改指引擎件；工具件退役即 CONTRACT 退役标注与 CALL-LOG 尾行承先例；完成档与 GOV-003 v1.6 与 SPEC-013 修订五与本节修订一齐随本批落档。融回门三查对表：第一查验收判据五条全过、SPEC-013 修订四与五条款在档；第二查接口契约未变、CONTRACT.md 退役登记不删接口仅加注；第三查回迁债已评估、AGENTS.md 指针改写随连带改写五项一并结算归位承 PRO-005 全态节。

v1.2 修订二，2026-09-02 随 deyimerge-switch-solo 批得一融回三步曲第三步执行记录：得一融回三步曲前两步已毕即 SPEC-014 规格与引擎侧实装加测试守卫即 deyimerge-tdd-solo 批 src/attractor/ 十三件与六子命令二进制与金向量 64 件先红后绿；本步即切换批执行实录完成：双跑判据活体五场景 check cmp 全 IDENTICAL 且退出码对齐与 score 活体双跑 IDENTICAL，证据件在 deyimerge-tdd-solo-materials，pk-036 金向量重录以围堰件为基准按现行 GOV-003 真实内容刷期望输出断言逻辑零改 cargo test golden_des001_gov003 转绿；判定位换旗即 BATCH-FACE 执契与 facet 测量命令段改指引擎件 target/debug/attractor、DEC-020 无调用位表述零碰；工具件退役即 facet 与 tally 两 CONTRACT 退役标注与两 CALL-LOG 尾行承先例，facet CLI 采样双模并存保留即 GOV-002 判据四；完成档与 GOV-003 v1.7 与 SPEC-014 修订一与本节修订二齐随本批落档。融回门三查对表：第一查 SPEC-014 验收判据 A1 至 A5 全过承 deyimerge-tdd-solo 结果档；第二查接口契约未变、两 CONTRACT 退役登记不删接口仅加注、围堰件与引擎件输出逐字节而浮点文本形按豁免条款引用；第三查回迁债已评估、serde_yaml 依赖已声明与腿切分二十三行零偏差与判据 v3 闸未融回如实列明属围堰采样腿边界。

v1.3 修订三，2026-09-02 随 autoflow2-solo 批谓词融回三步曲前两步执行记录：谓词融回三步曲前两步已毕即 SPEC-015 规格批落档走管线域内零违规与 TDD 实装批全绿即 src/attractor/route.rs 谓词机与两包纯数据逐字节随迁与 route 子命令与截流谓词装配位与金向量八场景围堰实测冻结；凭据即 predsplit-a1/a2/a3 三枚 stable_clear 终签在链，件一过裁直执行。双跑判据执行实录：活体三场景 net-core 与 net-parking 与 attribution-core cmp 全 IDENTICAL 且退出码对齐，证据入 autoflow2-solo-materials/route-double-run-cmp.log；判据五过程件归零同批执行即 f-anchors-x11-t6d.md 归档入 sih/event/plan 原位删除。融回门三查对表：第一查 SPEC-015 验收判据 A1 至 A6 全过承 tests/attractor_route.rs 十件与 T4 模块单测与 tdd-red-green.log；第二查接口契约未变、围堰 selector CONTRACT 零改动仅引擎实装对表、解析失败类运行时报文不逐字节复刻已显式申报；第三查回迁债已评估、依赖零新增与归位映射行在案与域逐件判定显式不做、切换批管线调用面换旗归后批即第三步未启，本修订只记前两步，GOV-003 v1.8 随批落档只表述状态不宣称结算。

v1.4 修订四，2026-09-04 随 contribmath-solo 批融回贡献度判据数学化：门开条款达标判据由用户裁量改检验判据，令源即用户 2026-09-04 裁定原话「融合贡献度应该走数学公理，不走人节点」入修订记录作令源；判据凭据承 m-contrib-load-1 stable_clear 终签 f9520776 在链即 crosscheck-m-contrib-load-1、九发全合变卦 0%、谨慎信号 0/9、R1 至 R7 全过、verify identical，承重三判据即认证引用与闸门改流与材料引用钉为 SPEC-017 条款，贡献度测度即窗内承重事件计数 v1 均匀权重、可加性恒等式承 PROB-016，达标即承重率二项检验对空转基线加控制图连续窗判读两腿合取即 PROB-010 与 PROB-013 在役、facetmath 检验族同构、alpha=0.05 候值随 pk-049 待裁登记，实装载体 gauge contrib 只读子命令 formula gd-1 与推导档 sih-math/docs/contribmath-derivation-2026-09-04.md 随批落位；体量与时间两重门槛降级为贡献度的观察窗参数，窗长入环境参数登记面。判变申报：判变不溯往，本修订前已融回四件即书简、核阅、得一、谓词机不改写不重评，新判据只施于未来门评。孵化环第四步措辞同步随本批。

v1.5 修订五，2026-09-09 随 fusadopt-solo 批门开判据文本随 gd-2 收口：锚即 fusiongate 处置档与主窗复算对表，凭据即确定性核对五查全过即 C1 本档全文零 gd-2 零三件合同、C2 本档两处正典均载两腿检验与控制图与 formula gd-1、C3 gauge CONTRACT 十九行载 formula_version gd-2 与四十四行载两腿整族退场、C4 gauge cli.py GD_FORMULA 即 gd-2 与 _contrib_test 与 _contrib_chart 零命中、C5 contrib 金向量两件俱 formula gd-2，即 v1.4 门开判据文本在 gauge 0.8.0 现行 contrib 中已无对应载体，工具 0.8.0 起为 gd-2 构造式三件合同；判据凭据承 m-fusadopt-b1 stable_clear 终签 44fa67d1 在链即 crosscheck-m-fusadopt-b1、九发全合变卦 0%、谨慎信号 0/9、R1 至 R7 全过、verify identical。改笔即门开条款达标句改三件合同参数合取形即 union_threshold 与 sustained_min_days 与 monotonicity 四枚举、formula gd-1 改 gd-2、三参数缺省值以 constclear 登记册行指针随引即 GD_UNION_THRESHOLD_DEFAULT 与 GD_SUSTAINED_MIN_DAYS_DEFAULT 见 constclear2b-routing 工程实践三件套态表与 GD_MONOTONICITY_DEFAULT 系账面外常数族候后继清账批收编、证人面即 union_count 与 sustained_run 与 monotonicity_satisfied 承 P3.2.1 构造性可达性、PROB-010 与 PROB-013 转理论史登记；SPEC-017 关联句随动同批；gauge 载体任何实现零改动即本修订只动正典文本。判变申报：判变不溯往，v1.4 判据下已启动的门评不改写不重评，新判据文本只施于未来门评；修订为融回门干净裁决的前置件不阻主线结算。

v1.6 修订六，2026-09-11 随 judouwire-solo 批增占位承载条款：令源即 tree-sitter 重接债链上判因——DEC-016 三占位之崩溃语料重接测试批从未开立亦未入泊，实现批 F-1 借名未接实，义务停留散文致意图静默降级，真语料 58 件 rust 旧载体崩 21 件带伤运行至本批切流；改笔即决策内容节增占位承载定义项，已declared未承载存量义务一次性盘点入泊；DEC-016 三占位销账随同批修订落档。
