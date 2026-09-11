# pkfix-parallel 结果档：泊界存量总修（并联四簇）

> 批：pkfix-parallel（并联编组四簇甲乙丙丁，主线验收结算）。会话：sess-zcode-260911-pkfix（session_id f74e3153a6c61773）。
> 令源：用户 2026-09-11 「补齐，然后拉多子代理一次性修复」。前置：散账补齐双泊 pk-099 加 pk-100（声明笔 26d2cc1d）。

## 一、簇结算

| 簇 | 交付 | 测试 | 验收 |
|---|---|---|---|
| 甲 mcpline 写面 | pk-094 其一 repo 形参 union 兼容（passthrough commit_repo_single 归一，多条教学拒）；pk-094 其二 record 系 valid_params 喂满；pk-099 版本串单源（根因实勘：FastMCP 零 version 参落 mcp SDK 1.30.0，runtime 注入 pkg_version 单源） | 178 passed | 主线复跑同数 |
| 乙 lease 面 | pk-097 其二 build_message 前缀去重（剥序自长及短）；pk-100 guardcore 经 git common dir 锚中央账本加钩子接线（GUARD_VERSION 1.17.0）端到端两态实测；pk-095 _default_trails 三居所补 canonical 域家 | 368 passed | 主线复跑同数 |
| 丙 引擎 scribe | 五处报错文教学化（missing_file_reason 三态：缺文件/目录/空串），五行俱活体验证 exit 2 | cargo test 189 passed 全套零回归 | 主线活体抽验过 |
| 丁 gauge 域回退 | pk-094 其五 底座中央回退（两典映射加显式回显 evidence_basis/fallback，域内齐备零变，trail 不回退混域卫 fail-closed） | 56 passed 3 skipped | 主线复跑同数 |

## 二、随批出泊

pk-095、pk-097、pk-099、pk-100 四账出泊（出泊笔四笔在链，disposition promoted）。pk-094 部分出泊：其一其二其五已修，其三候 mcpboot、其六候 close 透明小改，泊位维持。

## 三、边界申报

1. 甲簇偏离：实改 runtime.py 与 httpface.py（简报语义覆盖位）；版本串根因实勘为 FastMCP 零 version 参落 SDK 版本 1.30.0（非字面常数），1.30.0 之谜破案。
2. 乙簇偏离：pk-095 并入本簇（原域形感知批拆并）；hooks/pre-commit 接线属 pk-100 语义覆盖位。
3. 丁簇缩小申报：中央发现沿 _central_code_root 上溯（工作区内 checkout 形可达），工作区外纯 wheel 域装不可发现维持原报错，未引入硬编码。
4. 不做面零越界：pk-094 其三、pk-098 其一、主树无主五件零触碰。
