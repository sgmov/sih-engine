# 任务包 001 修订 DES-005 v2 删编造 commit hash

## 任务目标

修订 `sih-engine/doc/design/DES-005-semantic-verification-calculus.md` 第 210-211 行,删除编造的 commit 8a7f2d3 引用。L210 原文「v1 全文保留在 git 历史 commit 8a7f2d3」是 Mavis 起草时未查 git 编造,L211 git show 命令也是无效引用。修订后 §十一 v1 归档段只承载 v1 与 v2 的差异,不再编造具体 commit 引用。

## 项目背景

DES-005 v2 是 N 等于 4 隔离采样范式到偏离率收敛范式的实质性重写。v1 全文归旧仓 `sihankor/doc/draft/...` 不在本仓可触达位置。子代理修订时不要尝试 git show,直接删除 commit 引用。

## 任务构成规则

### self-check(六项核心)

1. 主动判断产出性质 — 这是文档修订,不是新建
2. 服务原始意图 — 删编造内容,不加新内容
3. 范畴排除显式 — 只删 L210-211 段,不展开 §十一 其它段
4. 不逃避当下责任 — 删 L210 触发 doclint 仍需 exit 0
5. 每行去掉会犯错吗 — 修订后 §十一 还要承载 v1 归档的「不可继承部分」语义
6. 与哲学仓相容 — 修订不引入新哲学命题

### 通用格式规范

遵守 `sih-engine/doc/design/DES-001-document-format/` 规范。修订后跑 `sihankor/tools/doclint/target/release/sih-doclint` 必须 exit 0。

### 类型特异化约束(五条)

1. 结构特异非字符集 — 修订属结构变更不是字符
2. 特异化需支撑 — 删 L210 触发自检段「内容自检」删「参数留空不拍脑袋」相关条目
3. 不照抄通用 — 不复用 v1 草稿,直接重写
4. 不重复定义 — 不重新定义 §十一 v1 归档结构
5. 最小化原则 — 只动 L210-211,不动其它

### 哲学检索

本修订不涉及哲学命题变化,跳过哲学检索。

## 参考文件(最小必需集)

- `sih-engine/doc/design/DES-005-semantic-verification-calculus.md`
- `sihankor/tools/doclint/target/release/sih-doclint`

## 交付要求

修订后 L210-211 段消失,doclint exit 0,内容自检段删「v1 全文保留在 git 历史 commit 8a7f2d3」引用相关条目。最小化三问:动必要?是,删错内容。动充分?是,只动这两行。动最小?是,不动其它。
