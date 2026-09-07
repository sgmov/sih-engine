# hookfix-solo：注入钩子热修补即根判据与永不阻断

> 治理任务包（实装修复类，单线形 solo，DEC-018）
> 承接：用户 2026-09-07 报错即 hooks_prompt_block 因子根会话下 ${ZCODE_PROJECT_DIR} 解析为 sih-visual/assets 子根致脚本路径断，python 缺文件退出码二被释为故意阻断；配置层热修已先行即绝对路径加 || true 兜底，本批承载代码与契约修复
> 日期：2026-09-07

## 一、问题陈述 {#problem}

- 根判据缺陷：anchor.py 的 root() 信环境变量优先，子目录会话即视图精修窗口下环境变量指向子根，账本泊界链三行全部降级
- 阻断缺陷：命令形经模板变量拼相对路径，脚本缺场时 python 退出码二即钩子阻断语义，违背退出恒零承诺即保零位须在脚本之外的 shell 层
- 契约滞后：CONTRACT 注入位节仍载模板变量形，与热修后实态不符

## 二、关键设计 {#design}

### 2.1 根判据反转

root() 改脚本定位优先即脚本物理所在仓根经账本存在性验证后即用，环境变量降为回退即测试注入位。工作区根是工具家位的属性不是会话 cwd 的属性。

### 2.2 双层保零

shell 层 || true 兜底任何脚本级失败含缺场，脚本层退出码恒零，两层叠加即注入链路在命令形、脚本、依赖三层任何断裂下都不阻断会话。

### 2.3 契约修订

CONTRACT 修订一登记注入位改绝对路径加永不阻断包裹与根判据反转，热修先行如实记档。

## 三、工作清单 {#work}

### Cluster 1：工地写

- [ ] anchor.py 根判据反转与 docstring 补记
- [ ] tests 重构即单元式直调函数加子进程式全链各半
- [ ] CONTRACT.md 修订一

### Cluster 2：主线串行验证

- [ ] 测试全绿含子根模拟路径
- [ ] 双跑一致、严格单键、退出零
- [ ] 管线三步、认证、双仓 settle、放锁收约、reconcile 与链 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 子根会话全活 | 工程 | 模拟子根即环境变量指向子目录时，五行无降级即账本泊界链三行回算真值 |
| **F-2** 双层保零 | 工程 | 脚本缺场形命令实测退出零不阻断；脚本在场退出零 |
| **F-3** 双跑与单键 | 工程 | 同输入双跑逐字节一致，输出严格 JSON 单键 |
| **F-4** 契约同步 | 数据治理 | CONTRACT 修订一在档且与实态一致即绝对路径与根判据反转 |
| **F-5** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |

## 五、必读文件 {#read}

- 报错原文即用户 2026-09-07 转述与 TraceID a0dd17e0
- 在役件：`sih-tools/attnanchor/anchor.py` 与 `CONTRACT.md`
- 钩子规范：zcode-guide 诊断 skill 即退出码二为阻断语义

## 六、约束 {#constraints}

1. 零 LLM 零网络纪律承 attnanchor CONTRACT 不变
2. 配置层热修先行如实记档不倒填
3. 主树零直写即待提交件经工地 settle 通道，链文件只经引擎 scribe 写位
4. 守卫在位禁 plain git commit，收约补笔走 bypass 登记
5. 在盘遗留无主件不豁免不代清

## 七、验收标准 {#acceptance}

本任务包验收 = 四项：

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle 提交号在档，收约后本批零活跃锁
- [ ] 链 verify valid，reconcile 零新增
- [ ] 结果档 hookfix-solo-results.md 落 event/plan

## 八、风险点 {#risks}

- 测试重构须保双跑与单键断言不弱化
- 环境变量回退位保留给测试即生产路径不再依赖它，误用须测试拦

## 十、关联文件 {#related}

- 工具：`sih-engine/target/debug/scribe`
- 关联批：attnanchor-solo 即本件母批

## 十一、请求写入 {#requested-writes}

- `sih-tools/attnanchor/anchor.py`
- `sih-tools/attnanchor/tests/`
- `sih-tools/attnanchor/CONTRACT.md`
- `.zcode/config.json`
- `sih-engine/sih/state/plan/hookfix-solo.md`
- `sih-engine/sih/event/plan/hookfix-solo-results.md`
- `sih-engine/sih/event/plan/hookfix-solo-materials/`
- `sih-engine/sih/event/trail/2026-09-07.ndjson`
- `sih-tools/scribe/reports/`
- `sih-tools/identity/reports/`
- worktrees 双仓 hookfix-solo 工地
