# pkfix-parallel 任务包：泊界存量总修（并联编组）

> 令源：用户 2026-09-11 「补齐，然后拉多子代理一次性修复」。泊件来源 pk-094 全件加 pk-095 加 pk-097 加 pk-099 加 pk-100。
> 队形：并联四簇（甲 mcpline 写面、乙 lease 面、丙 引擎 scribe 报错、丁 gauge 域回退），主线验收结算。
> 会话：sess-zcode-260911-pkfix。域：双仓。

## 簇划分与交付面

- 簇甲 sih-tools/mcpline：pk-094 其一 lease_commit repo 形参 str/list 兼容（列表禁字面量串化，逐条透传；教学文写明工位路径形为成功形）；pk-094 其二 MCP 侧 record 系 valid_params 喂满；pk-099 8765 版本串单源化。
- 簇乙 sih-tools/lease：pk-097 其二 build_message 前缀去重；pk-100 guardcore 工位锁面读数盲区（git common dir 锚主树中央账本）。
- 簇丙 sih-engine/src/bin/scribe.rs：pk-094 其二 CLI 侧五处报错文附「应为文件路径」与合法形提示。
- 簇丁 sih-tools/gauge：pk-094 其五 gauge 底座域回退（域 root 无底座时回退中央 tool 底座并明示回退事实，非裸死）。
- 随批出泊：pk-094 其一其二其五、pk-095、pk-097、pk-099、pk-100。

## 明确不做

pk-094 其三（候 mcpboot）；pk-098 其一（锁生命周期候裁）；NAMESPACE_FACES 与派生规则本体。

## F 锚定

| F | 簇 | 判据 | 验法 |
|---|---|---|---|
| F-1 | 甲 | repo 列表形与单串形俱可达 CLI 且禁字面量串化 | pytest |
| F-2 | 甲 | record 系 valid_params 非空带路径形 | pytest |
| F-3 | 甲 | 8765 serverInfo 版本读包版本单源 | pytest |
| F-4 | 乙 | message 前缀 subject 首部去重 | pytest |
| F-5 | 乙 | 工地上下文锁面读数见中央账本 | pytest |
| F-6 | 丙 | scribe 五处报错文附路径提示 | cargo test 加活体 |
| F-7 | 丁 | gauge 域内无底座回退中央并明示 | pytest |
| F-8 | 全 | 既有测试零回归 | 四面全套绿 |
| F-9 | 主线 | 验收抽检加双仓 settle 链笔 | 链 verify |
