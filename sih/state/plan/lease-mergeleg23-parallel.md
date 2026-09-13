# lease-mergeleg23-parallel 任务包（并联形）

## 形声明与令源

- 队形：并联形（parallel），主会立约取锁，并行簇三子代理后台各领一簇施工，主线验收与结算。
- 令源：用户 2026-09-14 goal 令续推（SPEC-025 腿序：腿二余件与腿三七件，随后腿四）；正典 SPEC-025-toolful-mergeback-v1.md。

## 目标（本批 = SPEC-025 腿二余件加腿三全部）

- 簇D（腿二余件两件）：parser（围堰 1426 行）加 locator（756 行）Rust 融回。
- 簇E（腿三前三件）：selector（800）加 cascade（497）加 identity（503）Rust 融回。
- 簇F（腿三后四件）：tally（527）加 meter（272）加 watchcheck（380）加 calllog（488）Rust 融回。
- 主线：三簇验收加同参双跑抽验加结算。

## 范围与要求（承首战批先例）

- 每件工具：围堰源码只读对表，引擎侧新 bin（工地 src/bin/<tool>.rs），CLI 出参与退出码三值对表，测试件金向量三例起（正常加拒绝加边界）。
- 头 20 行内须载正典指针行（SPEC-025 全路径）——SDDG-2 窗口硬约束。
- 落差显式头注申报；子代理只写工地源码与测试，零 git 零台账零链写。
- 零新增 Cargo 依赖（A6）。

## 验收判据

- F1 簇D两 bin 加测试绿；F2 簇E三 bin 加测试绿；F3 簇F四 bin 加测试绿。
- F4 全族回归不破（lease 全族加首战批九 bin 加 registry 测试）。
- F5 同参双跑抽验（至少两件）与级联对表。

## 明确不做

- 腿四 MCP 分派面 registry 接线与 mcpline 收编让位下批（ SPEC-025 腿序「随后」）。
- 围堰零改动；进程外插件桥实装维持 A4 缺席申报。

## 改动文件清单

sih-engine/src/bin/{parser,locator,selector,cascade,identity,tally,meter,watchcheck,calllogface2}.rs 或对应名（新）、tests/mergeall_t3_*.rs 与 t4_*.rs（新）、本包、结果档与 materials、当日 trail。
