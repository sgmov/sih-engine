# packhyg-solo 结果档

> 批：出货包卫生三件（nomenclator 词表规范形归一、parser 测试包装补齐、BATCH-FACE 协调纪律勘误）
> 会话：23b7ea6c7850a6b7（sess-zcode-2026-09-04-packhyg，租约 uuid 与会话预备号双标识空间照勘误各认各的）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理
> 承接：主会 2026-09-04 carrwire 追认验收两发现加协调误读教训；用户 2026-09-04 裁定「继续推进数学仓的完善，遇到裁决点过得一裁，退人节点」——本批纯机械零裁决点，机械链全绿自行收口，不设「等你令」节（pk-045 参与者）
> 意图哈希：ask3 记录 2d9fc8e832d35861（双门第一门 exit 0 零违规、repeater status ok 三锚），意图事件 fc3878b6（闸三 --session 加 --sessions 双带）
> 开工前置：立即开工（主会排艇令修订版），租约开局时点锁台账零在锁零他会话，claim 一笔 TTL 480 分先行留痕

## 一、完成度表

| 件 | 判定 | 备注 |
|---|---|---|
| F-1 terms.json 规范形 | 完成（实判零 diff 形） | 化格 exit 0 targets_changed 0 加 canonical 30 绿双证；红态来路与预防条款见三节 |
| F-2 parser 测试包装 | 完成 | conftest.py 版控内补件，收集错误解，批前批后同态零回归 |
| F-3 BATCH-FACE 协调纪律勘误 | 完成 | 勘误补遗节两条立档 |
| F-4 全工具线抽查 | 完成 | 四工具读数同态见五节 |
| F-5 写入仅 allow | 完成 | 双仓工地 git status 对表未越 allow 冻结面 |

## 二、F 表

| F | 类别 | 判据 | 结果 | 证据 |
|---|---|---|---|---|
| **F-1 terms.json 规范形** | 工程 | 化格归一内容零改，test_core_pack_shipped_canonical 绿，BATCH-FACE 增预防条款 | 过 | 工地 terms.json 化格 exit 0 零改（targets_changed 0，内容语义零改红线达成）；canonical 测试工地 30 绿；BATCH-FACE 勘误补遗节第一条「词表并集收编后必须跑化格归一」随批入版控。实判偏差如实申报：主会验红时点（carrwire 验收 29 绿 1 红规范化形态）之后，词表红态已经由后继归并通道修复，本批开工实跑主树 30 绿，故本批 terms.json 零 diff，修复动作可溯不在本批 diff 内 |
| **F-2 parser 测试包装** | 工程 | 定位缺角补齐随批入版控，主树 pytest 全绿（存量红同态申报），parser 行为零改 | 过 | 缺角三层定位：①根工作区 sih-tools/pyproject.toml 成员表八件无 parser（carrwire 新立漏挂）致 parser 自立 .venv 而共享 venv 的 pytest 依赖组不达；②pytest 依赖声明无处落版控（uv.lock 全线 gitignore 系工作区惯例）；③原批次绿态经临时依赖通道（--with 形）承载未随批入版控。补件即新立 parser/conftest.py（src 前插 sys.path 修进程内导入加 PYTHONPATH 注入修 run_cli 子进程 `-m parser.cli` 导入面）。pyproject dev 组方案实测与 test_f4_deps 零第三方分布契约测试相抵，既有测试断言零触碰红线下达位让位。工地批后 84 测全跑 82 绿 2 存量红（f2_xref narrow_pack 与 tokencap filtered_stream，语料漂移）与批前基线同名同理由同态零回归；主树复验收口附记回填 |
| **F-3 BATCH-FACE 协调纪律勘误** | 治理 | 勘误一条：回避判定依据为锁台账归属与对方最新写入时间戳，禁以会话存在性推断在途态 | 过 | BATCH-FACE「坑位勘误 2026-09-04 补遗（packhyg-solo 批增两条）」节第二条立档，承 2026-09-04 主会协调误读教训原文 |
| **F-4 全工具线抽查** | 工程 | 本批触达两工具测试全绿外加 formatter 与 meter 抽查绿（carrwire 后主树首查基线） | 过 | nomenclator 主树 30 绿加工地 30 绿；parser 工地 82 绿 2 存量红同态（主树批前基线同名同理由）；formatter 主树 13 绿（carrwire 基线 13 同）；meter 主树 16 绿（carrwire 基线 16 同） |
| **F-5 写入仅 allow** | 治理 | 写入仅请求写入节所列 | 过 | 双仓工地 git status 对表：tools 工地仅 BATCH-FACE.md 与两 CALL-LOG.md 与 parser/conftest.py，engine 工地仅任务包与 materials 与 results；terms.json 零 diff；越线申报见六节 |

## 三、两件实态判定（红绿两时点如实分流）

### 3.1 词表规范形（F-1）

- 主会验红时点：carrwire-solo 管线读数在案「nomenclator 29 passed 1 failed（规范化形态）」，红因即多批并集收编通道追加后未跑规范化。
- 本批开工实跑时点：主树 test_realpack 3 绿加全套件 30 绿，canonical 已规范。来路核：主树 terms.json 自 HEAD（6cf28e63）零修改（git status 对表），其后一改即 leasewire-solo 段1 提交 3716884d「词表四词」，修复动作在其归并通道内完成。
- 判定：本批按红线内容语义零改，化格复验零改即规范形达成，不以绿饰红亦不以红饰绿；预防条款立入 BATCH-FACE 使「并集收编后必须跑化格归一」成为调用面成文纪律，红态复发有机械要件可拦。

### 3.2 parser 测试包装（F-2）

- 收集错误机理：`uv run --project . pytest` 在 parser/ 下因 parser 非工作区成员而自立 .venv，该 venv 无 pytest，uv 回退 PATH 的 pyenv pytest 裸解释器，`from parser import ...` 收集即炸 ModuleNotFoundError。
- 补件选型实录：dev 组方案（pyproject [dependency-groups] dev=pytest）在工地实测引出 test_f4_deps::test_uv_tree_zero_dists 新红（uv tree 出现 pytest 等分布，破 parser 空腹零第三方分布契约），且改既有测试断言触本批红线，故回滚 pyproject 与 uv.lock 至 HEAD 逐字节同（diff 双证），改立 conftest.py 补件，任务包明列三补形（pyproject 打包声明或 conftest 或测试依赖件）中 conftest 形落地。
- 批前基线与批后同态：批前主树经 --with pytest 临时环境实跑 82 绿 2 存量红；批后工地直跑形（`uv run --project . pytest`）82 绿 2 红同名同理由，收集错误解除即本批修复面，两存量红为语料漂移非本批产出（carrwire 读数在案同名同理）。

## 四、管线读数（笔在核前）

- 化格：terms.json exit 0 零改；BATCH-FACE.md exit 0 无需改；两 CALL-LOG exit 0；conftest.py exit 2（general-v1 只盖 md/json/yaml/toml，py 域外如实记不属违规）。
- 核阅：BATCH-FACE.md 与 terms.json 与 conftest.py 三目标均 exit 2 域外（des-001 域只盖 sih-engine/doc 等），如实记不属违规；ask3 双门第一门 exit 0 零违规。
- 检词：三目标 exit 0 零违例。
- 叩问：五轻信号（规范形、并集收编、测试包装、回避判定、协调纪律），领取登记已在册零信号；digest passed covered 5；处置为不登记申报（terms.json 内容零改红线下零词表写入，词债让位后批，见六节申报）。

## 五、F-4 全工具线抽查读数

| 工具 | 批前基线 | 本批复验 | 判定 |
|---|---|---|---|
| nomenclator | 主树 30 绿（开工实跑） | 工地 30 绿 | 同绿 |
| parser | 主树 82 绿 2 存量红（--with 临时环境实跑） | 工地 82 绿 2 红（直跑形） | 同态零回归，收集错误解除 |
| formatter | carrwire 13 绿 | 主树 13 绿 | 同绿 |
| meter | carrwire 16 绿 | 主树 16 绿 | 同绿 |

## 六、越线与误差申报

1. append 锁长持偏差：开工取锁时四共享追加面（当日链、scribe/reports、identity/reports、meter/counts）按 exclusive 同批取形持锁约一刻钟，后依「append 短持即取即放」纪律改短持，放四锁后逐笔短持认证；持锁期零他会话等锁（开局锁台账零在锁），无实害，申报在案。
2. 管道掩退出码误差两笔：parser 批前基线读数与 nomenclator test_realpack 首跑经管道取尾，退出码读数为管尾非被测工具；两笔均诊断读非判据读，判据读（同命令重跑直取退出码）已在案（82 绿 2 红同态、30 绿），真实被测退出码 pytest=1（2 存量红）如实记。
3. 循环内命令替换掩 $? 误差一笔：核阅与检词首轮循环读数 $(basename) 重置 $? 致六读数全 0 失真，即判读废，净取重跑六笔在案（核阅三笔 exit 2 域外、检词三笔 exit 0）。
4. checkcite 首跑 fail 样本：引证面误传全卷 BATCH-FACE.md，两笔数学载体引用号命中系该档既有节引用（双跑同参形条款等）非本批增量（本批勘误两条零数学引用）；引证面收窄至本批产出结果档后重跑，读数见认证清单。首跑 fail 与重跑读数双留档不隐藏。
5. 词债让位申报：叩问五轻信号在 terms.json 内容零改红线下不登记，词债（规范形、并集收编、测试包装、回避判定、协调纪律）转记本节，让位后批词表批处置。
6. 范围外观察不动作申报：根工作区成员表缺 parser（F-2 缺角定位证）不属本批写入面（请求写入面限 sih-tools/parser/），零改动，登记为后批候选输入。

## 七、冲突样本节（pk-045 样本库）

并行批 leasewire-solo 通报在途（lease/locks 接线面，与本批施工面互斥）。实测冲突点与响应：

1. **撞锁竞态（本样本第 1 条）**：本批六 exclusive 加四 append 取锁十路全一次获放，重试计数 0，零 locked_elsewhere；租约开局时点锁台账零在锁他会话，leasewire-solo 已于本批开工前自行收约（其 sih-math 归并 366298b 与词表提交 3716884d 在主树在案），本批未以「会话存在」推断其在途，实查锁台账归属判定，正对新勘误条款的实证形。
2. **trail 收约冲突**：本批认证先落主树活链，settle 前一次性拷链入引擎工地，收约时若遇并行追加按事件数并集超集且 scribe verify 过为唯一放行形；本批收约时点链零并行写，冲突未触发，预案在案。
3. **主树同名未跟踪件让位**：任务包与 materials 两件主树未跟踪副本与工地提交落地路径相同，收约按备份让位归并对表法四步（备份、让位、归并、diff identical），实录见收口附记。

## 八、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| fc3878b6 | 意图笔（scribe intent，闸三双带） | ask3 记录 2d9fc8e8 |
| bf7f034b | 管线报告（2026-09-04-packhyg-solo-pipeline.json） | 三步读数全量 |
| 78cbbf1d | 变更件报告（2026-09-04-packhyg-solo-changed-files.json） | 变更与零改与范围外三清单 |
| （checkcite 认证见收口附记回填） | 书单对表守卫报告 | 结果档引证面 |

认证一律先落主树活链，链 verify 以收口读数为准。

## 九、收口读数

settle 双仓提交号、放锁、close 归并、reconcile、当日链 verify、主树复验（parser 直跑形与 canonical 与四工具抽查）读数，见收口附记（close 后经 bypass 通道回填，pk050sw-solo 收口回填先例同形）。

## 十、队形验证

单线形 solo 零子代理全程成立：本批全部写入由会话 23b7ea6c7850a6b7（sess-zcode-2026-09-04-packhyg）亲写，零 Agent/Task 子代理调用；规范形与包装补件与勘误全由确定性程序承载（化格 targets_changed 0、canonical 断言、pytest 对表、checkcite），链写入经引擎 scribe 闸三（--session 加 --sessions）零直写链文件；零裁决点，机械链全绿自行收口。

## 十一、收口附记（close 后回填，pk050sw-solo 收口回填先例同形）

- settle 双仓：tools 段1 c6af8ad4（base integral-stage-build@6cf28e63，cert a198349f，三验 staged_in_scope 加 cert_on_chain 加 session_active 全过）、engine 段1 5ffb404（base main@30575a6 同 cert）；归并 merge 17440396（tools）与 2c25ca9（engine）。
- 范围闸让位实录：tools 工地首轮 settle 遭 staged_out_of_scope 拒（nomenclator 与 scribe 两 CALL-LOG 未纳 allow，真实退出码 1；首轮读数经管道取尾掩码失真误差并入六节申报），照 facepark/selwire 先例工地回退让位 scoped settle，两笔随收约经 bypass 通道补笔入版控，bypass 台账登记 sha 9d749624。
- close 两段式：首轮 close tools 侧成（worktree 删支净）engine 侧 merge_failed——主树当日链活写未提交加任务包与 materials 未跟踪件双阻（git 拒并详情在 close 台账）；备份让位归并对表法四步响应即备份当日链 145 事件与任务包与 materials 四件、checkout 加 rm 让位、close 重跑自愈（CONTRACT 自愈条款，failed 清零 revoked true）、diff 四件逐字节 identical 全过。
- reconcile 双仓：engine exit 0（unrouted 0 加 cert_missing 0 加 unbypassed 0）；tools exit 1 系 cert_missing 1——sha 526e2be 日期 2026-09-03 entryunique-solo 段2 旧账非本批新增（本批提交零新增 cert_missing，unrouted 0），判据「相比批前零新增」达成如实记。
- 当日链 verify：145 事件 valid，first 05a8a75e（当日首事件不变）尾 a198349f（本批 checkcite 认证即链尾）。
- 主树复验：parser 直跑形 uv run --project . pytest 82 绿 2 存量红同名同理由（收集错误解除，退出码 1 系两存量红如实记）；nomenclator 30 绿；canonical 五件机械复断全绿；conftest.py 主树在案（c6af8ad4 落地）；BATCH-FACE 勘误补遗节与两条主树在案（grep 三读数 1/1/1）。
- 本附记随收约经 bypass 通道入版控（close 通道外提交 --no-verify 加 lease bypass 登记）。
