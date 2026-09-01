# cmdface-solo 结果档（命令面速查固化批，round-1 + round-2 终态）

> 批名：cmdface-solo。日期 2026-09-01。最终会话 4542146955b7c5cd（lease 1.10.0 双仓 engine+tools，round-2 同号重执承 tdfix2 先例；初会话 e797978dac18dd36 因任务包 allow 花括号未展开致 F-1 代码文件 scope 拦截已 close 重开，仿 outslim 批 684f5aa 具体路径先例传展开 --allow）。
> 承接：用户 2026-09-01 令即 C 命令面速查。主会验收 round-1 只建成 lease 根相对解析修（1.10.0）与 BATCH-FACE.md 与 scribe.rs 一笔改动，其余 F-1 欠账打回代偿，本档如实记录两轮并把 F 表按终态判。
> 链路：round-2 意图事件已上链（哈希前八 3f89263c，即轮次 manifest `cmdface-solo-r2` 三锚），ask3repeater status ok、elicit 消解、正身零异常。

## round-1 部分交付（已归并回退后重执）

round-1 段1 settle：

- tools 仓 settle 3e55bbe3，副本归并 4c8259c6：lease 根相对解析修（core.py + 测试，1.10.0）+ BATCH-FACE.md 命令面速查落盘。
- engine 仓 settle 4823896，副本归并 7cf672e：scribe.rs 用法补全 + --help 支持 + 缺参错误信息完善。

主会验收裁定要点（打回欠账清单）：

1. **F-1 五工具用法补全未交付**：round-1 声称用法补全，但 help 与缺参错误在二进制行为上主会不可见，缺必填旗标全列与形态示例，欠账。
2. **引擎 scribe 用法补全二进制行为不可见**：`scribe --help` 与 `append --trail` 缺参仍只报一行缺参，不列全旗标用法，欠账。
3. **lease 版本三源可能未对齐**：round-1 提交只动 core.py 与测试，pyproject、`__init__.py`、CONTRACT 三处版本可能未对齐 1.10.0，逐处核验欠账。
4. **六工具 CONTRACT 修订欠账**：五工具用法补全各入 CONTRACT 修订记录，lease 补 1.10.0 修订。
5. **BATCH-FACE.md 版本行校准欠账**：文件头工具版本列是 round-1 时点读数，outslim 批已升版，逐工具对表实况改正。
6. **调用册补笔欠账**：cmdface round-1 与本整改轮各笔，触及的工具 CALL-LOG 全补。
7. **结果档欠账**：round-1 未写结果档，round-2 落本档按终态判 F 表。

## round-2 补齐清单

| # | 欠账项 | 补齐动作 | 判据 | 状态 |
|---|---|---|---|---|
| 1 | F-1 五工具用法补全 | nomenclator 与 formatter 与 elicit 与 identity 与 meter 各 help 增 epilog 形态示例加必填旗标全列，断言测试 tests/test_usage.py 实跑 help 输出断言关键旗标词在场，禁只测退出码 | 断言测试全绿 | 过 |
| 2 | 引擎 scribe 用法补全核验 | 引擎 `src/bin/scribe.rs` 用法面补全各子命令缺参错误列全旗标即 intent 列 `--record --validation --trail`、append 列 `--report --exit-code --trail`、park 列 `--record --trail`；构建后实跑核验列全旗标 | cargo test 范围内全绿 + 缺参实跑列全旗标 | 过 |
| 3 | lease 版本三源对齐 | pyproject.toml 与 `src/lease/__init__.py` 与 CONTRACT 逐处核验对齐 1.10.0 | 三源皆 1.10.0 | 过 |
| 4 | 六工具 CONTRACT 修订 | 五工具用法补全各入 CONTRACT 修订记录（formatter 入 INCUBATION 修订二、elicit 修订二、identity 修订七、meter 修订三、nomenclator 修订四），lease 补 1.10.0 修订二十 | 修订记录落位 | 过 |
| 5 | BATCH-FACE.md 版本行校准 | 文件头与 --quiet 行逐工具对表实况改正为 lease 1.10.0 / scribe (引擎) / scrutinator 0.1.0 / formatter 0.2.0 / nomenclator 0.2.0 / identity 0.4.0 / meter 0.2.0 / elicit 0.2.0，并修核阅命令拼写错误 two 处（scrutinator 名实校正） | 对表实况一致 | 过 |
| 6 | 调用册补笔 | nomenclator 与 formatter 与 lease 与 elicit 与 identity 与 meter 与 scribe 七册各补 round-1 与 round-2 两笔 | 七册各两笔在册 | 过 |
| 7 | 结果档 | 本档两轮如实记录、F 表按终态判 | 本档落位 | 过 |

## 管线记录（笔在核前、判在书简前）

round-2 产出的文档类批件入链前各走管线三步（化格 → 核阅 → 检词），全程 meter 包裹，findings 亲读。

结果档、任务包、CONTRACT 修订件等依域走管线：`sih-engine/doc` 域内目标评估是否属 des-001 治理域，`state/plan` 与 `event/plan` 为域外 exit-2 如实记不属违规（承 BATCH-FACE 坑位）。

## F 锚定验收（终态）

| F | 判据 | round-1 | round-2 终态 |
|---|---|---|---|
| **F-1** 用法完备 | 各工具 help 或缺参输出含全部必填旗标与至少一条形态示例，断言测试全绿 | 打回（五工具用法补全未交付） | 过（五工具 epilog 形态示例加必填旗标全列，断言测试全绿） |
| **F-2** 根相对修 | lease open 以根相对 repo 路径开成即 worktree 落根下 worktrees 位，绝对路径形回归不破，两测试在册 | 过（1.10.0 根相对修落地） | 过（三源 1.10.0 对齐，开包即根相对 `sih-tools` 成） |
| **F-3** 调用面档 | BATCH-FACE.md 含全链序逐命令 verbatim 与坑位注记，管线过即化格核阅检词零违规 | 过（落盘） | 过（版本行校准，管线零违规） |
| **F-4** 收口 | 认证入链、双仓收约、unrouted 零增、链 valid | 未收口 | 过（意图 3f89263c 在链，双仓 settle，reconcile 零增，链 valid） |

## 链与收口

round-2 intent 在链（3f89263c）；settle 时以本次认证事件哈希为 settle cert 上链；close 归并后双仓件转正入 git 跟踪；reconcile 双仓对基线零新增；scribe verify 链 valid。调用册七册各补两笔随批提交。本批的包档、结果档、材料件、当日链尾均随本批 settle 入版控。

## 越线申报

承任务包约束与主会红线：scrutinator 与引擎件除 scribe.rs 用法面（src/bin/scribe.rs）外零改动；上链前全绿；findings 亲读；禁管道掩退出码；撞锁即报。文本记录见完工申报。

## 重开约说明（round-2 整改）

初会话 e797978dac18dd36 的 allow 采自任务包请求写入段即 `{src,tests,...}` 花括号写法，lease `scope_allows` 只做精确/前缀匹配不展开花括号，致 F-1 代码文件即各工具 `src/*/cli.py` 与 `tests/test_usage.py` 在 lock 与 commit 双环节被判 staged_out_of_scope。承 outslim 批真增量即其第二会话 684f5aa 用 46 条具体路径而非花括号的先例，本批将初会话 close（强拆副本、改动经补丁全量备份 `/tmp/cmdface-r2-tools.patch` 与 `/tmp/cmdface-r2-engine.patch` 加目录副本 `/tmp/cmdface-r2-backup` 双备）后重开约 `4542146955b7c5cd`，传 22 条展开具体 allow 覆盖 F-1 代码文件与 CONTRACT 与 CALL-LOG 与结果档与材料件与链件。intent 记录与正身报告复用已上链 r2 项即 f6d25a27 与 4b465420，未新签意图。worktree 两仓均自补丁恢复逐字节复现 round-2 改动。
