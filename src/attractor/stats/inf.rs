//! 推断校正族（融回自围堰 facet_stats_inf.py，Path B）。
//!
//! 五函数：binomial_test_pvalue、boundary_test_pvalue、bonferroni_correction、
//! bh_fdr、statistical_power。精确法不用正态近似：二项检验对概率质量逐项
//! 精确求和，组合数经 lgamma 在对数空间计算后逐项取指数，求和用精确和；
//! Bonferroni 与 Benjamini-Hochberg 为有限步精确代数运算；统计功效在精确
//! 二项分布上确定临界值。
//!
//! 数值域：概率返回值落 [0, 1]；真实 p 值低于双精度下溢阈返回 0.0，浮点和
//! 越过 1.0 封顶 1.0（双精度表示边界行为，非算法近似）。

use serde_json::Value;

use super::super::jsonc::{fsum, verr, PyError};

fn require_counts(k: i64, n: i64) -> Result<(), PyError> {
    if n < 0 {
        return Err(verr(format!("n 须非负，实际为 {}", n)));
    }
    if k < 0 || k > n {
        return Err(verr(format!("须 0 <= k <= n，实际 k = {}, n = {}", k, n)));
    }
    Ok(())
}

fn require_probability(value: f64, name: &str, open_interval: bool) -> Result<f64, PyError> {
    if open_interval {
        if !(0.0 < value && value < 1.0) {
            return Err(verr(format!("{} 须落在开区间 (0, 1)，实际为 {}", name, value)));
        }
    } else if !(0.0 <= value && value <= 1.0) {
        return Err(verr(format!("{} 须落在闭区间 [0, 1]，实际为 {}", name, value)));
    }
    Ok(value)
}

fn require_p_values(p_values: &[Value]) -> Result<Vec<f64>, PyError> {
    if p_values.is_empty() {
        return Err(verr("p_values 须非空"));
    }
    let mut out = Vec::with_capacity(p_values.len());
    for (i, item) in p_values.iter().enumerate() {
        let v = item.as_f64().ok_or_else(|| verr(format!("p_values[{}] 须为实数", i)))?;
        out.push(require_probability(v, &format!("p_values[{}]", i), false)?);
    }
    Ok(out)
}

/// 对数空间组合数：log C(n, j) = lgamma(n+1) - lgamma(j+1) - lgamma(n-j+1)。
fn log_binom(n: i64, j: i64) -> f64 {
    lgamma((n + 1) as f64) - lgamma((j + 1) as f64) - lgamma((n - j + 1) as f64)
}

/// 精确上尾 P(X >= k)，X 服从二项分布 (n, p)。
fn binom_upper_tail(k: i64, n: i64, p: f64) -> f64 {
    if k == 0 {
        return 1.0;
    }
    let log_p = p.ln();
    let log_q = (1.0 - p).ln();
    let terms: Vec<f64> = (k..=n)
        .map(|j| (log_binom(n, j) + j as f64 * log_p + (n - j) as f64 * log_q).exp())
        .collect();
    fsum(&terms).min(1.0)
}

/// 单侧精确二项检验的上尾 p 值。
pub fn binomial_test_pvalue(k: i64, n: i64, p_h0: f64) -> Result<f64, PyError> {
    require_counts(k, n)?;
    let p = require_probability(p_h0, "p_h0", true)?;
    Ok(binom_upper_tail(k, n, p))
}

/// boundary 计数相对厂基线翻桌率的单侧精确二项检验上尾 p 值（p_h0 须显式
/// 传入，防调用方以 0.5 充当基线）。
pub fn boundary_test_pvalue(k: i64, n: i64, p_h0: f64) -> Result<f64, PyError> {
    require_counts(k, n)?;
    let p = require_probability(p_h0, "p_h0", true)?;
    Ok(binom_upper_tail(k, n, p))
}

/// Bonferroni 多重比较校正：p_i' = min(p_i * m, 1.0)。
pub fn bonferroni_correction(p_values: &[Value], alpha: f64) -> Result<Vec<f64>, PyError> {
    let ps = require_p_values(p_values)?;
    require_probability(alpha, "alpha", true)?;
    let m = ps.len();
    Ok(ps.iter().map(|p| (p * m as f64).min(1.0)).collect())
}

/// Benjamini-Hochberg 步升法 FDR 校正调整 p 值，步升单调化后按原序返回。
pub fn bh_fdr(p_values: &[Value], alpha: f64) -> Result<Vec<f64>, PyError> {
    let ps = require_p_values(p_values)?;
    require_probability(alpha, "alpha", true)?;
    let m = ps.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|a, b| ps[*a].partial_cmp(&ps[*b]).unwrap_or(std::cmp::Ordering::Equal));
    let mut adjusted = vec![0.0f64; m];
    let mut running_min = 1.0f64;
    for rank in (1..=m).rev() {
        let idx = order[rank - 1];
        let candidate = (m as f64 / rank as f64) * ps[idx];
        if candidate < running_min {
            running_min = candidate;
        }
        adjusted[idx] = running_min.min(1.0);
    }
    Ok(adjusted)
}

/// 单侧精确二项检验的统计功效：临界值 k* 为 H0 上尾不超过水平的最大 k，
/// 功效为 H1 下落入拒绝域的概率。
pub fn statistical_power(n: i64, p_h0: f64, p_h1: f64, alpha: f64) -> Result<f64, PyError> {
    if n < 1 {
        return Err(verr(format!("n 须为正整数，实际为 {}", n)));
    }
    let p0 = require_probability(p_h0, "p_h0", true)?;
    let p1 = require_probability(p_h1, "p_h1", true)?;
    let a = require_probability(alpha, "alpha", true)?;

    let log_p0 = p0.ln();
    let log_q0 = (1.0 - p0).ln();
    let terms0: Vec<f64> = (0..=n)
        .map(|j| (log_binom(n, j) + j as f64 * log_p0 + (n - j) as f64 * log_q0).exp())
        .collect();

    let mut k_star = n + 1;
    let mut suffix = 0.0f64;
    for k in (0..=n).rev() {
        suffix += terms0[k as usize];
        if suffix <= a {
            k_star = k;
        } else {
            break;
        }
    }
    if k_star == n + 1 {
        return Ok(0.0);
    }

    let log_p1 = p1.ln();
    let log_q1 = (1.0 - p1).ln();
    let terms1: Vec<f64> = (k_star..=n)
        .map(|j| (log_binom(n, j) + j as f64 * log_p1 + (n - j) as f64 * log_q1).exp())
        .collect();
    Ok(fsum(&terms1).min(1.0))
}

/// lgamma：Lanczos 逼近（g=7, n=9），x < 0.5 走反射式。
pub fn lgamma(x: f64) -> f64 {
    const LANCZOS_G: f64 = 7.0;
    const COEF: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    if x < 0.5 {
        // 反射式：Γ(x)Γ(1-x) = π / sin(πx)
        let pi = std::f64::consts::PI;
        (pi / (pi * x).sin()).abs().ln() - lgamma(1.0 - x)
    } else {
        let x = x - 1.0;
        let mut a = COEF[0];
        let t = x + LANCZOS_G + 0.5;
        for (i, c) in COEF.iter().enumerate().skip(1) {
            a += c / (x + i as f64);
        }
        0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
    }
}
