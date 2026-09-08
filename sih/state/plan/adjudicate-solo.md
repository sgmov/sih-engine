# adjudicate-solo：四席裁决得一测量批（A1 至 A4）

> 治理任务包（测量类＋执行类合一，单线形 solo，DEC-018）
> 承接：用户 2026-09-08 FORK-1 令「执行 adjudicate-solo 批（四席裁决得一测量批）」承「所有裁决都走得一」；命题文本系 agent 材料同权送测不预设结论；先例形 pk063split-solo 与 sitruling-solo（两档结果档坑位注记必读已读：席位基线配 identity.hash 非 core_hash、tally 从工地根 cwd 装配、verify 先落报告件、正身件在场直至 unlock 毕）
> 会话：sess-zcode-260908-fork1-adjudicate；日期：2026-09-08

## 一、问题陈述 {#problem}

- 四席待裁事项散于泊界与会话：pk-063 出泊（双通道裁决执行）、改判三件收编、constclear2 后继拆分、confpreempt 停滞会话处置；用户令全走得一即命题化测量后执行
- 测量形：emit-contract 零 LLM、九发采样、判据 v3、执契 tally 终签；任一席位刀锋即如实转人诊断不硬凑

## 二、四命题 {#propositions}

- A1 pk-063 出泊：promoted，登记面裁决按双通道执行即能数学化的面挂数学载体、纯约定面走登记面且不得伪装数学，双通道判定标准随出泊落档（锚 P1 67353ed9、P2 f4b8f93f）
- A2 改判三件：r1/r2/r3 收编环境参数类，且逐件缺省值仍须过两态制即数学模型或工程实践三件套，类目收编不豁免取值依据（账面 sih-math/docs/constclear2-routing-2026-09-08.md §4）
- A3 constclear2 后继拆分：批二＝登记面落位＋RECLASS 收编＋新账扫描（GC_*/GD_UNION 族 rev3 未覆盖，账面 L77）；批三＝代码注记候选；pk-049 联动件随批三呈报
- A4 confpreempt 处置：停滞会话（pid_dead、持锁零、09-07 后无活）立即接管清理，置信度线择期重开，进度以 09-07 意图笔 05762749 为续作锚

## 三、关键设计 {#design}

- 四命题各自独立 gid 测量即 m-adjudicate-a1 至 a4，各九发合同模式；回填同席即席作答，谱系披露双声明入 topic frontmatter；seat-baseline 按 sitruling 正形即账本尾行 props 配当日正身 identity.hash 实值
- 执行面随测量结果：A1 过即 pk-063-exit.json 材料与 parking_exited 上链（promoted，ruling 照录令源，context 载 P1/P2 双锚与进泊笔 47101ff6）；A2 过即落据锚入结果档（账面零改写归批二）；A3 过即拆分定义入结果档为后继批输入；A4 过即接管清理读数与上链留痕（takeover 已先行执行见误差申报）
- 测量工件落 tools 工地 facet/contracts/adjudicate-260908/ 与 DES 单元格；执行材料落 engine 工地

## 四、工作清单 {#work}

- [ ] 四 topic 与四合同与 4×9 发回填与计分与装配与执契终签
- [ ] A1 执行：pk-063 出泊链笔与材料
- [ ] A4 执行：接管读数上链留痕
- [ ] 任务包与结果档管线三步与 checkcite 与认证与双仓收约对表

## 五、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** 四席测量 | 采样道 | 四 gid 各九发 stable_clear 且执契裁决通过四笔 crosscheck 落链；任一 boundary 或 near_threshold 即该席冻结呈人零硬凑 |
| **F-2** A1 执行 | 数据治理 | pk-063 出泊 promoted 上链，ruling 照录令源，context 载 P1/P2 锚与进泊笔，材料按 pk-044-exit 形落引擎线 |
| **F-3** A4 执行 | 治理 | takeover 读数（window_state pid_dead、released_locks 空、窗口清）上链留痕在档 |
| **F-4** 写入仅 allow | 治理 | 写入仅请求写入节所列路径 |
| **F-5** 链面全绿 | 治理 | 双仓 settle 提交号在档，close 零失败，verify valid，reconcile 零新增 |

## 六、必读文件 {#read}

- sih-engine/sih/event/plan/pk063split-solo-results.md 与 sitruling-solo-results.md（坑位注记四条）
- sih-math/docs/constclear2-routing-2026-09-08.md（§4 与 §5 与 L77 只读）
- sih-engine/sih/state/parking/materials/pk-044-exit.json（出泊材料形先例）

## 七、约束 {#constraints}

1. 命题同权送测不预设结论；任一席位刀锋即如实转人诊断不硬凑；回填逐发真实作答零橡皮章
2. 主树零直写；链文件只经引擎 scribe；禁管道掩退出码；误差红证如实记档
3. A2 账面（sih-math）零改写，落据锚归批二；A4 引擎分支未归并提交零触碰即工地保留为续作现场
4. 守卫在位禁 plain commit；补笔走 bypass 登记；正身件在场直至 unlock 毕

## 八、验收标准 {#acceptance}

F-1 至 F-5 全过；四 gid 重放锚在档；结果档 adjudicate-solo-results.md 落 event/plan。

## 九、风险点 {#risks}

- A4 takeover 先于测量执行（时序倒置）——topic 谱系披露声明，测量判 violate 即误差升级呈人
- 装配路径形三态（facet 相对与工地根相对与绝对）历史三病——按 sitruling 正形工地根 cwd 一次到位，异常即红证在档再修

## 十、请求写入 {#requested-writes}

- sih-engine/sih/state/plan/adjudicate-solo.md
- sih-engine/sih/state/parking/materials/pk-063-exit.json
- sih-engine/sih/event/plan/adjudicate-solo-results.md
- sih-engine/sih/event/plan/adjudicate-solo-materials/
- sih-engine/sih/event/trail/2026-09-08.ndjson
- sih-tools/facet/contracts/adjudicate-260908/
- sih-tools/proposition/DES/m-adjudicate-a1/
- sih-tools/proposition/DES/m-adjudicate-a2/
- sih-tools/proposition/DES/m-adjudicate-a3/
- sih-tools/proposition/DES/m-adjudicate-a4/
- sih-tools/scribe/reports/
- sih-tools/identity/reports/
