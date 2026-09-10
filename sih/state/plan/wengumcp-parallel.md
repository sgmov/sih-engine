# wengumcp-parallel 批任务包：M5 修复——温故投影 MCP 只读面与新城域自身档案索引

> 令源：用户 2026-09-11 令「同意。开始M5的修复，你拉子代理」承司梦侧 M5 发现（温故未上 MCP 面且索引不覆盖新城域）与 domaware-solo 完工报告候令项
> 形：**并联 parallel**（parallel／并联 established 在册）——并行簇子代理领 mcpline 投影簇，主线簇亲写最复杂件即温故 Rust 本体，主线串行验收。**并联形在租约下零活体，本批即首个活体，随批申报**
> version: v1

## 一、使命

把温故（retriever，DEC-017 登记的只报不判检索面）从引擎域裸 CLI 搬上 MCP 工具面，并让它的根定位与档案索引认得新城正典域（canonical sih/ 形）——城内代理经 MCP 一调即检本域沉淀，不再离会手敲引擎 CLI 且查不到本域档案。判定语义零新增：投影是透传，索引扩展是确定性路径规则。

## 二、钉死契约（两簇共同对表面）

**布局判别与两根分离（Rust 侧与 MCP 侧共用语义）**

- 布局两形：root 下 sih-engine 与 sih-tools 目录俱在即 first_domain（中央第一域，现形逐字节零变）；root 下 sih/ledger 为目录即 canonical（新城正典形，与 mcpline 开域落地形及 lease detect_domain_context 同构）；root 即显式 --root 给定或 cwd 上溯派生
- **数据根／码根分离**：canonical 形下检索数据（trail 与档案文件）取本域 root，locator 码根（sih-tools/locator 工具与 uv 项目）自 root 向上溯最近祖先供给——城市零 sih-tools 也可检，中央码根全域共用（critsweep 泊界路由与 mcpnomgate stem 闸两根分离先例同构）

**retriever CLI 增旗标（bin/retriever.rs）**

- `--root <路径>`：显式数据根，给定即不 walk（路径须为布局根，两形标记俱缺即 exit 2 fail-closed 教学报文列缺什么）；缺省 cwd 上溯现形不变
- 其余九参数零变更，退出码三值零变更，stdout NDJSON 零变更

**canonical 城档五映射（archives.rs 分形判档表）**

| 城 rel 路径 | 档属 |
|---|---|
| sih/event/plan/*-results.md | Conclusion |
| sih/state/plan/*.md | Intent（任务包即意图精炼载体） |
| sih/state/parking/materials/*.json | Parked |
| sih/state/parking/PARKING-v1.md | Parked |
| sih/event/trail/*.ndjson 链事件 | classify_event 现表零变 |
| Experience | 城无对应档案，零命中如实 |

**canonical 记忆包（locator_bridge.rs 内嵌）**

- 常量内嵌 canonical include 表（上列四路径 glob）＋空 exclude，build 至系统临时文件传 locator --pack，零城侧配置零工作区落盘
- first_domain 形记忆包路径与内容逐字节零变

**MCP 投影（mcpline server.py 与 httpface.py）**

- 工具名 `retriever_recall`（镜像 CLI 子命令，nomenclator_query 命名先例）
- 入参：topic 数组可选、word 数组可选、event 数组可选、since 与 until 与 archive 与 at 可选；四轴全缺即拒教学（镜像 CLI 轴全缺报文）；out 与 miss-log 不投影（工具面零写盘）
- 承接：spawn 主树 `sih-engine/target/debug/retriever recall ... --root <域根>`；stdio 面 root=中央根，HTTP 面 root=令牌所绑域 SIH_ROOT（writeface 域解析同源）；stdout NDJSON 透传、退出码透传（0 成功 1 拦 2 异常）、stderr 错误透传
- 面计数：stdio 19 升 20、HTTP 17 升 18（α 相七读数升八读数）；矩阵 local 与 external 俱可调（只读形）

## 三、簇拆分（并联形）

- **并行簇 A（子代理）**：mcpline 投影全套——server.py 与 httpface.py 与 writeface/matrix.py 与 tests/（透传、面计数、矩阵行）与 README、SPEC-023 修订二、AGENTS.md MCP 节计数（原地改归档入 engine 工地材料）。只碰上列工具面文件，零触 Rust。
- **主线簇 B（主线亲写，最复杂件）**：retriever 四文件（mod.rs 与 archives.rs 与 locator_bridge.rs 与 bin/retriever.rs）＋ Rust 测试（canonical 夹具命中、第一域零回归、--root 显式与拒面、locator 祖先回溯、内嵌包）＋ SPEC-007 修订（interface-signature 与布局两形与城档映射表）。
- **依赖簇**：无串行簇；收敛后主线 e2e 验收（canonical 夹具城经 CLI 与 MCP 两面检索命中）。

## 四、工作清单

- [ ] T-1（B 簇）Rust 四文件改动与测试
- [ ] T-2（B 簇）SPEC-007 修订
- [ ] T-3（A 簇）mcpline 投影与测试与 README
- [ ] T-4（A 簇）SPEC-023 修订二与 AGENTS.md 计数
- [ ] T-5 主线 e2e 收敛验收（canonical 城夹具两面检索）
- [ ] T-6 管线三步（两 SPEC 与任务包与结果档）与新词登记两笔（wengumcp 与 retriever_recall）
- [ ] T-7 认证上链与双仓 settle 与收约对账与主树真跑（含 cargo build 重编与 cargo test 主树跑）与回锚回显

## 五、可证伪条件（跑前立文）

| F 锚定 | 类别 | 判据 |
|---|---|---|
| F-1 | 根 | canonical 夹具根 derive 与 --root 显式两路俱命中城根；两形标记俱缺即 exit 2 教学报文 |
| F-2 | 索引 | canonical 城档夹具 topic 与 word 轴检索命中结论档与意图档与悬置档；first_domain 既有 Rust 测试零回归逐字节 |
| F-3 | 投影 | retriever_recall 两面透传（出参 NDJSON 与退出码 0/1/2）与 CLI 一致；轴全缺拒教学；out/miss-log 零投影 |
| F-4 | 面计数 | stdio 20、HTTP 18 断言绿；α 相八读数在 AGENTS.md 与 SPEC-023 修订二在档 |
| F-5 | e2e | canonical 夹具城经 CLI 裸调与 MCP 面调用俱检索命中本域沉淀 |
| F-6 | 回归 | cargo test 主树跑全绿（重编后）与 pytest 三族（mcpline 与 lease 与 gauge）主树跑全绿 |
| F-7 | 文档 | 两 SPEC 修订走管线三步；新词登记两笔在册且 query established |

## 六、必读文件

- sih-engine/src/retriever/mod.rs 与 archives.rs 与 locator_bridge.rs（现形）
- sih-engine/doc/spec/SPEC-007-project-memory-component.md（温故权威）
- sih-tools/mcpline/src/mcpline/server.py 与 httpface.py（nomenclator 投影先例现形）
- sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md（面契约与修订一现形）
- sih-tools/mcpline/tests/fixture_root.py（canonical 城夹具参照）
- 温故落包前检索件：wengumcp-parallel-materials/recall-topic-batch.md 与 recall-event-axis.ndjson（零缺席在档）

## 七、约束（红线）

1. 温故只报不判（DEC-017 既判）：投影零判定语义新增；MCP 面零新执行者（SPEC-023 红线承袭）
2. 中央第一域布局零迁移；first_domain 形输出与索引逐字节零变
3. 引擎 Rust 改动限 retriever 目录三文件加 bin/retriever.rs；其余组件零触碰；合并后主树 cargo build 重编方作验收（旧二进制勘误承 0803 事故）
4. out 与 miss-log 不投影 MCP 面（工具面零写盘）；读面零写零治理语义
5. 主树零直写；每条命令立即取退出码失败即停禁管道掩码；先红留痕禁清洗
6. scribe 二进制一律主树 target/debug/scribe；子代理产出属符号材料入库前过主线验收

## 八、风险点

- 并联形租约下零活体：本批即首跑，簇间碰面风险以文件面零交集对冲（A 簇只碰 mcpline 与 SPEC-023，B 簇只碰 retriever 与 SPEC-007），收敛验收归主线
- 他会话 28d1c5c3 持 sih-tools/mcpline 锁：open 预检必撞即候锁轮询不绕行（packhyg 纪律，mcpboot 先例）
- locator uv run 依赖 uv 在 PATH：MCP 面调用环境与 CLI 一致性由透传测试钉
- cargo 重编后主树二进制为唯一验收位；工地内 cargo test 绿不算主树绿（F-6）
- AGENTS.md MCP 节计数变更与 SPEC-023 修订二措辞须逐字对表（α 八读数），漏一处即红

## 九、范式偏离声明

无。选形制在位：并联形经用户 2026-09-11 令「你拉子代理」触发，parallel／并联 established 在册，主线簇亲写最复杂件与并行簇子代理各领与主线串行验收俱按 skill 工作流；并联形租约下零活体之谱系注记由本批终结，随批申报。

## 十、关联文件

- SPEC-007（温故权威）；SPEC-008（五档映射落差节）；DEC-017（温故只报不判既判）；mcpnomgate-solo（MCP 投影与两根分离先例）；domaware-solo（canonical 布局判别三工具先例与 M5 报告源）；mcpauth-solo（每项目一域既裁）
- 司梦 M5 发现（司梦工作区评估报告）：温故未上 MCP 面、索引不覆盖新城域两病灶

## 十一、请求写入（逐路径分行）

sih-engine/src/retriever/mod.rs
sih-engine/src/retriever/archives.rs
sih-engine/src/retriever/locator_bridge.rs
sih-engine/src/bin/retriever.rs
sih-engine/tests/
（目录级理由：retriever canonical 夹具测试新件出生地）
sih-engine/doc/spec/SPEC-007-project-memory-component.md
sih-engine/doc/spec/SPEC-023-mcpline-alpha-readonly-v1.md
sih-engine/sih/state/plan/wengumcp-parallel.md
sih-engine/sih/event/plan/wengumcp-parallel-results.md
sih-engine/sih/event/plan/wengumcp-parallel-materials/
（目录级理由：批材料出生地，先红留痕与温故 recall 与管线读数入档）
sih-engine/sih/event/trail/2026-09-11.ndjson
sih-tools/mcpline/src/mcpline/server.py
sih-tools/mcpline/src/mcpline/httpface.py
sih-tools/mcpline/src/mcpline/writeface/matrix.py
sih-tools/mcpline/src/mcpline/__init__.py
sih-tools/mcpline/pyproject.toml
sih-tools/mcpline/README.md
sih-tools/mcpline/tests/
（目录级理由：投影测试新件与面计数断言更新地）
sih-tools/nomenclator/packs/core/
（目录级理由：新词登记两笔走 register 写位与 manifest 升位）
sih-tools/scribe/reports/
（目录级理由：ask3 双门与叩问与认证报告出生地）
sih-tools/identity/reports/
（目录级理由：正身件出生地）
AGENTS.md
（工作区根文件不在 git 仓内，原地改归档入 engine 工地——agentslim 先例）
