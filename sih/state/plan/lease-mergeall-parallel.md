# lease-mergeall-parallel 任务包（并联形）

## 形声明与令源

- 队形：并联形（parallel），主会立约取锁，并行簇三子代理后台各领一簇施工，主线亲写插件槽位骨架与总验收，依赖簇串行结算。
- 令源：用户 2026-09-14 goal 令全量融回（SPEC-025 正典先行）；温故检索先例 SPEC-024 三腿与腿二金向量形。

## 目标（本批 = SPEC-025 首战三腿加一骨架）

- 簇A（腿一治理例行三件）：attnanchor 加 critsweep 加 gauge Rust 融回。
- 簇B（腿二 T6 管线前两件）：formatter 加 nomenclator Rust 融回（parser 加 locator 让位后批，本批簇B两件控量）。
- 簇C（腿五插件骨架）：ToolProvider trait 加 ToolRegistry 加 plugin manifest 解析加 sihmcp 注册面改造演示（至少一具内置工具迁 registry）。
- 主线：三簇验收加测试补强加结算。

## 范围与要求（逐簇）

- 每件工具：围堰源码只读对表，引擎侧新 bin（src/bin/<tool>.rs）或 sihmcp 内模块（簇C），CLI 出参与退出码三值对表，测试先红后绿（tests/ 下新测试件，金向量形核心三例起：正常形加拒绝形加边界形）。
- 落差如实申报（ndjson 推导形与平台差与依赖缺席）。
- 子代理只写工地源码与测试，零 git 操作零台账零链写。

## 验收判据

- F1 簇A三件 bin 编译绿加每件核心三例测试绿。
- F2 簇B两件同上。
- F3 簇C registry trait 在役加 manifest 解析在役加至少一具演示迁移加单测绿。
- F4 全族回归不破（lease 全族加既有 bin 测试）。
- F5 SPEC-025 走 T6 三步（des-001 域内核阅绿）。

## 明确不做

- parser 加 locator 加腿三腿四腿六全部让位后批（SPEC-025 腿序）。
- 进程外插件桥实装（A4 申报缺席）。
- 围堰零改动；零新增 Cargo 依赖（A6）。

## 改动文件清单

sih-engine/src/bin/attnanchor.rs（新）、critsweep.rs（新）、gauge.rs（新）、formatter.rs（新）、nomenclator.rs（新）、src/sihmcp.rs 加 src/tools_registry.rs（簇C）、tests/mergeall_t1_gauge.rs 等新测试件、本包、SPEC-025、结果档与 materials、当日 trail。

## 验收补节（用户 2026-09-14 加严令：SDD/TDD 级联与 rmcp 重启实测）

- G1 级联更新：SPEC-025 正典对表全批件；工作区 AGENTS.md 工具层节与引擎 bin 清单级联对表（融回件经同参双跑对表围堰后声明引擎位）；围堰 CONTRACT 冻结零触碰；CALL-LOG 与结果档与泊界落差申报件随批落。
- G2 TDD 轨迹：三簇测试先红后绿轨迹在结果档如实登记（红证即首跑输出摘录）；主验收复跑全数。
- G3 同参双跑：融回 bin 与围堰 Python 同参双跑逐项对表（formatter 加 nomenclator 加 gauge 核心例），行为不一致即落差申报或修。
- G4 rmcp 重启实测：归并重建 sihmcp 后 kill 旧进程（75422 与 24445 双进程形态收敛为单进程），按原形态绝对路径工作区根 cwd 重启，实测三连即 initialize 加 tools/list 加只读工具调用；8765 面板页 500 为 sih-visual 静态资产缺席既有病如实申报非本批引入。
- G5 插件槽位在役验证：registrydemo 实跑加 registry 演示迁移 provider 与在役面元数据逐字节对表。
