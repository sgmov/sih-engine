# leaseface-solo 结果档：lease commit 旗标对齐与 allow 派生可见性批

> 批：leaseface-solo（单线 solo，零子代理）。会话：sess-zcode-260911-leaseface（session_id dce82fcd9f8363cf）。
> 令源：用户 2026-09-11 「需要过得一裁，过了就立批走租约」。前置双终签在链：m-leasefix-1 与 m-leasework-1（stable_clear 九发合同模式同席回填，direction 俱 violate 即缺陷态被判违规）。

## 一、交付面

1. `lease/src/lease/cli.py` commit 子命令补 `--locks` 旗标：与自举闸三参要求对齐，工位内 commit 结构性死锁（m-leasefix-1）修复。
2. 自举闸拒绝载荷增 `resolved_defaults` 三键：ledger、locks、bills 已解析缺省绝对路径进拒载（canonical 域落 `<域根>/sih/ledger/`，first_domain 落 `tool_dir()/ledger/`），教学从口号变实物（pk-094 其四完整修复）。
3. lease open 回执增 `allow_effective` 与 `allow_derived` 两键：NAMESPACE_FACES 父声明被保留位替换的对照进回执（m-leasework-1 可见性修复）；台账行形态零变更（两键仅回执层）。
4. `mcpline/AI-MANUAL.md` 剧本 C 后附命名空间派生教学段；`lease/CONTRACT.md` 修订五十四。
5. 回归测试 `lease/tests/test_commit_gate.py` 三测（子进程加进程内 monkeypatch 形）。

## 二、F 锚定

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 | 闸面对齐 | commit --locks 旗标在面且工位内三参全传过闸 | 通过（test_commit_parser_accepts_locks_flag 加 three_flags_passes_gate） |
| F-2 | 裁判面 | 缺参拒载携 resolved_defaults 三键 | 通过（without_flags_rejected） |
| F-3 | 可见性 | 回执两键接线在源 | 通过（cli.py 派发段加 drift 源钉） |
| F-4 | 零回归 | lease 全测试面 | 通过（355 passed） |
| F-5 | 结算治理 | intent 笔与认证笔在链、双仓 settle 落、锁零会话吊销 | 通过（intent 967a208d、cert 见链、close 后双零） |

## 三、边界申报

1. first_domain 自举闸自动满足的根修（行为放宽）不入本批，候闸主裁另批。
2. NAMESPACE_FACES 冻结清单与派生规则本体零改动（仅回显与教学）。
3. 结果档落 event/plan/leaseface-solo/ 派生子目录（open allow 面派生规则所定，--namespace 教学已入手册）。
