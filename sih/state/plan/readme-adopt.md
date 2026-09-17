# readme-adopt：README 接入节实测修订

> 令源：用户 2026-09-17 令「立即做一下」（承接 README 发布前接入走查之议）
> 范式：solo 单线，agent 亲写零子代理
> 前置：readme-plain 批已收约（c2dee6d 已推双远端）；本批修其接入节与实态不符处
> stem 认领：readme-adopt，甲表三件即 zh 门面接入实测修订、code 无承、派生 readme:new 加 adopt:new

## 一、问题陈述 {#problem}

README 接入走查（/tmp/sih-adopt-test 实测）暴露四处不符：
- 接入第一步 `sih init` 在新域不可达：预检要 active 标识牌行，而签发闸 2026-09-14 起严拒牌先于域（sim-dev 病灶收口），两条路互锁；现行唯一活入口是管理台 /tokens/open 开域全链（签发加开域加镜像）。
- 「全程无后台服务、无监听端口、无访问令牌」表述错：开域须一次性起 8765 管理台；标识牌是登记册行（防呆锚点非安全凭据，可如实表述）。
- stdio 配置块缺 SIH_MCPLINE_CODE_ROOT 字段（正典形在 adoption-guide-v1.md 第三节，引擎与项目分离的典型接入必需）。
- 开域链不写 .git/info/exclude（引擎缺陷，见泊件），README 承诺的隔离保证当前需手动补一行。

## 二、关键设计 {#design}

- 接入节改两段形：开域一次（管理台两步确认全链，含两个注意即工作区位坑与 exclude 手动补行）加日常 stdio。
- MCP 配置块补 SIH_MCPLINE_CODE_ROOT 加释义行。
- 指南旧序列与现行闸不符处，README 内如实标注以本页为准。
- 两缺陷入泊（trail 停泊笔）：开域链 exclude 缺失（引擎位候修）、指南第二节序列锁死（文档位候裁）；PARKING-v1.md 索引同步候后续批（doc/ 域写须走化格核阅检词三步，本批不扩面）。

## 三、工作清单 {#work}

- [ ] ra-01：README 接入节两段形重写与 MCP 块补字段
- [ ] ra-02：检词与指纹自检零违例
- [ ] ra-03：两缺陷停泊笔上链（pk-105 加 pk-106）
- [ ] ra-04：settle commit + close + 归并 + 推双远端

## 四、验收 {#acceptance}

- 接入节每一步与 /tmp/sih-adopt-test 实测判词对得上；零 emoji 零营销词；检词零违例；停泊笔两件在链；远端快进。
