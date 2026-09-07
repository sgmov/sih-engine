# facefit-solo:锁面瘦身与置信度联动预备批

> 令源:用户 2026-09-07 裁定「要修租约，置信度交付了马上可以联动了」——修在联动前,置信度数学模型（pk-073）交付即阶段二可咬合
> 范式:T6 单线 solo,委外代理亲写零子代理(用户亲转发委外)
> 定位:lease 域第四轮治理批,锁面结构修正加联动预备加 pk-075 收口
> 前置:204 基线,lease 1.31.0,锁面时况开工前实查

## 一、问题陈述 {#problem}

- **命名空间面父目录独占即排队元凶**:要出生新文件的命名空间面（proposition/DES 与 facet/contracts 与 event/plan 批材料目录）本质是各批建各批的子目录互不冲突,现却锁整个父目录——每个要过一裁的批都在 DES 排队,长批四小时后批堵四小时（当日实录）;当年设计即新文件命名空间走子路径保留不走父锁,leaseup 批只上船一半
- **账单事件缺正身锚**:lockface-bills 事件只载 session_id 无 identity_hash,置信度阶段二按正身聚合即无锚可挂,联动断点
- **pk-075 挂泊中**:chain_gate_check 宣称与实装漂移（契约修订四十三宣称收约前链证守门,实装零调用点,主会亲核在案）,出泊二途候裁,主会建议接线
- **任务包模板缺版本位**:declguard 收桌重开根因,存量小件
- **请求写入肥面无立策**:防御性全家桶申报（十五至二十一路）为惯例,账单计价已罚但文规无收窄条款

## 二、关键设计 {#design}

### 2.1 命名空间面子路径保留锁

- 冻结常量 NAMESPACE_FACES 起步三面:sih-tools/proposition/DES、sih-tools/facet/contracts、sih-engine/sih/event/plan
- 落锁规则:声明面落 NAMESPACE_FACES 内的目录时不锁父目录,锁子路径保留位即 <目录>/<批名>/ 缺省派生;显式形 open --namespace <完整子路径> 可重复传,缺省与显式俱入会话档 allow 面
- 守卫与预检:路径包含判定复用即子路径粒度对表——两会话各保留不同子路径零交即并行,同子路径撞即拒;提交面守卫按会话 allow 面含子路径判越界,父目录不再作越界判据
- 真名形兼容:实际写入子目名录与保留位不符（如 DES 用 m-gid 名）即以 --namespace 显式保留实际子路径,批内实测自证

### 2.2 账单事件挂正身（联动预备）

- lockface-bills 四类事件增 identity_hash 键,取会话台账 issued 行 identity.identity_hash,缺席记 null 如实
- lockdb lock_bill 表同增列,双写一致夹具;账单台账非冻结遗留面增键属增量非改写,CONTRACT 注记
- 阶段二置信度台账按正身聚合即有锚,session_id 保留双锚并存

### 2.3 pk-075 接线（承主会建议,用户可改判修约）

- close_session 前置实调 chain_gate_check,核本会话意图笔与认证笔在链（工作区根发现位）,缺即拒收约报文指明缺笔类
- 无链证会话 close 拒夹具红转绿;pk-075 出泊随批

### 2.4 模板与立策

- 任务包模板增版本位必填（治 declguard 收桌重开根因）
- CONTRACT 增请求写入收窄条款:已存在文件点名到文件、目录级须理由行、命名空间面走子路径保留;主会立策同步即主会所出任务包即日起行收窄形

### 2.5 判定语义

- 锁面结构变更（父目录锁改子路径保留）与账单增键与守门接线俱属判定行为域,一裁承前裁族材料单锚 baseline_4 共裁

## 三、工作清单 {#work}

- [ ] T-1 NAMESPACE_FACES 冻结与子路径保留锁实装（缺省派生加 --namespace 显式）
- [ ] T-2 预检与提交守卫子路径粒度对表
- [ ] T-3 账单四类事件与 lock_bill 增 identity_hash 双写一致
- [ ] T-4 pk-075 接线 chain_gate_check 实调用加无链证拒夹具
- [ ] T-5 模板版本位与 CONTRACT 收窄条款
- [ ] T-6 TDD 先红后绿五族加判定一裁加全测零回归加活体（本批 DES 与材料面自走子路径保留,与在途批并行自证）

## 四、可证伪条件（跑前立文） {#falsifiable}

| F 锚定 | 类别 | 判据 |
|---|---|---|
| **F-1** | 子路径保留 | 两会话并行保留不同子路径双绿零排队夹具;同子路径撞拒;父目录不再入锁面（本批活体锁单照录） |
| **F-2** | 账单挂正身 | 四类事件载 identity_hash（缺席 null 如实）,ndjson 与 lock_bill 双跑一致 |
| **F-3** | 守门接线 | chain_gate_check 有调用点（grep 实证）,无链证会话 close 拒,pk-075 出泊材料随批 |
| **F-4** | 模板条款 | 模板版本位在档;收窄条款入 CONTRACT |
| **F-5** | 一裁 | 共裁 stable_clear 过执契,near_threshold 呈用户 |
| **F-6** | 零回归 | 204 基线全绿;openhyg 与 closefix 与 declguard 与 chainstamp 交付零动;sessions 与 locks 台账行格式零变更 |

## 五、必读文件 {#read}

- 现行锁面与预检:core.py 与 lockdb.py 与 cli.py 的 lock 与 precheck 与 close 路径
- pk-075 泊材料:sih-tools/parking/materials/pk-075.json
- 账单现状:lease/ledger/lockface-bills.ndjson 与 lock_bill 表
- 先例:leaseup T-5 追加形正典与用户 2026-09-06 锁文件夹问答裁定（改啥锁啥与未出生件子路径位）
- 机械链:sih-tools/BATCH-FACE.md（含全部勘误节）

## 六、约束 {#constraints}

1. 一裁红线 near_threshold 呈用户;pk-075 若用户改判修约即 T-4 换修约形不硬接线
2. sessions 与 locks 与 bypass 台账行格式零变更;账单面增键入 CONTRACT 注记
3. SCOPE_SHARED_SURFACE 追加形七路面零动;命名空间面与追加面与文件面三分类清晰互不侵蚀
4. NAMESPACE_FACES 冻结常量起步三面宁窄勿宽,扩面走 CONTRACT 修订
5. scribe 裸调逐笔 grep;退出码真值核;verify 主树二进制;收约走新机械加守门;任务包绝对路径;des-001 域外如实记
6. 在途批避让排队;本包自身即收窄示范（请求写入节本形）

## 七、验收标准 {#acceptance}

- [ ] F-1 至 F-6 全过
- [ ] 双仓 settle + close(过守门) + reconcile + verify
- [ ] 结果档 sih-engine/sih/event/plan/facefit-solo-results.md 与 materials/
- [ ] CONTRACT 修订与模板件与 CALL-LOG 双笔与 pk-075 出泊材料随批

## 八、风险点 {#risks}

- 实际子目名录与派生保留位不符（gid 名形）:--namespace 显式通道兜底,批内实测自证并注记惯例
- 存量会话持父目录锁与新制并存窗:预检对旧形照旧判,新制只对新开约生效,零迁移
- 守门接线与差集闸顺序:守门挂差集闸前（链证先于声明对提交）,顺序入 CONTRACT

## 九、范式偏离声明 {#deviation}

委外单线 solo 零子代理;保留 T6-D 命名约定、F 锚定、得一裁红线、双仓同步。

## 十、关联文件 {#related}

- 上游裁定:用户 2026-09-07「要修租约,置信度交付了马上可以联动」与 2026-09-06 锁面问答
- 泊位:pk-075（本批承载出泊）与 pk-073（联动对象,数学模型在途）
- 同线前批:leaseup 至 declguard 全族

## 十一、请求写入 {#requested-writes}

- sih-tools/lease/src/lease/(core.py、cli.py、lockdb.py)与 CONTRACT.md 与 CALL-LOG.md 与任务包模板件
- sih-tools/lease/tests/(五族夹具)
- sih-tools/lease/ledger/lockface-bills.ndjson 与 lockdb lock_bill（增键形随批）
- sih-tools/parking/materials/pk-075-exit.json（出泊材料）与 sih-tools/PARKING-v1.md（投影行）
- sih-engine/sih/event/plan/facefit-solo-results.md 与 materials/ 与 sih-tools/scribe/CALL-LOG.md
- sih-engine/sih/event/trail/ 当日链
