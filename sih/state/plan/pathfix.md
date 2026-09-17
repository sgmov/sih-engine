# pathfix：lease 路径缺陷双修

> 令源：用户 2026-09-17 令「1要修」（bypass 台账误位缺陷）加 pk-089 问询（receipts_dir 写死仍活在引擎件即 closegate.rs:1049 实锤）
> 范式：solo 单线，agent 亲写零子代理
> 前置：orphansweep 批已收约；同日撤 sim-aesthetic-workbench 牌（两步确认停行在册）；工作台侧存量假骨架与五回执实查俱无踪，pk-089 剩余实体即代码病
> stem 认领：pathfix，甲表三件即 zh 路径缺陷修复、code 无承、派生 pathfix:new

## 一、问题陈述 {#problem}

- 缺陷一即 bypass 台账误位（orphansweep 批两遇实证）：commitlaw.rs cmd_bypass 的 root 取 `--root` 缺省 `"."` 即 cwd 相对，未传即拼出 `sih-engine/sih-tools/...` 迷路位落行；且零参校验，空 repo 空 sha 空事由照写台账（用法探针实锤空行落册）。
- 缺陷二即 pk-089 病在引擎件仍活：closegate.rs:1043 与 1049 加 sweepcore.rs:311 三处以 root 拼 `sih-tools/lease/ledger/{checks,receipts}` 第一域常量，对新城域 root 跑 close 或 sweep 即沉积假 sih-tools 骨架或漏读域内位。

## 二、关键设计 {#design}

- bypass 修形两钉：缺参即拒即 repo 与 sha 与 reason 俱必填，缺席退出码二带教学语；root 缺省改工作区根上溯发现即自 cwd 上溯找 sih-engine 与 sih-tools 双目录在位者，无果 fail-closed 拒落 cwd 缺省。
- 台账基座分流：新立 ledger_surface(root) 即 root 下 sih-tools/lease/ledger 在位走中央形，否则走新城正典形 sih/ledger，closegate 与 sweepcore 三处俱改走基座；中央形行为零改即既有金向量与 t6 套件不动。
- pk-089 出泊 promoted：receipts_dir 域感知即本批代码病修复承载，存量五回执与假骨架实查无踪即迁移与清理两件事实性免除，出泊笔上链。

## 三、工作清单 {#work}

- [ ] pf-01：批三件加 lease open 甲表认领加锁
- [ ] pf-02：工地施工四文件即 commitlaw 与 closegate 与 sweepcore 三改加 tests/pathfix.rs 新立
- [ ] pf-03：worktree 测试即 pathfix 新套件加 mergeback 六套件回归全绿
- [ ] pf-04：pk-089 出泊笔加 settle 加 close 加推双远端

## 四、验收 {#acceptance}

- pathfix 套件四件绿即缺参拒与根发现落正册与非工作区 fail-closed 与域形回执位；mergeback 六套件零回归；中央形路径逐字节不变；当日链 valid；reconcile 零新增。
