//! 温故语义通道引擎件 —— retrieverline 批（rl-03）自工具壳 wikirecall semantic.py 对表移植。
//! 正典指针：retrieverline 批立项包（sih-engine/sih/state/plan/retrieverline.md）；
//! 算法源：sih-tools/wikirecall/semantic.py（pk037impl 实装，pk050sw 批切缺省位）。
//!
//! 算法承 PROB-017 逆文档频率与自信息加权（idf=log2(N/df) 即词现于一篇文档这一事件
//! 在经验文档分布下的自信息，df=N 时 idf=0 即零鉴别力退化，承 PROB-014 零增益判据）
//! 与 ALG-011 向量空间与余弦相似度（实内积空间 Cauchy-Schwarz 界给余弦落 [-1,1]）。
//! 既裁路线：确定性统计向量，零模型零外部依赖零 LLM，语料规模内全量暴力计算不引
//! 近似最近邻。确定性纪律对表 Python 版：遍历恒按排序序列（BTreeMap 序同 Python
//! sorted），浮点累加按固定次序，序决胜为得分降序加条目 id 升序；同输入全中间量可复现。
//!
//! 引擎侧语料面申报（对 semantic.py _hay_of 的偏差如实申报）：wikirecall 条目带
//! title 与 triggers 两面即 hay=title+换行+触发问句；引擎侧 locator 条目无此两面，
//! hay 取条目文本面 entry.text——与词面通道（文轴逐字子串扫）同一文本面，通道对价
//! 公平承原口径。分词与加权与相似度度量三件与 Python 版逐算法对表，零偏差。

/// 缺省语义通道前 K 正分命中（承 pk050sw 批 DEFAULT_SEMANTIC_K=3 一次性登记值）。
pub const DEFAULT_SEMANTIC_K: usize = 3;

use std::collections::BTreeMap;

use regex::Regex;

/// 词元化：小写化后拉丁词元取连续 [a-z0-9]+ 串，中日韩文取连续汉字重叠二字组，
/// 落单汉字尾则单字成元，其余字符为分隔（semantic.py tokenize 对表，序保持）。
pub fn tokenize(text: &str) -> Vec<String> {
    let lowered = text.to_lowercase();
    let latin = Regex::new(r"[a-z0-9]+").expect("拉丁词元正则恒合法");
    let cjk = Regex::new(r"[\u{4e00}-\u{9fff}]+").expect("汉字段正则恒合法");
    let mut tokens = Vec::new();
    for m in latin.find_iter(&lowered) {
        tokens.push(m.as_str().to_string());
    }
    for m in cjk.find_iter(&lowered) {
        let run: Vec<char> = m.as_str().chars().collect();
        for i in 0..run.len().saturating_sub(1) {
            tokens.push(run[i..=i + 1].iter().collect());
        }
        if run.len() % 2 == 1 {
            tokens.push(run[run.len() - 1..].iter().collect());
        }
    }
    tokens
}

/// 词元计数，按词元排序返回列表，累加序确定（semantic.py term_counts 对表）。
pub fn term_counts(text: &str) -> Vec<(String, u32)> {
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    for tok in tokenize(text) {
        *counts.entry(tok).or_insert(0) += 1;
    }
    counts.into_iter().collect()
}

/// 语料统计：N 与各文档词元计数与 df 表。纯函数，遍历全排序（build_stats 对表）。
pub struct CorpusStats {
    pub n_docs: usize,
    pub docs: BTreeMap<String, BTreeMap<String, u32>>,
    pub df: BTreeMap<String, usize>,
}

/// 文档集统计构建：键序即 BTreeMap 序（同 Python sorted(index)），df 逐文档累加。
pub fn build_stats(docs: &BTreeMap<String, String>) -> CorpusStats {
    let mut stats = CorpusStats {
        n_docs: docs.len(),
        docs: BTreeMap::new(),
        df: BTreeMap::new(),
    };
    for (key, hay) in docs {
        let tc = term_counts(hay);
        let mut per_doc: BTreeMap<String, u32> = BTreeMap::new();
        for (tok, cnt) in tc {
            *stats.df.entry(tok.clone()).or_insert(0) += 1;
            per_doc.insert(tok, cnt);
        }
        stats.docs.insert(key.clone(), per_doc);
    }
    stats
}

/// 逆文档频率 log2(N/df)，df=0 或 df>=N 即 0.0（零鉴别力退化承 PROB-017）。
pub fn idf(stats: &CorpusStats, tok: &str) -> f64 {
    let n = stats.n_docs;
    let d = stats.df.get(tok).copied().unwrap_or(0);
    if d == 0 || d >= n {
        return 0.0;
    }
    (n as f64 / d as f64).log2()
}

/// 查询向量：q(t)=tf(t)*idf(t)，语料外词元无 idf 即忽略，按词元排序。
pub fn query_vector(stats: &CorpusStats, text: &str) -> BTreeMap<String, f64> {
    let mut vec = BTreeMap::new();
    for (tok, cnt) in term_counts(text) {
        let w = idf(stats, &tok);
        if w > 0.0 {
            vec.insert(tok, cnt as f64 * w);
        }
    }
    vec
}

/// 文档向量：w(t)=tf(t)*idf(t)，按词元排序构造。
pub fn doc_vector(stats: &CorpusStats, key: &str) -> BTreeMap<String, f64> {
    let mut vec = BTreeMap::new();
    if let Some(counts) = stats.docs.get(key) {
        for (tok, cnt) in counts {
            let w = idf(stats, tok);
            if w > 0.0 {
                vec.insert(tok.clone(), *cnt as f64 * w);
            }
        }
    }
    vec
}

/// 余弦相似度，累加按排序词元序固定；零向量即 0.0（Cauchy-Schwarz 退化侧）。
pub fn cosine(vec_q: &BTreeMap<String, f64>, vec_d: &BTreeMap<String, f64>) -> f64 {
    let mut dot = 0.0;
    for (tok, qv) in vec_q {
        if let Some(dv) = vec_d.get(tok) {
            dot += qv * dv;
        }
    }
    let nq: f64 = vec_q.values().map(|v| v * v).sum();
    let nd: f64 = vec_d.values().map(|v| v * v).sum();
    let (nq, nd) = (nq.sqrt(), nd.sqrt());
    if nq == 0.0 || nd == 0.0 {
        return 0.0;
    }
    dot / (nq * nd)
}

/// 语义通道：全量暴力余弦排序取前 K 正分命中。
///
/// 返回（条目键, 得分）序列：按（-score, key）升序的前 K 条，遍历与累加全排序，
/// 同输入逐字节同输出（semantic.py semantic_channel 对表）。
pub fn semantic_channel(
    docs: &BTreeMap<String, String>,
    terms: &[String],
    k: usize,
) -> Vec<(String, f64)> {
    let stats = build_stats(docs);
    let qtext = terms.join("\n");
    let qvec = query_vector(&stats, &qtext);
    let mut scored: Vec<(String, f64)> = Vec::new();
    for key in docs.keys() {
        let score = cosine(&qvec, &doc_vector(&stats, key));
        if score > 0.0 {
            scored.push((key.clone(), score));
        }
    }
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(&b.0))
    });
    scored.truncate(k);
    scored
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docs_fixture() -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "a".to_string(),
                "租约 锁 定 生命周期 治理 写入 按任务包".to_string(),
            ),
            (
                "b".to_string(),
                "租约 close 受 SDDG 两道门 约束 写入 治理".to_string(),
            ),
            (
                "c".to_string(),
                "句读 解析 PEG 空腹 条目投影 纯数据".to_string(),
            ),
            (
                "d".to_string(),
                "级联 上游 洁净 不变式 路径 即 id".to_string(),
            ),
        ])
    }

    /// 分词对表 semantic.py：拉丁与汉字二字组与奇数尾单字与混合串。
    /// 金值出 Python semantic.term_counts 实测（2026-09-18 双跑对表批内取值）。
    #[test]
    fn tokenize_matches_python() {
        assert_eq!(
            term_counts("Abc12 锁锁 lock-99"),
            vec![
                ("99".to_string(), 1),
                ("abc12".to_string(), 1),
                ("lock".to_string(), 1),
                ("锁锁".to_string(), 1),
            ]
        );
        assert_eq!(
            term_counts("中文中中文"),
            vec![
                ("中中".to_string(), 1),
                ("中文".to_string(), 2),
                ("文".to_string(), 1),
                ("文中".to_string(), 1),
            ]
        );
        assert_eq!(term_counts(" ABC abc"), vec![("abc".to_string(), 2)]);
    }

    /// 通道金值对表 Python semantic.semantic_channel 实测：查询「租约 锁」K=2
    /// 命中 [a, b]，得分逐位同 Python（log2 与累加序一致）。
    #[test]
    fn channel_golden_matches_python() {
        let docs = docs_fixture();
        let hits = semantic_channel(&docs, &["租约 锁".to_string()], 2);
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].0, "a");
        assert_eq!(hits[1].0, "b");
        assert_eq!(hits[0].1, 0.3779644730092272);
        assert_eq!(hits[1].1, 0.08032193289024989);
    }

    /// 全序金值对表：查询「治理 写入 解析」K=10 得全三正分序 c 先 b 后 a。
    #[test]
    fn channel_full_ordering_golden() {
        let docs = docs_fixture();
        let hits = semantic_channel(&docs, &["治理 写入 解析".to_string()], 10);
        let got: Vec<(String, f64)> = hits;
        assert_eq!(got.len(), 3);
        assert_eq!(got[0].0, "c");
        assert_eq!(got[1].0, "b");
        assert_eq!(got[2].0, "a");
        assert_eq!(got[0].1, 0.25819888974716115);
        assert_eq!(got[1].1, 0.1466471150213533);
        assert_eq!(got[2].1, 0.13801311186847084);
    }

    /// 零命中即零正分零出：查询与语料全无共词时通道空手（同 Python hits=[]）。
    #[test]
    fn channel_zero_overlap_empty() {
        let docs = docs_fixture();
        let hits = semantic_channel(&docs, &["量子 巧克力".to_string()], 3);
        assert!(hits.is_empty(), "实见 {hits:?}");
    }

    /// K 截断即 K 小于正分数时取前 K；确定性即同参双跑逐位一致。
    /// K=1 首名金值承 Python semantic.semantic_channel(docs, ['治理 写入'], 1) 实测。
    #[test]
    fn channel_k_cap_and_determinism() {
        let docs = docs_fixture();
        let a = semantic_channel(&docs, &["治理 写入".to_string()], 1);
        assert_eq!(a.len(), 1);
        assert_eq!(a[0].0, "b", "最高分文档 b 先出（Python 实测对表）");
        assert_eq!(a[0].1, 0.254000254000381);
        let b = semantic_channel(&docs, &["治理 写入".to_string()], 10);
        let c = semantic_channel(&docs, &["治理 写入".to_string()], 10);
        assert_eq!(a[0].1, b[0].1, "K 截断不改首名得分");
        assert_eq!(b, c, "同参双跑逐位一致");
    }

    /// 零鉴别力退化：词现于全语料即 idf=0 权重零出，查询独词全语料共有则零命中。
    #[test]
    fn idf_zero_when_doc_freq_eq_n() {
        let docs = docs_fixture();
        let stats = build_stats(&docs);
        // 「租约」现于 a 与 b 两文档，N=4，idf=log2(4/2)=1
        assert_eq!(idf(&stats, "租约"), 1.0);
        // 全语料 N=4 时构造同词全有语料查退化：单文档语料内任何词 df=N 即 0
        let one = BTreeMap::from([("x".to_string(), "独词".to_string())]);
        let stats_one = build_stats(&one);
        assert_eq!(idf(&stats_one, "独词"), 0.0);
        // 语料外词 df=0 即 0
        assert_eq!(idf(&stats, "语料外"), 0.0);
    }
}
