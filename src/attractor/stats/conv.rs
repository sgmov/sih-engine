//! 收敛推断族（融回自围堰 facet_stats_conv.py，Path B）。
//!
//! 二函数：permutation_test_shared_convergence（跨厂共享真收敛命题数的置换
//! 检验）与 mixed_effects_anova（双因素方差分析含交互）。
//!
//! 置换检验显式 seed 承可复现性：同 seed 双跑逐字节一致。RNG 为 Rust 侧
//! 确定性实现（splitmix64 种子化的无放回均匀抽样），零假设重排语义与
//! 加一校正 p = (命中数 + 1) / (n_perm + 1) 与围堰同构；围堰 MT19937 序列
//! 不复刻——金向量判据取统计结论等值（SPEC-014 腿切分清单边界声明显式
//! 范畴排除），非 p 值逐位。

use serde_json::{json, Value};

use super::super::jsonc::{verr, PyError};
use super::inf::lgamma;

/// 确定性 RNG：splitmix64（显式 seed，无外部状态）。
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// [0, n) 均匀整数（无偏拒绝采样）。
    fn below(&mut self, n: usize) -> usize {
        let bound = n as u64;
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let r = self.next();
            if r >= threshold {
                return (r % bound) as usize;
            }
        }
    }
    /// 无放回均匀抽 size 个（Fisher-Yates 部分洗牌）。
    fn sample(&mut self, population: &[String], size: usize) -> Vec<String> {
        let mut pool: Vec<String> = population.to_vec();
        for i in 0..size {
            let j = i + self.below(pool.len() - i);
            pool.swap(i, j);
        }
        pool[..size].to_vec()
    }
}

/// 跨厂共享真收敛命题数的置换检验 p 值。
///
/// 统计量 = 所有厂真收敛命题集的交集大小。零假设下每厂标记在全域上随机
/// 重排（保持容量，无放回均匀抽样），p 值 = 重排下交集 >= 观测交集的比例，
/// 加一校正。
pub fn permutation_test_shared_convergence(
    factory_verdicts: &[(String, Vec<String>)],
    universe: Option<&[String]>,
    n_perm: usize,
    seed: u64,
) -> Result<f64, PyError> {
    if factory_verdicts.is_empty() {
        return Err(verr("factory_verdicts 须为非空 dict 即厂名到命题集合映射"));
    }
    if n_perm < 1 {
        return Err(verr("n_perm 须为正整数"));
    }
    let mut sets: Vec<(String, Vec<String>)> = Vec::new();
    for (name, props) in factory_verdicts {
        let mut prop_set: Vec<String> = props.clone();
        prop_set.sort();
        prop_set.dedup();
        if props.iter().any(|p| p.is_empty()) {
            return Err(verr(format!("厂 {} 的命题 id 须为字符串", name)));
        }
        sets.push((name.clone(), prop_set));
    }
    let pool: Vec<String> = match universe {
        None => {
            let mut union: Vec<String> = sets.iter().flat_map(|(_, s)| s.iter().cloned()).collect();
            union.sort();
            union.dedup();
            union
        }
        Some(u) => {
            let mut pool: Vec<String> = u.to_vec();
            pool.sort();
            pool.dedup();
            if pool.is_empty() {
                return Err(verr("universe 为空集合"));
            }
            pool
        }
    };
    // 观测交集
    let mut observed_set: Vec<String> = sets[0].1.clone();
    for (_, s) in sets.iter().skip(1) {
        observed_set.retain(|x| s.contains(x));
    }
    let observed = observed_set.len();
    if observed == 0 || pool.is_empty() {
        return Ok(1.0);
    }
    let sizes: Vec<usize> = sets.iter().map(|(_, s)| s.len()).collect();
    for size in &sizes {
        if *size > pool.len() {
            return Err(verr("厂集合容量超过全域大小"));
        }
    }
    let mut rng = SplitMix64(seed);
    let mut hits = 0usize;
    for _ in 0..n_perm {
        let mut inter: Option<Vec<String>> = None;
        for size in &sizes {
            let drawn = rng.sample(&pool, *size);
            inter = Some(match inter {
                None => drawn,
                Some(cur) => cur.into_iter().filter(|x| drawn.contains(x)).collect(),
            });
            if inter.as_ref().map(|i| i.is_empty()).unwrap_or(false) {
                break;
            }
        }
        let inter_len = inter.as_ref().map(|i| i.len()).unwrap_or(0);
        if inter_len >= observed {
            hits += 1;
        }
    }
    Ok((hits + 1) as f64 / (n_perm + 1) as f64)
}

/// F 分布上尾概率 P(F_{df1,df2} > f) = I_x(df2/2, df1/2)。
fn f_sf(f_value: f64, df1: f64, df2: f64) -> f64 {
    if f_value <= 0.0 {
        return 1.0;
    }
    let x = df2 / (df2 + df1 * f_value);
    reg_inc_beta(x, df2 / 2.0, df1 / 2.0)
}

/// 正则化不完全贝塔的连分数核（Lentz 算法）。
fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let max_iter = 200;
    let eps = 3e-12;
    let fpmin = 1e-300;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < fpmin {
        d = fpmin;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=max_iter {
        let m2 = 2 * m;
        let aa = m as f64 * (b - m as f64) * x / ((qam + m2 as f64) * (a + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < fpmin {
            d = fpmin;
        }
        c = 1.0 + aa / c;
        if c.abs() < fpmin {
            c = fpmin;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m as f64) * (qab + m as f64) * x / ((a + m2 as f64) * (qap + m2 as f64));
        d = 1.0 + aa * d;
        if d.abs() < fpmin {
            d = fpmin;
        }
        c = 1.0 + aa / c;
        if c.abs() < fpmin {
            c = fpmin;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < eps {
            break;
        }
    }
    h
}

/// 正则化不完全贝塔函数 I_x(a, b)，对称变换保数值稳定。
fn reg_inc_beta(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let ln_front = lgamma(a + b) - lgamma(a) - lgamma(b)
        + a * x.ln()
        + b * (1.0 - x).ln();
    let front = ln_front.exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * betacf(a, b, x) / a
    } else {
        1.0 - front * betacf(b, a, 1.0 - x) / b
    }
}

/// 双因素方差分析（含交互作用），平衡设计经典分解。
///
/// data：单元格键 (因素A水平, 因素B水平) 到重复观测序列。平衡设计要求每格
/// 重复数相同且不少于二，至少两行两列，全因子。返回 grand_mean、n_obs、
/// factors 三键，每因素与 interaction 各含 ss、df、ms、f、p。
pub fn mixed_effects_anova(
    data: &[((String, String), Vec<f64>)],
    factor_names: (&str, &str),
) -> Result<Value, PyError> {
    if data.is_empty() {
        return Err(verr("data 须为非空 dict，factor_names 须为二元组"));
    }
    let mut cells: Vec<((String, String), Vec<f64>)> = Vec::new();
    let mut rep_counts: Vec<usize> = Vec::new();
    for (key, values) in data {
        if values.len() < 2 {
            return Err(verr(format!("单元格 {:?} 须为至少两个数值的重复观测", key)));
        }
        cells.push((key.clone(), values.clone()));
        rep_counts.push(values.len());
    }
    if rep_counts.iter().any(|r| *r != rep_counts[0]) {
        return Err(verr("平衡设计要求各单元格重复数相同"));
    }
    let r = rep_counts[0];

    let mut levels_a: Vec<String> = cells.iter().map(|((a, _), _)| a.clone()).collect();
    levels_a.sort();
    levels_a.dedup();
    let mut levels_b: Vec<String> = cells.iter().map(|((_, b), _)| b.clone()).collect();
    levels_b.sort();
    levels_b.dedup();
    let a_n = levels_a.len();
    let b_n = levels_b.len();
    if a_n < 2 || b_n < 2 {
        return Err(verr("两因素各须至少两个水平"));
    }
    for i in &levels_a {
        for j in &levels_b {
            if !cells.iter().any(|((ci, cj), _)| ci == i && cj == j) {
                return Err(verr("设计须为全因子即每水平组合都有单元格"));
            }
        }
    }

    let cell_mean = |i: &str, j: &str| -> f64 {
        let s: Vec<f64> = cells
            .iter()
            .filter(|((ci, cj), _)| ci == i && cj == j)
            .flat_map(|(_, v)| v.iter().cloned())
            .collect();
        s.iter().sum::<f64>() / s.len() as f64
    };

    let all_vals: Vec<f64> = cells.iter().flat_map(|(_, v)| v.iter().cloned()).collect();
    let grand_mean = all_vals.iter().sum::<f64>() / all_vals.len() as f64;

    let row_mean = |i: &str| -> f64 {
        levels_b.iter().map(|j| cell_mean(i, j)).sum::<f64>() / b_n as f64
    };
    let col_mean = |j: &str| -> f64 {
        levels_a.iter().map(|i| cell_mean(i, j)).sum::<f64>() / a_n as f64
    };

    let ss_a = (b_n * r) as f64
        * levels_a.iter().map(|i| (row_mean(i) - grand_mean).powi(2)).sum::<f64>();
    let ss_b = (a_n * r) as f64
        * levels_b.iter().map(|j| (col_mean(j) - grand_mean).powi(2)).sum::<f64>();
    let mut ss_ab = 0.0f64;
    for i in &levels_a {
        for j in &levels_b {
            ss_ab += (cell_mean(i, j) - row_mean(i) - col_mean(j) + grand_mean).powi(2);
        }
    }
    ss_ab *= r as f64;
    let mut ss_err = 0.0f64;
    for ((i, j), series) in &cells {
        let cm = cell_mean(i, j);
        for v in series {
            ss_err += (v - cm).powi(2);
        }
    }

    let df_a = (a_n - 1) as f64;
    let df_b = (b_n - 1) as f64;
    let df_ab = ((a_n - 1) * (b_n - 1)) as f64;
    let df_err = (a_n * b_n * (r - 1)) as f64;
    let ms_a = ss_a / df_a;
    let ms_b = ss_b / df_b;
    let ms_ab = ss_ab / df_ab;
    let ms_err = if df_err > 0.0 { ss_err / df_err } else { 0.0 };

    let block = |ss: f64, ms: f64, df_factor: f64| -> Value {
        let f_value = if ms_err == 0.0 {
            if ss == 0.0 { 0.0 } else { f64::INFINITY }
        } else {
            ms / ms_err
        };
        let mut p_value = if !f_value.is_finite() || f_value <= 0.0 {
            1.0
        } else {
            f_sf(f_value, df_factor, df_err)
        };
        if f_value.is_infinite() {
            p_value = 0.0;
        }
        json!({"ss": ss, "df": df_factor, "ms": ms, "f": f_value, "p": p_value})
    };

    Ok(json!({
        "grand_mean": grand_mean,
        "n_obs": all_vals.len(),
        "factors": {
            factor_names.0: block(ss_a, ms_a, df_a),
            factor_names.1: block(ss_b, ms_b, df_b),
            "interaction": block(ss_ab, ms_ab, df_ab),
        },
    }))
}
