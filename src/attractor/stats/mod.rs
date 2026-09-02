//! facet 严格统计量库门面（融回自围堰 facet_stats.py，Path B）。
//!
//! 覆盖 FACET-MATHEMATICAL-FOUNDATION.md 第三节所列十一函数，内部拆三私有
//! 模块：metrics 距离度量族、inf 推断校正族、conv 收敛推断族。本门面
//! re-export 十一函数全名为唯一公共入口，纯数值实现：零 LLM、零网络、零
//! 写路径。

pub mod conv;
pub mod inf;
pub mod metrics;

pub use conv::{mixed_effects_anova, permutation_test_shared_convergence};
pub use inf::{
    bh_fdr, binomial_test_pvalue, bonferroni_correction, boundary_test_pvalue, statistical_power,
};
pub use metrics::{generalized_jaccard, hellinger_distance, js_divergence, tv_distance};
