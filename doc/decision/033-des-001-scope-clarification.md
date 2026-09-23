# DEC-033 DES-001 适用界澄清

本决策澄清核阅规则包 des-001 的适用界，划定治理行文与入口宣传与风格表达与数据载体四类边界。背景即评估报告文档维度 W1 与 W2 两弱项：README.md 实测 13 处破折号与 21 处表格行与 1 处水平分割线，RANTS.md 实测 17 处破折号加 emoji 加粗体加块引用，两件是否受 DES-001 约束未在治理文档明示；auditfix 与 rulenorm 两批已完成主力文档族归一实践，余量集中于入口件与风格件与数据件。判词：格式规范管治理行文，不管入口宣传的排版取用，不管吐槽体的风格表达，不管数据件的载体格式；界内从严，界外豁免须显式落痕，禁默契豁免。

## 概览 {#overview}

- DES-001 全约束面即 AGENTS.md 与 doc/ 目录族六型加 guide::[全约束面](#full-scope)
- README.md 作入口宣传件享三例外，破折号与 emoji 照禁::[README 例外](#readme-exception)
- RANTS.md 不归受辖，frontmatter 显式声明豁免::[RANTS 豁免](#rants-exempt)
- .json 与 .ndjson 数据件不入行文约束，承 DEC-035 域界条款与域外实证::[数据件界](#data-scope)

## 全约束面 {#full-scope}

约束面
: DES-001 全约束面即 AGENTS.md 与 doc/ 目录族与 guide。doc/ 目录族六型即 proposal 与 design 与 spec 与 decision 与 governance 与 knowledge，对应 PRO 与 DES 与 SPEC 与 DEC 与 GOV 与 KNOW 六型文档。域内目标受核阅试判，退出码三值照章。

## README 例外 {#readme-exception}

README.md 定位
: README.md 是仓入口宣传件，受众是仓外访客，排版取用自由度高于治理文档。裁定：README.md 属 DES-001 行文约束对象，享三例外即行内代码与围栏代码块与必要表格，此三形不受对应禁令约束。例外是减项不是全免：破折号与 emoji 照禁，例外不及于字符禁令。

## RANTS 豁免 {#rants-exempt}

RANTS.md 定位
: RANTS.md 是吐槽体风格件，风格表达即其内容本体，格式规训毁表达。裁定：RANTS.md 不归 DES-001 受辖。豁免声明以文件首 frontmatter 承载，标注 style: exempt-from-DES-001，豁免落痕在件首，见声明即知界外。声明缺席即视同受辖，冲突照违例计。

## 数据件界 {#data-scope}

数据件
: .json 与 .ndjson 数据件不入 DES-001 行文约束。承 DEC-035 域界条款：字符与排版与结构禁令只及 .md 行文材料，数据件格式归各数据规格承载。域外实证在案：des-001 包域声明 include 族全为 .md 通配，.json 与 .ndjson 不在域内，scrutinator 对域外目标判 exit 2，如实记入档，不属违规。

## 关联 {#relation}

元规范即 doc/design/DES-001-document-format/design.md；域界先例即 DEC-035 即 doc/decision/035-doc-retired-directory-policy.md；冲突实证即评估报告 doc/assessment/2026-09-22-sih-engine-evaluation-report.md 文档维度 W1 与 W2；归一实践即 auditfix 与 rulenorm 两批；受试工具即 scrutinator des-001 包；豁免样板即仓根 RANTS.md 文件首 frontmatter。
