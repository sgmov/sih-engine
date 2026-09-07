# hookfix-solo 结果档：注入钩子热修补即根判据与永不阻断

> 承接：任务包 hookfix-solo.md 与用户 2026-09-07 报错即 hooks_prompt_block TraceID a0dd17e0。配置层热修先行即绝对路径加 || true 兜底，本批承载代码与契约修复。
> 队形单线形 solo，日期 2026-09-07，会话 sess-zcode-260907-hookfix（session_id 60d3d7c37bc8f913）。

## 意图锚定

- 意图事件：intent_refined event_hash `5ce61ad4...`
- record：sih-tools/scribe/reports/2026-09-07-ask3-hookfix-solo-record.json
- validation：sih-tools/scribe/reports/2026-09-07-ask3-hookfix-solo-validation.json
- 三锚引文程序切片于生成器 make_ask3_hookfix-solo.py。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0 status ok anchor_count 3。
- 叩问：四词出信号，契约内四条处置后 digest passed covered 4。
- 正身：identity verify attest 零异常。
- 实弹首注已验证：用户报错消息顶部附五行注入即母批机制在本会话活的证据。

## 修复读数

- 根因二件：其一模板变量在子根会话解析为 sih-visual/assets 子根致脚本相对路径断；其二 python 缺文件退出码二被钩子释为故意阻断。
- 配置层热修先行：command 改绝对路径加 || true 永不阻断包裹，时序如实记档不倒填。
- 代码修复：root() 改自脚本位逐级上溯首个含账本目录者定根，环境变量降为测试回退位；固定层数 parents 在工地形误指工地根即首修后红一暴露改上溯形，如实记档。
- 契约同步：CONTRACT 修订一登记注入位与根判据两改。

## 验证读数

| 验证 | 实态 |
|---|---|
| F-1 子根会话全活 | 仪表盘子根 cwd 实跑五行无降级即账本泊界链真值，样本入批材料 |
| F-2 双层保零 | 脚本缺场形命令实测退出零不阻断；在场退出零 |
| F-3 双跑与单键 | 同输入双跑 cmp IDENTICAL；严格 JSON 单键即测试过 |
| F-4 契约同步 | 修订一在档与实态一致 |
| 测试 | 八测全绿含子根模拟与缺锚两支与静态扫描 |

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 子根会话全活 | 工程 | 子根环境五行无降级 | 通过 |
| F-2 双层保零 | 工程 | 缺场退出零不阻断 | 通过 |
| F-3 双跑与单键 | 工程 | 逐字节一致与严格单键 | 通过 |
| F-4 契约同步 | 数据治理 | 修订一与实态一致 | 通过 |
| F-5 写入仅 allow | 治理 | 写入仅请求写入节 | 通过 |

## 越线与误差申报

- 根判据首修即固定层数 parents 形在工地测试自暴露误指工地根，二修改上溯形过，红绿档如实记档。
- 测试件空文件支误写即 "" 命中缺场支，改 "\n" 命中首行为空支，如实记档。
- 配置层热修先行于批开立即时序如实记档不倒填。
- 无越线项。其余误差零申报。

## 管线读数

- 化格：任务包与结果档与 CONTRACT 过 packs/general-v1。
- 核阅：des-001 对三件即 tools 与 state 与 event 路皆域外退出码二如实记档。
- 检词：nomenclator packs/core 对三件。

## 认证清单

| 件 | 类型 | 状态 |
|---|---|---|
| ask3 记录与验证件 | 意图记录 | 书简认证 |
| 正身件 | 身份报告 | 书简认证 |
| 修复验证件 | 批验证 | 书简认证 |

## 结算读数

- 双仓 settle：tools 工地提交 576828c1、engine 工地提交 0621bd3，cert 取 970c8299 即链末哈希可证。
- 放锁收约：十路径 unlock 毕、close 一次过、会话 60d3d7c37bc8f913 收约。
- 主树真形态：anchor.py 上溯形根判据与 CONTRACT 修订一与八测随归并在位；工作区 config 热修形在位。
- 链 verify：2026-09-07 当日链 valid，末哈希 970c8299 即本批末笔认证。
- reconcile：双仓 unrouted 零；closeguard 两笔已补 bypass 登记。
- 收约补笔：结算读数回填即本笔，经 --no-verify 加 lease bypass 登记通道入版控，先例同形。
