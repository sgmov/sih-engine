# domface-solo 结果档：域面闸组首批（pk-096 域锚修复加 pk-098 其二 archive 枚举教学）

> 批：domface-solo（单线 solo，零子代理）。会话：sess-zcode-260911-domface（session_id 10e1d30ec4daa4e0）。
> 令源：用户 2026-09-11 「开始修复」。前置泊件 pk-096 与 pk-098 其二。

## 一、交付面

1. `sih-tools/mcpline/src/mcpline/server.py` _nomenclator_check 域锚修复（pk-096）：相对 target 存 layout 即锚 layout.root，layout None 才回退 resolve_root()；params 教学文同步改「所绑域根相对或绝对形（layout 缺席时回退中央根）」。
2. `sih-tools/mcpline/tests/test_nomenclator_check_anchor.py` 三测：layout 域根锚定、None 回退中央根、params 教学文与实锚一致（源钉形）。
3. `sih-engine/src/retriever/mod.rs` RecallError::Blocked 载荷附五档全表：实测「档名非枚举值 bogus（合法值：fact、conclusion、experience、parked、intent）」（工地板 6.02s 构建后 CLI 活体验证）。
4. 引擎 retriever lib 测试 13 passed；release 构建 20.22s 成。

## 二、F 锚定

| F | 类别 | 判据 | 实态 |
|---|---|---|---|
| F-1 | 域锚 | layout 存在时相对 target 锚所绑域根 | 通过（test_relative_target_anchors_bound_domain_root） |
| F-2 | 域锚 | layout None 回退中央根 | 通过（test_relative_target_without_layout_falls_back） |
| F-3 | 教学 | params 教学文与实锚一致 | 通过（test_params_teaches_bound_domain_anchor） |
| F-4 | 教学 | archive 报错附五档全表 | 通过（工地板 CLI 活体验证） |
| F-5 | 结算治理 | intent 笔与认证笔在链、双仓 settle 落、锁零会话吊销 | 通过（intent 925ad8a0、cert 见链、close 后双零） |

## 三、边界申报

1. 主树引擎二进制（target/debug 与 release 的 retriever）需归并后重建方载新报错载荷；MCP 投影经子进程逐调执行，重建即生效零重启。
2. 司梦域孤儿锁（d4323d98 八把 web/ 加 ava-exam 十一把）不在中央账本，清偿属其域人节点 CLI 位，本批够不着如实记。
3. pk-098 其一锁生命周期设计变更候裁另议，本批零触碰。
