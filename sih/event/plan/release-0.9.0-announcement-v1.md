# 司衡引擎 0.9.0 发布宣告

宣告位：主窗终验执行位，2026-09-09。承 DEC-022 版本定约与 doc/plan/release-0.9.0-v1.md 发布清单，tag v0.9.0 已于本仓主线 f7dd9ca 打立（annotated），本件即清单第九步发布宣告落档件。清单第八步推送远程候人节点令，本宣告不含推送。

## 执行实录

一 版本位：Cargo.toml 0.1.0 改 0.9.0，commit 500487a，bypass 留痕在册。二 构建验证：cargo build rc=0。三 测试验证：库测 191 绿（含金向量 des-001-gov002 重冻 5ea53e5，GOV-002 追记哈希漂移单行 diff），mem_recall_f_suite 8 绿 1 红即 f2_refs_machine_verifiable 存量红，已载入清单已知边界第五项带债放行，修复归 recallfix-solo 批（在飞）。四 工具线冒烟：lease reconcile 双仓 unrouted 0 cert_missing 0。五 链验证：当日链 status valid。六 MCP 面冒烟：stdio 工具面十五具即 alpha 五只读加 beta 十写，takeover 与 bypass 结构性缺席与 DES-014 矩阵一致，alpha 五工具回归全绿，锁面净态零持锁零在册。七 打 tag：v0.9.0 归于 f7dd9ca。八 推送：候人节点令未执行。九 发布宣告：即本件。

## 主线提交链（本发布新增）

500487a 版本位对表、5ea53e5 金向量重冻、f7dd9ca 已知边界增补第五项，三笔俱 lease bypass 显式留痕在册。

## 已知边界

五项载明于 doc/plan/release-0.9.0-v1.md 已知边界节：beta 写面部署未配置、GOV-002 v3 候裁、出参超集申明、帮助面漂移、f2 引用抽取存量红。

## 回滚法

四腿见 doc/plan/release-0.9.0-v1.md 回滚法节：删 tag 腿、版本位还原腿、MCP 注册摘除腿、零数据迁移申明。
