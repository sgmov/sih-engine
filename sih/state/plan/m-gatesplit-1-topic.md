---
title: 采样闸判据 v3 单旗语义应修订为 v4 双旗分立
authored: m-gatesplit 批起草，AI 起草（ZCode/GLM-5.3），2026-09-24；方向裁定承用户原话在档
ng: medium
n: 9
gid: m-gatesplit-1
---

# 待裁命题

判据 v3 之 boundary_flag 单旗语义应修订为 v4 双旗分立——boundary_flag 重定义为裁判信心信号（对判定本身拿不准举 true），新增 coverage_flag 为题材覆盖描述信号（命题题材超出所给规约直接覆盖范围举 true）；闸三态判定只统计信心旗（阈值 0.2/0.34/0.10 承 v3 不变），coverage 旗单列记账（计分材料 coverage_flags 计数与 reason 摘引）不参与三态不在线扰人；旧四键响应兼容（coverage 缺席即 false）、历史 trail 零改写向前生效。承用户 2026-09-24 方向裁定原话：「在司衡体系内，人与LLM同权，我引入得一就是要把LLM的责任转给固定程序，而固定程序又是人的授权。按照这个逻辑，闸门的修复应该也是同样的方向，而不是又丢给人」。病灶证据：m-siacarr 批三轮采样十八发全合零变卦而牌率 33–55% 常驻警示带，根因即 v3 单旗把话题属性（超纲）与裁判信心（拿不准）缝在一根旗上。本命题单基线锚即可验证性，五 F 全过即事实面达成：maturation.rs 判据 v4 单测九件与集成三件绿即过 F-1；既有 attractor 全测与金向量逐字节对表通过即过 F-2；sih-tools 围堰零写入（git status 对证）即过 F-3；本命题经 v4 闸九发 stable_clear 即过 F-4；写入仅 allow 清单（src/attractor、tests、sih/state/plan）即过 F-5；任一 F 不过即整批退回零合入（双态可证伪）。

## 谱系披露与对己不利声明

起草与采样作答与裁定执行位同族（ZCode:GLM 同族席位）。对己不利声明第一重：v4 若立，本席后续一切采样命题的举牌将只反映真不确定，本席借"超纲牌"拖延或引导人介入的暗道即被关闭，判成立收窄本席操作自由；第二重：本席 m-siacarr 批正卡在 v3 病灶上，修闸本席即失去"闸有问题"作为该批停滞的陈述空间，后续须以内容而非机制为由重送。两重不利都指向 comply 从严。判定流按合同与响应哈希复算，本席位无法挑选作答；证据为既成链档非本批新造。

## anchors

- path: sih/state/plan/m-gatesplit.md
  range: ""
  note: 任务包含病灶证据与设计
- path: sih/state/plan/m-gatesplit-recon.md
  range: ""
  note: 三落点侦察与围堰冻结发现
- path: sih/state/plan/m-gatesplit-impl.md
  range: ""
  note: 实施与测试记录即 F-1/F-2 面

## 前置分道披露

事实面（v3 病灶证据、实施与测试结果、围堰零写入）已由链档与 git 状态机械钉死，本采样只裁应然面即判据应否由 v3 修订为 v4。判定依据限于所给规约与命题文内事实，不得引入命题外信息。
