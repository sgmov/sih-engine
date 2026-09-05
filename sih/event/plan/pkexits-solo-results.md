# pkexits-solo 结果档（三件出泊确定性核对道 + A/B 设计采样道）

> 批：pkexits-solo（用户令「全部用新的得一裁继续」执行批）
> 会话：ea73245a2d2765c1（ask3 侧 sess-zcode-260905-pkexits，双标识空间各认各）
> 日期：2026-09-05。队形：单线形 solo，零子代理。

## 一句话结论

四件裁决全部经新得一裁承载收官：核对道三件全过（xcheck-049/048/054），采样道 m-abdesign-2 首发刀锋打回经通道分解改写一次后九发单源 stable_clear 执契终签 d2977bdf，三笔出泊 promoted 落链（pk-049 a8d57696、pk-048 b7fabeae、pk-054 8ca3e604），名录九项与链一致，GD_ALPHA 0.05 随 pk-049 追认为在役检验水平。

## 一、F 表（完成度表）

| F | 判据 | 结论 | 证据 |
|---|---|---|---|
| F-1 核对道 | 三份 crosscheck 全绿 | 过 | xcheck-049（v2 修正凭据位置后四断言全过）、xcheck-048、xcheck-054 在批材料 |
| F-2 采样道 | stable_clear 执契 | 过 | m-abdesign-1 boundary（刀锋）→ 通道分解改写 m-abdesign-2 stable_clear → 终签 d2977bdf |
| F-3 出泊 | 三笔 promoted 在链 | 过 | 三 event grep 验证在档，裁定文载用户令与核对/测量结论 |
| F-4 投影 | 名录九项一致 | 过 | 十二减三，历史住户二十项 |
| F-5 收口 | 见收口读数 | | |

## 二、新判据首战记录（生产判定标准变更后的第一组治理测量）

- 首战 m-abdesign-1：九发全 comply、变卦 0%、谨慎 0——旧占比判据下应为 clear/near，**新二项检验判据判 boundary（刀锋类）**，成因即命题规范面横跨基线一/四/五三通道致依据族三值分散。闸比旧标准显著更严的语义由本例直接证实。
- 改写 m-abdesign-2：按修订一通道分解收窄至基线四单通道（基线一/五事实面归确定性核对 xcheck-054 已核），九发单源 stable_clear。改写 1 次（R7 线内），谱系披露在 topic 文。
- 对比参照：gateswitch-switch-2 同日以同法（单通道收窄）获 stable_clear——通道分解改写法在新判据下两连验证有效。

## 三、越线与误差申报

1. xcheck-049 首跑误报：凭据位置启发式读错形（archived_mismatch 实落 recheck 实测产物与邻件证据包，非 049 证据包本体），修正后复跑全过；误报已留档。
2. 席位基线两次排障：score --out 未落基线文件（记录在 calibration 账本，手工提取落盘）；借用他批基线致 R5 身份不配对挂起——本批以 temp_probe 模式一自标定解决（4 命题 5 发 20 行，violate 全票判违、体温 0.0、判定可用），基线件 contracts/pkexits-260905/seat-baseline-zcode-2026-09-05-pkexits.json。
3. 台账竞态第四例在案：gateswitch 批 issued 行遭并行重写洗失已由其 ledger-repair 补录；constmodel 会话行缺录仍待补——pk-057 优先级建议维持最前。
4. A/B 设计批准不解除 pk-044 硬前置：实装批开工前 pk-044 独立性来源仍须用户裁。

## 四、管线与链

- 三问双门、叩问 digest covered 4、正身 attest、意图 54d98e8b 在链
- 核对道：三份 crosscheck 报告零 LLM 纯机械
- 采样道：两 gid 各九发，合同哈希与响应哈希绑定，identity-report 必挂
- 认证与收口：见收口读数

## 五、收口读数（close 后回填）

- 待回填：双仓 settle 号、主树 des-001 复跑、reconcile、verify 终读、CALL-LOG 双笔 bypass 补笔
