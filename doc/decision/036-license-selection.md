# DEC-036 开源许可证选型

本裁定落 sih-engine 的开源许可证面：双许可证 SPDX 表达 MIT OR Apache-2.0，顶层两件许可证文本与 Cargo.toml 许可证字段随本批同形落地。令源即用户 2026-09-22 得一裁继续推进。

## 概览 {#overview}

- 裁定即双许可证 SPDX 表达 MIT OR Apache-2.0，顶层 LICENSE-MIT 与 LICENSE-APACHE 两件::[裁定](#verdict)
- 理由三条即 Rust 生态惯例双许可、crates.io 与 GitHub 平台自动识别、专利授予与简明互补::[理由](#reasons)
- 可逆性条款即 crates.io 首发前可经本裁定修订换轨，首发后换轨走新 DEC::[可逆性条款](#reversibility)
- 执行面即两 LICENSE 件与 Cargo.toml 字段随本批落地，CONTRIBUTING 与 CHANGELOG 候后批::[执行面](#execution)

## 裁定 {#verdict}

双许可证
: SPDX 表达即 MIT OR Apache-2.0。仓库顶层落两件许可证文本：LICENSE-MIT 承 MIT 许可证全文，LICENSE-APACHE 承 Apache License 2.0 全文。

Cargo.toml 字段
: package 的 license 字段填同 SPDX 表达 MIT OR Apache-2.0，与顶层两件互为表里，包面与仓面单源。

## 理由 {#reasons}

生态惯例
: Rust 生态双许可为惯例位，主流仓普遍以 MIT OR Apache-2.0 表达，使用者与贡献者对此形零认知成本。

平台识别
: crates.io 与 GitHub 对 SPDX 双许可表达自动识别并展示，无需人工注记。

互补
: Apache-2.0 载专利授予条款，MIT 胜在简明；两件并列让消费方按需择取，专利顾虑走 Apache-2.0，轻量引用走 MIT。

## 可逆性条款 {#reversibility}

换轨边界
: crates.io 首次发布前，许可证选型可经本裁定修订换轨；首次发布后，包面许可证已随包固化，换轨走新 DEC，不修本件。

## 执行面 {#execution}

随本批
: 两 LICENSE 件与 Cargo.toml 的 license 字段随本批落地。

候后批
: CONTRIBUTING 与 CHANGELOG 候后批落地，属迭代建议报告 REC-018 余项。

## 关联 {#relation}

SPDX 许可证表达规范；Cargo.toml 的 package license 字段；迭代建议报告 REC-018 余项清单；令源即用户 2026-09-22 得一裁继续推进。
