# adjudicate-solo 结果档：四席裁决得一测量批（A1 至 A4）

> 承接：任务包 adjudicate-solo.md 与用户 2026-09-08 FORK-1 令承「所有裁决都走得一」；命题同权送测不预设结论。
> 队形单线形 solo，日期 2026-09-08，会话 sess-zcode-260908-fork1-adjudicate（session_id 7487fb4b81d8aac9）。

## 意图锚定

- 意图事件：intent_refined event_hash 前 8 `5e847e45`
- record 与 validation：sih-tools/scribe/reports/2026-09-08-ask3-adjudicate-solo-record.json 与同目录 validation（status ok anchor_count 3）
- 三锚引文程序切片（07-on-assay.md L55、08-on-settle.md L110、01-ontology-of-names.md L18）于生成器 make_ask3_adjudicate-solo.py，禁手打承契约。

## 前置读数

- 三问双门：核阅 ask3 包 exit 0；ask3repeater exit 0。
- 叩问：三轻信号即四席裁决、双通道、续作锚；digest passed covered 3。
- 正身：anomalies 0（identity.hash e38080df／core e1c9a7fa）。
- 判据扫（启动节律）：C1/C3/C5 达成、C2 在飞、C4 转在飞（constclear2 与 pk-053 出泊后由沉底转在飞）、五判据零沉底，degraded 假。
- watch 对表：exit 1 已知 callloghyg 候清项呈报不代清。
- 锁面：12 锁全持（lock 子命令对已持锁重入报 duplicate 且退出码二，语义为重复非失败，以 lease status 持锁现势为准如实申报）。

## 四席测量读数（F-1）

| gid | 命题摘要 | 采样 | 规约引用 | 闸门 | 终签链笔 |
|---|---|---|---|---|---|
| m-adjudicate-a1 | pk-063 出泊双通道执行 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 | stable_clear | `4073c5fa` |
| m-adjudicate-a2 | 改判三件收编环境参数类不豁免取值依据 | 9/9 合变卦 0% 谨慎 0/9 | baseline_4 主 baseline_5 次 | stable_clear | `15f9a0ad` |
| m-adjudicate-a3 | constclear2 批二批三拆分 | 9/9 合变卦 0% 谨慎 0/9 | baseline_1 与 4 与 5 | stable_clear | `1f03de72` |
| m-adjudicate-a4 | confpreempt 停滞会话接管清理择期重开 | 9/9 合变卦 0% 谨慎 0/9 | baseline_1 与 4 与 5 | stable_clear | `f4165e2b` |

- 四席俱清晰稳定零刀锋，执契 check 十二项全过 disposition 裁决通过、verify identical、sign 四笔 crosscheck_completed 落链；重放锚即四 DES 单元格 signcheck 件。
- 装配路径形老病一笔：计分材料 facet 相对形与装配工地根解析失配首跑四红，按 reroute 先例材料形修复（facet/ 前缀）后四绿，判定内容零改动，红证即首跑报文在批材料。

## 执行面

- **A1 执行**：pk-063 出泊链笔 `472ec2b4`（parking_exited promoted，ruling 照录令源，context 载 P1 双锚 67353ed9 与 P2 双锚 f4b8f93f 与进泊笔 47101ff6 与本批 A1 终签 4073c5fa）；材料 pk-063-exit.json 按 pk-044-exit 形落引擎线工地随批提交。
- **A2 落据**：测量终签 15f9a0ad 即改判三件收编的落据锚；账面 §4 零改写（本批范畴排除），落据注记归 constclear2 批二登记面落位承载。
- **A3 落据**：测量终签 1f03de72 即拆分定义落据；批二＝登记面落位＋RECLASS 收编＋新账扫描（GC_*/GD_UNION 族账面 L77 未覆盖）、批三＝代码注记候选＋pk-049 联动件呈报，后继批作者以此为开工输入。
- **A4 执行**：takeover 读数（window_state pid_dead、released_locks 空、窗口清、会话 09326a760119e4ed）上链留痕经本件认证归批材料 takeover-confpreempt-readout.json；进度锚 09-07 意图笔 05762749；引擎分支 msh/confpreempt-solo 未归并提交保留工地为续作现场零触碰；sessions 账本 issued 行仍在册（takeover 清窗口不清账本行，重开时按 open 闸序自然处置）如实记档。

## 管线读数

随收约回填。

## 认证清单

随收约回填。

## F 表

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 四席测量 | 采样道 | 四 gid 九发 stable_clear 执契通过四笔 crosscheck 落链，零刀锋零硬凑 | 通过（4073c5fa 与 15f9a0ad 与 1f03de72 与 f4165e2b） |
| F-2 A1 执行 | 数据治理 | pk-063 出泊 promoted 上链，ruling 照录，context 载双锚与进泊笔，材料按先例形 | 通过（472ec2b4，材料随批提交） |
| F-3 A4 执行 | 治理 | takeover 读数上链留痕在档 | 通过（读数件随批认证留痕） |
| F-4 写入仅 allow | 治理 | 写入仅请求写入节所列路径 | 通过（写入面即任务包、出泊材料、结果档与批材料、facet 合同目录、四 DES 单元格、链文件、报告目录） |
| F-5 链面全绿 | 治理 | 双仓 settle、close 零失败、verify valid、reconcile 零新增 | 待收约回填 |

## 越线与误差申报

- **A4 时序倒置**：takeover 执行先于 A4 测量（前置核实与执行同轮完成），topic A4 谱系附加披露在案；测量判 comply 后处置成立，时序瑕疵如实申报不因结果合意免报。
- **装配路径形首跑四红**：材料形修复（facet/ 前缀）后四绿，判定内容零改动，红证（首跑报文）在批材料。
- **lock 子命令 duplicate 语义**：对已持锁重入报 duplicate 且退出码二，与失败二态混淆，持锁现势以 lease status 为准，语义澄清申报候 CONTRACT 面。
- 其余误差零申报。

## 结算读数

待收约回填。
