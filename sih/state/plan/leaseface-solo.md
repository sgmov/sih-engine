# leaseface-solo 任务包：lease commit 旗标对齐与 allow 派生可见性

> 令源：用户 2026-09-11 「需要过得一裁，过了就立批走租约」；前置双终签在链 m-leasefix-1 与 m-leasework-1（stable_clear 九发合同模式，direction 俱 violate 即缺陷态被判违规）。
> 队形：单线 solo，零子代理。会话：sess-zcode-260911-leaseface。域：sih-tools 主。

## 交付面

1. `sih-tools/lease/src/lease/cli.py` commit 子命令补 `--locks` 旗标（与自举闸三参要求对齐）。
2. 自举闸拒绝载荷增 `resolved_defaults` 三键（ledger、locks、bills 已解析缺省绝对路径），解析逻辑上移闸前复用。
3. lease open 的 issued 回执增 `allow_effective` 与 `allow_derived` 两键（NAMESPACE_FACES 父声明被保留位替换的对照回显）。
4. `sih-tools/mcpline/AI-MANUAL.md` 剧本 C 补 NAMESPACE_FACES 派生教学句；`sih-tools/lease/CONTRACT.md` 修订笔。
5. 回归测试：工位内三参全传 commit 可过闸、缺参仍拒且载 resolved_defaults、commit --help 含 --locks、namespace_reservations 返回对照。

## 明确不做

first_domain 自举闸自动满足语义（根修候裁另批）；NAMESPACE_FACES 冻结清单内容变更；namespace_reservations 派生规则本体变更。

## F 锚定

| F | 判据 | 验法 |
|---|---|---|
| F-1 | commit --locks 旗标在面且工位内三参全传可过闸 | pytest 工位子进程 |
| F-2 | 缺参拒绝载荷载 resolved_defaults 三键 | pytest |
| F-3 | open 回执含 allow_effective 与 allow_derived | pytest + 实测 |
| F-4 | 手册教学句与 CONTRACT 修订笔在案 | 对表 |
| F-5 | lease 既有测试零回归 | 全套绿 |
