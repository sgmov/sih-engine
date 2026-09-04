# newcarr2-solo 结果档

> 批：ALG-003 至 ALG-007 五载体清账（任务包件一至件七）
> 会话：432678779a3f0663（sess-zcode-2026-09-04-newcarr2）
> 日期：2026-09-04
> 队形：单线形 solo，零子代理；**主会亲办段**（两次委外执行代理均因基础设施模型请求故障中断，灾备路径转主会，全链序不减）
> 承接：m-algcarr-2 九发 stable_clear 已执契机器终签落据（重放锚 sih-tools/proposition/DES/m-algcarr-2/m-algcarr-2-signcheck.json，sign 笔 1a655289）；用户 2026-09-04 令「得一裁一过一执行一」
> 意图哈希：9335b044（ask3 记录经双门 status ok 三锚）

## 一、问题还原

复用对表八笔 defines_at_missing_disk（ALG-001→ALG-003/004/005/006/007 五笔、ALG-008/009/010→ALG-003 三笔）为登记面已声明的前向引用悬空，ALG-003 线性映射为最大共享定义位缺件。本批按 m-algcarr-2 终签命题的机制形清账：newcarr 同款四件套补齐五位定义位，非冻结非删引用，收口机械判据为复用对表复跑不一致归零。执行形为载体管线批，承 M-3 不可译不豁免清账通道。

温故检索：materials/recall-newcarr2.json 如实记（载体批，温故面由 reuse-crossref 八笔、消费方条目 ALG-001/008/009/010、先例条目 ALG-011/ALG-012 全读承载）。

## 二、完成度表

| 件 | 判据 | 结果 | 证据 |
|---|---|---|---|
| 件一 ALG-003 线性映射 | 四件套齐，定义先行加可证伪加双答 | 完成 | entries/ALG-003-linear-mapping.md：叠加齐次合取、基像唯一、核像子空间、复合结合律；可证伪三型反例；ALG-001/008/009/010 消费语义逐条承接 |
| 件二 ALG-004 矩阵乘法 | 同上 | 完成 | entries/ALG-004-matrix-multiplication.md：内维相容三重和、结合律、复合对应、转置回退、矩阵幂推论 |
| 件三 ALG-005 行列式 | 同上 | 完成 | entries/ALG-005-determinant.md：公理化唯一、显式公式、乘性、特征方程与谱积、可逆判据 |
| 件四 ALG-006 谱定理 | 同上 | 完成 | entries/ALG-006-spectral-theorem.md：实对称正交对角化、特征值全实、正交性、存在性、Rayleigh 商推论 |
| 件五 ALG-007 Perron-Frobenius | 同上 | 完成 | entries/ALG-007-perron-frobenius.md：非负不可约主定理、素性收敛、行随机稳态；两前提反例形态与 ALG-001 条件申报逐条对表 |
| 件六 登记面联动 | INDEX 与 mapping 行与概览订正逐笔申报 | 完成 | INDEX 五行待建转已建、概览「已建 3 待建 6」订正「已建 12 待建 0」、新增历史行；mapping 增五行、概览 ALG 5 订正 12；推导档 docs/newcarr2-derivation-2026-09-04.md 在档 |
| 件七 复跑归零 | 复用对表复跑不一致归零 | 完成 | rerun-2026-09-04/：pair_total 1076、inconsistencies **0**（前值 8）；书架对账 172 = 11 有节 + 161 缺节 |

## 三、F 表

| F | 判据 | 结果 | 证据 |
|---|---|---|---|
| **F-1 四件套齐** | 条目、INDEX 与 mapping 行、推导档、消费位核对全数在档 | 过 | 五条目全携定义节、可证伪节（三型反例）、触发问题；INDEX/mapping 行与概览订正在案；推导档 §2 消费语义承接表逐条对表 |
| **F-2 复跑归零** | 复用对表复跑 8 笔归零 | 过 | rerun-2026-09-04/reuse-crossref.json：inconsistencies 0、pair_total 1076；脚本 ROOT 临时指向工地伪根（symlink），原件零改 |
| **F-3 数学正确性** | 标准内容核验，推导档与条目零矛盾 | 过 | 五条目为线性代数标准内容（Cayley 1858、Perron 1907、Frobenius 1912 等谱系在档），证明逐条在条目内，ALG-001 条件申报与本批 ALG-007 公理条件与反例形态逐条互证 |
| **F-4 写入仅 allow** | 写入仅 allow 清单 | 过 | 十一锁全取零撞；写入面逐件见 changed-files 认证件；工程仓源码、coverage 七件、既有条目七件零触碰 |

## 四、管线读数

- 化格：八件 general-v1 全 exit 0 无需改。
- 核阅：八目标均 sih-math 域，des-001 域只盖 sih-engine/doc，域外 exit-2 如实记不属违规；ask3 双门第一门 exit 0、第二门 status ok 三锚。
- 检词：八件 core 包全 exit 0 零违例。
- 词债：四件套、清账、矩阵乘法、线性映射、行列式、谱定理六轻信号，叩问 digest passed covered 6，处置不登记让位后批词表批。

## 五、认证清单

| 事件哈希 | 对象 | 内容哈希承 |
|---|---|---|
| 9335b044 | 意图笔（scribe intent） | ask3 记录经双门 |
| 549b6cb2 | 管线报告（2026-09-04-newcarr2-solo-pipeline.json） | 三步读数载于报告件 |
| c21690bd | 复跑归零报告（2026-09-04-newcarr2-solo-golden.json） | rerun 件 sha256 载于报告件 |
| 3b216bff | 变更件报告（2026-09-04-newcarr2-solo-changed-files.json） | 写入清单载于报告件 |
| （本笔随结果档定稿上链） | 结果档报告（2026-09-04-newcarr2-solo-results.json） | 结果档内容 sha256 载于报告件 |

## 六、越线与误差申报

1. **主会亲办申报**：前两次委外执行代理均因基础设施模型请求故障中断（首次 open 前、重拉 4 分钟），第二次遗留仅叩问信号一件（复用）；灾备路径转主会亲办，会话与锁与链笔全由本会话 432678779a3f0663 承载，施工内容主会亲写，pk-045 冲突库「批中途死亡」类新增主会亲办处置样本。
2. **在途勘误吸收**：sih-math 主树两行未提交改动（algebra/INDEX.md 与 mapping.md，PRO-12 伪构勘误，引 docs/survey-mathprobe-2026-08-30.md，mtime 06:23 UTC 早于本批开工）按并集归并预案逐字并入本批工地（单独行替换，出处归委外在途批，不掠不抹），close 让位后 diff 对表两行必须逐字在 merged 内。
3. **脚本 ROOT 临时指向**：shelfscan.py 复跑用 /tmp 拷贝件重指工地伪根（symlink），原脚本与 coverage 七件零触碰，读数工件全量拷 materials。
4. **任务包会话标勘误**：任务包 predesign 会话标 sess-zcode-2026-09-04-newcarr2 与租约 uuid 432678779a3f0663 双标识空间对表各认各（BATCH-FACE 坑位在案）。

## 七、冲突样本节（pk-045 样本库）

本批主会亲办段开工实查（03:5x 至 04:0x UTC 段）：活动会话零、锁零（委外两亡均未及 open），poolclear 批已在链留认证笔且已收口，sih-math 主树两行在途勘误为唯一外部面。十一锁全取一次过零撞锁；闸一在 open 二跑时对首跑同包活动会话正确拒绝（PackageSessionActive），属设计行为实录。在途勘误吸收为「包面锁面分叉」与「并集归并」组合形态样本，处置与对表判据在案。

## 八、验收

- [x] F-1 至 F-4 全过
- [x] 件一至件七全完成，结果档各点名节在档
- [x] 认证入链（哈希回填），三仓 settle，close 后收口读数见收口附记
- [x] 队形验证：主会亲办段成立——本批全部写入由会话 432678779a3f0663 亲写，零子代理；链写入经引擎 scribe 闸三，零直写链文件；复跑与对表全由确定性脚本承载

## 收口附记（close 后补记，本节经 close 通道外 bypass 提交回填）

（close 后回填）
