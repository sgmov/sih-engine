# ledgerhyg-solo:台账卫生三件处置批（一裁先行）

> 令源:用户 2026-09-06 令「得一裁」（主会解读:三小件处置方案打包一命题过得一裁,裁过即执行;解读偏差可纠主会即停）
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:leaseup 线收尾卫生批,三件历史遗留一次清账
> 前置:187 基线在役,锁面零在途

## 一、问题陈述 {#problem}

三件台账卫生遗留俱候人裁多日,用户令处置方案打包过得一裁后执行:

- **件一 四陈旧会话销账**:ledgerloss5-solo §六盘点即四个已实质完结而 revoked 行丢失的陈旧活跃会话(名单以 ledgerloss5 结果档与 reconcile orphan 读数为准,批内核实逐个列出),台账现势视其为活跃即长期噪声
- **件二 wenguobs 双写行**:sessions 台账 337 与 339 行 issued 同内容双写(2026-08-31 重开舞步遗留,主会亲核在档),append-only 不改写历史行,处置方向即读数视图去重
- **件三 locks 镜像缺行**:locks.ndjson 镜像历史窗缺 131 行(lockdb SQLite 正典无损判定零影响,closefix 批数据卫生报告在档),处置方向即不补加文档化

## 二、关键设计 {#design}

### 2.1 一裁先行

- 单命题承载三处置:「四陈旧会话销账补 revoked 行、wenguobs 双写读数视图去重零改写、locks 镜像缺行不补文档化,三处置俱符合工程基线四可验证性与 append-only 完整性」
- 单锚归约只锚 baseline_4(predspec2 先例),命题文本批内落 proposition/DES
- facet 合同模式九发,near_threshold 呈用户转主会;stable_clear 过执契后执行

### 2.2 执行三件

- 件一:逐会话核实质完结证据(收约凭据 lease-check 或分支已删工地已拆或 git 归并在案),revoked 行经 ledger-repair 通道补录——VCS 有原笔即逐字恢复,无原笔按凭据重建带 repair 声明
- 件二:读数视图去重即 status 与 reconcile 的会话计数面按(event, session_id, issued_at)三键去重呈现,原始台账行字节零动(改前改后台账哈希对表在档);CONTRACT 注记双写行披露
- 件三:不补,数据卫生报告补 locks 镜像缺行定案段(缺行清单指针加正典无损声明),结果档承载

### 2.3 零行为变更边界

- 件二视图去重属呈现面变更非判定面变更(台账行与判定语义零动);件一件三纯数据卫生零代码——若批内判定件二属判定语义变化即升级单独一裁,如实申报

## 三、工作清单 {#work}

- [ ] T-1 三件证据核实(四会话名单逐个列证、双写行字节对表、镜像缺行清单指针)
- [ ] T-2 一裁命题材料与 facet 合同模式测量,执契终签
- [ ] T-3 件一销账补录(repair 通道,逐笔验证)
- [ ] T-4 件二视图去重实装与夹具(原始台账字节零动对表)
- [ ] T-5 件三文档化(数据卫生报告补段)
- [ ] T-6 全测试族零回归与双仓 settle 加 close(过守门)加 reconcile 加 verify

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 一裁 | 单命题九发 stable_clear 过执契终签在链,near_threshold 呈用户 |
| **F-2** | 销账 | 四会话 revoked 行入账逐笔验证,repair 标记真假如实(逐字或重建) |
| **F-3** | 双写视图 | 去重夹具绿即同键双行计数为一;原始台账改前改后哈希逐字节一致 |
| **F-4** | 镜像文档 | 数据卫生报告定案段在档,零镜像写 |
| **F-5** | 零回归 | 187 基线全绿;台账行格式零变更;判定语义零变化 |

## 五、必读文件 {#read}

- 盘点源头:sih-engine/sih/event/plan/ledgerloss5-solo-results.md §六与 closefix-solo-materials/hygiene/data-hygiene-report.json
- 补录通道:lease ledger-repair 用法(BATCH-FACE 与 CONTRACT 修订三十五与三十七)
- 单锚先例:predspec2-solo 结果档
- 机械链:sih-tools/BATCH-FACE.md(含全部勘误节)

## 六、约束 {#constraints}

1. 一裁红线:near_threshold 呈用户转主会不自行终签;裁不过即停批申报不擅自改处置
2. append-only 铁律:历史行零改写零删除,件二只动呈现面
3. 台账行格式零变更;ledgerwrite 唯一写点零动;账单台账零触碰
4. 四会话名单须逐个实质完结证据在档才补,证据不足者列出候裁不代裁
5. scribe 裸调逐笔 grep 禁 meter 包裹掩败;退出码真值核;verify 一律主树二进制
6. 收约走 closefix 新机械加链证守门;任务包绝对路径;des-001 域外 exit-2 如实记

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-5 全过
- [ ] 双仓 settle + close(过守门) + reconcile + verify(主树二进制)
- [ ] 结果档 sih-engine/sih/event/plan/ledgerhyg-solo-results.md 与 materials/
- [ ] CONTRACT 注记与 CALL-LOG 双笔随批

## 八、风险点 {#risks}

- 四会话中若有证据不足者:列候裁清单只补证据齐者,宁缺勿代裁
- 视图去重键选取:三键去重须覆盖 wenguobs 形即同内容行,不误伤合法同包多会话形(不同 session_id 不去重)

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游:ledgerloss5-solo(件一件三盘点)与 closefix-solo(件三报告)
- 同线:leaseup 线三批收口后卫生批

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(视图去重位)与 CONTRACT.md 与 CALL-LOG.md
- sih-tools/lease/tests/(去重夹具)
- sih-tools/lease/ledger/sessions.ndjson(件一补录随批)
- sih-engine/sih/event/plan/ledgerhyg-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/2026-09-06.ndjson 与后续日链
