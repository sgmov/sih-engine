//! 距离度量族（融回自围堰 facet_stats_metrics.py，Path B）。
//!
//! 四函数：hellinger_distance、tv_distance、js_divergence、generalized_jaccard。
//! 纯数值实现：零网络、零写路径、无副作用，相同输入恒同输出。输入约定：
//! 等长非负序列，内部归一化到和为一再计算（jaccard 不归一化）；空序列、
//! 长度不等、含负数、全零归一化基线均报错。

use serde_json::Value;

use super::super::jsonc::{fsum, verr, PyError};

fn validate_vector(values: &[Value], name: &str) -> Result<Vec<f64>, PyError> {
    if values.is_empty() {
        return Err(verr(format!("{}: 输入序列为空，至少需要一个分量", name)));
    }
    let mut checked = Vec::with_capacity(values.len());
    for (index, item) in values.iter().enumerate() {
        let v = item.as_f64().ok_or_else(|| {
            verr(format!(
                "{}[{}]: 元素类型 {} 不是实数",
                name,
                index,
                py_type(item)
            ))
        })?;
        if v < 0.0 {
            return Err(verr(format!("{}[{}]: 元素为负数 {}", name, index, v)));
        }
        checked.push(v);
    }
    Ok(checked)
}

fn py_type(v: &Value) -> &'static str {
    match v {
        Value::String(_) => "str",
        Value::Bool(_) => "bool",
        Value::Number(_) => "float",
        Value::Null => "NoneType",
        _ => "object",
    }
}

fn normalize(vector: &[f64], name: &str) -> Result<Vec<f64>, PyError> {
    let total = fsum(vector);
    if total <= 0.0 {
        return Err(verr(format!("{}: 全零向量无法归一化为概率分布", name)));
    }
    Ok(vector.iter().map(|x| x / total).collect())
}

fn validated_pair(p: &[Value], q: &[Value]) -> Result<(Vec<f64>, Vec<f64>), PyError> {
    let vp = validate_vector(p, "p")?;
    let vq = validate_vector(q, "q")?;
    if vp.len() != vq.len() {
        return Err(verr(format!(
            "p 与 q 长度不等：len(p)={} 对 len(q)={}",
            vp.len(),
            vq.len()
        )));
    }
    Ok((vp, vq))
}

/// Hellinger 距离，值域 [0, 1]。
pub fn hellinger_distance(p: &[Value], q: &[Value]) -> Result<f64, PyError> {
    let (vp, vq) = validated_pair(p, q)?;
    let pn = normalize(&vp, "p")?;
    let qn = normalize(&vq, "q")?;
    let terms: Vec<f64> = pn
        .iter()
        .zip(qn.iter())
        .map(|(a, b)| (a.sqrt() - b.sqrt()).powi(2))
        .collect();
    Ok(fsum(&terms).sqrt() / 2.0f64.sqrt())
}

/// total variation 总变差距离，值域 [0, 1]。
pub fn tv_distance(p: &[Value], q: &[Value]) -> Result<f64, PyError> {
    let (vp, vq) = validated_pair(p, q)?;
    let pn = normalize(&vp, "p")?;
    let qn = normalize(&vq, "q")?;
    let terms: Vec<f64> = pn.iter().zip(qn.iter()).map(|(a, b)| (a - b).abs()).collect();
    Ok(0.5 * fsum(&terms))
}

/// Jensen-Shannon 散度（nats），值域 [0, ln 2]；零概率约定 0·ln 0 = 0。
pub fn js_divergence(p: &[Value], q: &[Value]) -> Result<f64, PyError> {
    let (vp, vq) = validated_pair(p, q)?;
    let pn = normalize(&vp, "p")?;
    let qn = normalize(&vq, "q")?;
    let mut total = 0.0f64;
    for (a, b) in pn.iter().zip(qn.iter()) {
        let m = 0.5 * (a + b);
        if *a > 0.0 {
            total += 0.5 * a * (a / m).ln();
        }
        if *b > 0.0 {
            total += 0.5 * b * (b / m).ln();
        }
    }
    Ok(total)
}

/// Min-Max 广义 Jaccard 相似度，值域 [0, 1]；两向量全零无定义报错。
pub fn generalized_jaccard(p: &[Value], q: &[Value]) -> Result<f64, PyError> {
    let (va, vb) = validated_pair(p, q)?;
    let denominator = fsum(&va.iter().zip(vb.iter()).map(|(x, y)| x.max(*y)).collect::<Vec<_>>());
    if denominator == 0.0 {
        return Err(verr("p 与 q 全零，Σmax = 0，广义 Jaccard 无定义"));
    }
    let numerator = fsum(&va.iter().zip(vb.iter()).map(|(x, y)| x.min(*y)).collect::<Vec<_>>());
    Ok(numerator / denominator)
}
