// 正典指针：SPEC-025 全量工具融回与插件槽位规格（sih-engine/doc/spec/SPEC-025-toolful-mergeback-v1.md）。
//! 引擎侧 LaTeX 书写辅助（latextool）命令行面 —— lease-mergeleg6-parallel 簇G 移植件。
//!
//! 选名申报：bin 名 latextool。围堰名 latex-helper 带连字且贴近 LaTeX 控制词
//! 命名空间（latex / tex 系宏名），融回 bin 名取词干拼接形 latextool 避让宏名
//! 冲突，并保引擎 bin 面无连字歧义。
//!
//! 行为面移植自围堰 sih-tools/latex-helper 0.1.0（src/latex_helper/：cli.py、
//! validate.py、autofix.py、blocks.py、suggest.py、knowledge.py），只读对表
//! 移植，围堰源码零改动。五子命令全行为链：validate（5 类错误检测：括号未
//! 闭合 / 缺失环境 / 引用未声明 / 必填参数缺失 / 环境不匹配，含半开区间豁免）、
//! autofix（未闭合括号行尾补全 + 缺失 \\end 末尾补全 + 复杂问题人工分类，check
//! / --write 双模式）、block-create（9 模板 + k=v 引号参数解析 + --list）、
//! suggest（84 词条本地知识库 + 127 条中英关键词映射，精确词含分评分去重排序）、
//! knowledge（知识库清单）。退出码三值：0 = 合规 / 成功 / 无错；1 = 有违规 /
//! 有错已修；2 = 工具自身异常。
//!
//! 落差申报：
//! 1. compute 子命令未移植：围堰基于 SymPy CAS（parse_latex / sympify），零新增
//!    Cargo 依赖约束下不可机械承载。bin 保留其参数面（位置形与 --operation /
//!    --expression 旗标形及 --var/--lower/--upper/--to/--order/--at/--n/--rows），
//!    缺参时报文与围堰对齐；参数齐备时调用即出 {"error": ...} 退出码 2，报文
//!    申报未移植。
//! 2. 行切分按 '\n' 承载；围堰 Python splitlines 另认 '\r'、'\u2028' 等分隔符，
//!    该边缘形未对齐（\r\n 文件行尾 '\r' 计入扫描，主判定形不受影响）。
//! 3. suggest --limit 负值按 0 出；围堰 Python 负切片语义（自尾裁剪）未对齐。
//! 4. knowledge 子命令 categories 列表序：围堰 Python set 迭代序未指定，本件按
//!    知识库词条首次出现序出。
//! 5. 用法错（未知子命令、未知旗标、坏 --format 值、位置参数个数不对）为自足
//!    解析，stderr 短句回报，退出码 2 与围堰 argparse 对齐，报文形不对齐。
//! 6. Python str.lower() 与 Rust to_lowercase() 的极端 Unicode 边缘形未对齐
//!    （suggest 查询小写化主形 ASCII 与 CJK 不受影响）。
//! 7. 围堰 validate._extract_latex_blocks 在 validate_text 消费路径为死码（提取
//!    结果未被读取），未移植，零行为差。

use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::process::exit;
use std::sync::OnceLock;
use regex::Regex;

/// 围堰版本锚：sih-tools/latex-helper/src/latex_helper/__init__.py __version__。
const VERSION: &str = "0.1.0";

const EXIT_OK: i32 = 0;
const EXIT_VIOLATION: i32 = 1;
const EXIT_ERROR: i32 = 2;

fn out_json(v: &Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap());
}

fn usage_fail(msg: &str) -> ! {
    eprintln!("用法错: {msg}");
    eprintln!("用法: latextool <subcommand> [参数]");
    eprintln!("子命令: suggest / block-create / validate / autofix / compute / knowledge");
    eprintln!("退出码三值: 0=合规/成功 1=有违规/有错已修 2=工具自身异常");
    exit(EXIT_ERROR);
}

// ============ CLI 参数自足解析（落差申报五：报文形不对齐，退出码对齐） ============

struct Parsed {
    positionals: Vec<String>,
    values: HashMap<&'static str, String>,
    bools: HashSet<&'static str>,
}

fn parse_args(
    args: &[String],
    value_flags: &[&'static str],
    bool_flags: &[&'static str],
) -> Result<Parsed, String> {
    let mut out = Parsed { positionals: Vec::new(), values: HashMap::new(), bools: HashSet::new() };
    let mut i = 0usize;
    let mut no_more_flags = false;
    while i < args.len() {
        let a = &args[i];
        if !no_more_flags && a == "--" {
            no_more_flags = true;
            i += 1;
            continue;
        }
        if !no_more_flags && a.starts_with("--") {
            let (name, inline): (String, Option<String>) = match a.split_once('=') {
                Some((n, v)) => (n.to_string(), Some(v.to_string())),
                None => (a.clone(), None),
            };
            let key = name.trim_start_matches("--").to_string();
            if let Some(f) = bool_flags.iter().find(|f| **f == key) {
                out.bools.insert(f);
                i += 1;
                continue;
            }
            if let Some(f) = value_flags.iter().find(|f| **f == key) {
                let val = match inline {
                    Some(v) => v,
                    None => {
                        i += 1;
                        if i >= args.len() {
                            return Err(format!("旗标 {name} 需要取值"));
                        }
                        args[i].clone()
                    }
                };
                out.values.insert(f, val);
                i += 1;
                continue;
            }
            return Err(format!("未知旗标: {name}"));
        }
        out.positionals.push(a.clone());
        i += 1;
    }
    Ok(out)
}

/// --format 取值校验（围堰 choices=["json","human"]，默认 json）。
fn get_format(p: &Parsed) -> String {
    match p.values.get("format") {
        Some(f) if f == "json" || f == "human" => f.clone(),
        Some(f) => usage_fail(&format!("--format 取值只能是 json 或 human，得到 {f}")),
        None => "json".to_string(),
    }
}

// ============ knowledge.py：本地知识库（84 词条逐字对表） ============

struct KEntry {
    key: &'static str,
    latex: &'static str,
    category: &'static str,
    description: &'static str,
    params: &'static [&'static str],
    optional: bool,
}

static KNOWLEDGE: &[KEntry] = &[
    // === 算子类 ===
    KEntry { key: "integral_definite", latex: r"\int_{a}^{b} f(x) \, dx", category: "operator", description: "定积分：从 a 到 b 对 f(x) 积分", params: &["lower", "upper", "integrand", "var"], optional: false },
    KEntry { key: "integral_indefinite", latex: r"\int f(x) \, dx", category: "operator", description: "不定积分：f(x) 的原函数", params: &["integrand", "var"], optional: false },
    KEntry { key: "integral_double", latex: r"\iint_{D} f(x,y) \, dA", category: "operator", description: "二重积分：区域 D 上 f(x,y) 的积分", params: &["region", "integrand"], optional: false },
    KEntry { key: "integral_triple", latex: r"\iiint_{\Omega} f(x,y,z) \, dV", category: "operator", description: "三重积分", params: &["region", "integrand"], optional: false },
    KEntry { key: "integral_line", latex: r"\int_{C} f \, ds", category: "operator", description: "线积分：沿曲线 C", params: &["path", "integrand"], optional: false },
    KEntry { key: "sum", latex: r"\sum_{i=1}^{n} a_i", category: "operator", description: "求和：i 从 1 到 n 的 a_i 之和", params: &["lower", "upper", "summand"], optional: false },
    KEntry { key: "sum_infinite", latex: r"\sum_{i=1}^{\infty} a_i", category: "operator", description: "无穷级数", params: &["lower", "summand"], optional: false },
    KEntry { key: "product", latex: r"\prod_{i=1}^{n} a_i", category: "operator", description: "连乘：i 从 1 到 n 的 a_i 之积", params: &["lower", "upper", "factor"], optional: false },
    KEntry { key: "limit", latex: r"\lim_{x \to a} f(x)", category: "operator", description: "极限：x 趋近于 a 时 f(x) 的极限", params: &["var", "to", "expr"], optional: false },
    KEntry { key: "limit_infinity", latex: r"\lim_{x \to \infty} f(x)", category: "operator", description: "无穷远处的极限", params: &["var", "expr"], optional: false },
    KEntry { key: "derivative", latex: r"\frac{d}{dx} f(x)", category: "operator", description: "导数：f(x) 对 x 的一阶导数", params: &["var", "expr"], optional: false },
    KEntry { key: "partial_derivative", latex: r"\frac{\partial f}{\partial x}", category: "operator", description: "偏导数：f 对 x 的偏导", params: &["expr", "var"], optional: false },
    KEntry { key: "nth_derivative", latex: r"\frac{d^{n} f}{dx^{n}}", category: "operator", description: "n 阶导数", params: &["order", "expr", "var"], optional: false },
    KEntry { key: "gradient", latex: r"\nabla f", category: "operator", description: "梯度：f 的梯度向量", params: &["expr"], optional: false },
    KEntry { key: "divergence", latex: r"\nabla \cdot \vec{F}", category: "operator", description: "散度：向量场 F 的散度", params: &["field"], optional: false },
    KEntry { key: "curl", latex: r"\nabla \times \vec{F}", category: "operator", description: "旋度：向量场 F 的旋度", params: &["field"], optional: false },
    // === 函数类 ===
    KEntry { key: "sin", latex: r"\sin(x)", category: "function", description: "正弦函数", params: &["arg"], optional: false },
    KEntry { key: "cos", latex: r"\cos(x)", category: "function", description: "余弦函数", params: &["arg"], optional: false },
    KEntry { key: "tan", latex: r"\tan(x)", category: "function", description: "正切函数", params: &["arg"], optional: false },
    KEntry { key: "log", latex: r"\log_{b}(x)", category: "function", description: "对数函数：底为 b", params: &["base", "arg"], optional: false },
    KEntry { key: "ln", latex: r"\ln(x)", category: "function", description: "自然对数", params: &["arg"], optional: false },
    KEntry { key: "exp", latex: r"e^{x}", category: "function", description: "指数函数（自然底）", params: &["exponent"], optional: false },
    KEntry { key: "sqrt", latex: r"\sqrt{x}", category: "function", description: "平方根", params: &["arg"], optional: false },
    KEntry { key: "nth_root", latex: r"\sqrt[n]{x}", category: "function", description: "n 次方根", params: &["index", "arg"], optional: false },
    KEntry { key: "abs", latex: r"|x|", category: "function", description: "绝对值", params: &["arg"], optional: false },
    KEntry { key: "floor", latex: r"\lfloor x \rfloor", category: "function", description: "下取整函数", params: &["arg"], optional: false },
    KEntry { key: "ceil", latex: r"\lceil x \rceil", category: "function", description: "上取整函数", params: &["arg"], optional: false },
    // === 结构类 ===
    KEntry { key: "frac", latex: r"\frac{a}{b}", category: "structure", description: "分数：a 除以 b", params: &["num", "den"], optional: false },
    KEntry { key: "binom", latex: r"\binom{n}{k}", category: "structure", description: "二项式系数：n 选 k", params: &["n", "k"], optional: false },
    KEntry { key: "supsub", latex: r"x_{i}^{2}", category: "structure", description: "上下标：x 的下标 i 上标 2", params: &["base", "sub", "sup"], optional: true },
    KEntry { key: "overset", latex: r"\overset{a}{X}", category: "structure", description: "上方标注", params: &["label", "base"], optional: false },
    KEntry { key: "underset", latex: r"\underset{b}{X}", category: "structure", description: "下方标注", params: &["label", "base"], optional: false },
    KEntry { key: "stackrel", latex: r"X \stackrel{a}{=} Y", category: "structure", description: "上方带等号的标注", params: &["base", "label", "tail"], optional: false },
    KEntry { key: "cases", latex: r"\begin{cases} a & \text{if } x > 0 \\ b & \text{otherwise} \end{cases}", category: "structure", description: "分段函数定义", params: &["cases"], optional: false },
    // === 括号类 ===
    KEntry { key: "paren", latex: r"\left( x \right)", category: "delimiter", description: "圆括号（自动伸缩）", params: &["content"], optional: false },
    KEntry { key: "bracket", latex: r"\left[ x \right]", category: "delimiter", description: "方括号（自动伸缩）", params: &["content"], optional: false },
    KEntry { key: "brace", latex: r"\left\{ x \right\}", category: "delimiter", description: "花括号（自动伸缩）", params: &["content"], optional: false },
    KEntry { key: "angle", latex: r"\langle x, y \rangle", category: "delimiter", description: "内积/尖括号", params: &["left", "right"], optional: false },
    // === 关系符 ===
    KEntry { key: "leq", latex: r"a \leq b", category: "relation", description: "小于等于", params: &["left", "right"], optional: false },
    KEntry { key: "geq", latex: r"a \geq b", category: "relation", description: "大于等于", params: &["left", "right"], optional: false },
    KEntry { key: "neq", latex: r"a \neq b", category: "relation", description: "不等于", params: &["left", "right"], optional: false },
    KEntry { key: "approx", latex: r"a \approx b", category: "relation", description: "约等于", params: &["left", "right"], optional: false },
    KEntry { key: "equiv", latex: r"a \equiv b", category: "relation", description: "恒等于/同余", params: &["left", "right"], optional: false },
    KEntry { key: "sim", latex: r"a \sim b", category: "relation", description: "相似/渐近等价", params: &["left", "right"], optional: false },
    KEntry { key: "propto", latex: r"a \propto b", category: "relation", description: "正比于", params: &["left", "right"], optional: false },
    KEntry { key: "in_set", latex: r"x \in S", category: "relation", description: "属于集合", params: &["elem", "set"], optional: false },
    KEntry { key: "subset", latex: r"A \subset B", category: "relation", description: "子集", params: &["left", "right"], optional: false },
    // === 数学环境 ===
    KEntry { key: "equation", latex: r"\begin{equation} a = b \end{equation}", category: "env_math", description: "行间公式（带编号）", params: &["body"], optional: false },
    KEntry { key: "equation_star", latex: r"\begin{equation*} a = b \end{equation*}", category: "env_math", description: "行间公式（无编号）", params: &["body"], optional: false },
    KEntry { key: "align", latex: r"\begin{align} a &= b \\ c &= d \end{align}", category: "env_math", description: "对齐环境（多行等号对齐）", params: &["lines"], optional: false },
    KEntry { key: "gather", latex: r"\begin{gather} a \\ b \end{gather}", category: "env_math", description: "居中多行公式", params: &["lines"], optional: false },
    KEntry { key: "multline", latex: r"\begin{multline} a + b + c + d + e \\ + f + g \end{multline}", category: "env_math", description: "多行公式（首行左对齐，末行右对齐）", params: &["lines"], optional: false },
    KEntry { key: "split", latex: r"\begin{split} a &= b + c \\ &= d \end{split}", category: "env_math", description: "拆分对齐（需嵌套在 equation 内）", params: &["lines"], optional: false },
    // === 矩阵环境 ===
    KEntry { key: "matrix", latex: r"\begin{matrix} a & b \\ c & d \end{matrix}", category: "env_matrix", description: "无括号矩阵", params: &["rows"], optional: false },
    KEntry { key: "pmatrix", latex: r"\begin{pmatrix} a & b \\ c & d \end{pmatrix}", category: "env_matrix", description: "圆括号矩阵", params: &["rows"], optional: false },
    KEntry { key: "bmatrix", latex: r"\begin{bmatrix} a & b \\ c & d \end{bmatrix}", category: "env_matrix", description: "方括号矩阵", params: &["rows"], optional: false },
    KEntry { key: "Bmatrix", latex: r"\begin{Bmatrix} a & b \\ c & d \end{Bmatrix}", category: "env_matrix", description: "花括号矩阵", params: &["rows"], optional: false },
    KEntry { key: "vmatrix", latex: r"\begin{vmatrix} a & b \\ c & d \end{vmatrix}", category: "env_matrix", description: "单竖线行列式", params: &["rows"], optional: false },
    KEntry { key: "Vmatrix", latex: r"\begin{Vmatrix} a & b \\ c & d \end{Vmatrix}", category: "env_matrix", description: "双竖线范数", params: &["rows"], optional: false },
    KEntry { key: "cases_inline", latex: r"\begin{cases} x & x \geq 0 \\ -x & x < 0 \end{cases}", category: "env_matrix", description: "分段函数（cases 矩阵）", params: &["rows"], optional: false },
    // === 希腊字母（节选）===
    KEntry { key: "alpha", latex: r"\alpha", category: "greek", description: "希腊字母 alpha", params: &[], optional: false },
    KEntry { key: "beta", latex: r"\beta", category: "greek", description: "希腊字母 beta", params: &[], optional: false },
    KEntry { key: "gamma", latex: r"\gamma", category: "greek", description: "希腊字母 gamma", params: &[], optional: false },
    KEntry { key: "delta", latex: r"\delta", category: "greek", description: "希腊字母 delta", params: &[], optional: false },
    KEntry { key: "epsilon", latex: r"\epsilon", category: "greek", description: "希腊字母 epsilon", params: &[], optional: false },
    KEntry { key: "theta", latex: r"\theta", category: "greek", description: "希腊字母 theta", params: &[], optional: false },
    KEntry { key: "lambda", latex: r"\lambda", category: "greek", description: "希腊字母 lambda", params: &[], optional: false },
    KEntry { key: "mu", latex: r"\mu", category: "greek", description: "希腊字母 mu", params: &[], optional: false },
    KEntry { key: "pi", latex: r"\pi", category: "greek", description: "希腊字母 pi（圆周率）", params: &[], optional: false },
    KEntry { key: "sigma", latex: r"\sigma", category: "greek", description: "希腊字母 sigma", params: &[], optional: false },
    KEntry { key: "phi", latex: r"\phi", category: "greek", description: "希腊字母 phi", params: &[], optional: false },
    KEntry { key: "omega", latex: r"\omega", category: "greek", description: "希腊字母 omega", params: &[], optional: false },
    // === 符号 ===
    KEntry { key: "infty", latex: r"\infty", category: "symbol", description: "无穷大符号", params: &[], optional: false },
    KEntry { key: "partial", latex: r"\partial", category: "symbol", description: "偏导符号", params: &[], optional: false },
    KEntry { key: "nabla", latex: r"\nabla", category: "symbol", description: "nabla 算子", params: &[], optional: false },
    KEntry { key: "forall", latex: r"\forall", category: "symbol", description: "全称量词", params: &[], optional: false },
    KEntry { key: "exists", latex: r"\exists", category: "symbol", description: "存在量词", params: &[], optional: false },
    KEntry { key: "hbar", latex: r"\hbar", category: "symbol", description: "约化普朗克常数", params: &[], optional: false },
    KEntry { key: "cdot", latex: r"\cdot", category: "symbol", description: "点乘", params: &[], optional: false },
    KEntry { key: "times", latex: r"\times", category: "symbol", description: "叉乘/笛卡尔积", params: &[], optional: false },
    KEntry { key: "to_arrow", latex: r"\to", category: "symbol", description: "右箭头", params: &[], optional: false },
    KEntry { key: "mapsto", latex: r"\mapsto", category: "symbol", description: "映射箭头", params: &[], optional: false },
    KEntry { key: "Rightarrow", latex: r"\Rightarrow", category: "symbol", description: "双线右箭头（蕴含）", params: &[], optional: false },
    KEntry { key: "Leftrightarrow", latex: r"\Leftrightarrow", category: "symbol", description: "双线左右箭头（等价）", params: &[], optional: false },
];

/// 关键词 → 知识库 key 映射（插入序承载稳定排序语义，逐字对表）。
static KEYWORD_MAP: &[(&str, &str)] = &[
    // 英文
    ("integral", "integral_definite"),
    ("integrate", "integral_indefinite"),
    ("integration", "integral_definite"),
    ("indefinite integral", "integral_indefinite"),
    ("double integral", "integral_double"),
    ("triple integral", "integral_triple"),
    ("line integral", "integral_line"),
    ("sum", "sum"),
    ("summation", "sum"),
    ("series", "sum_infinite"),
    ("product", "product"),
    ("limit", "limit"),
    ("limit infinity", "limit_infinity"),
    ("derivative", "derivative"),
    ("differentiate", "derivative"),
    ("partial derivative", "partial_derivative"),
    ("nth derivative", "nth_derivative"),
    ("gradient", "gradient"),
    ("divergence", "divergence"),
    ("curl", "curl"),
    ("sin", "sin"),
    ("cosine", "cos"),
    ("cos", "cos"),
    ("tan", "tan"),
    ("tangent", "tan"),
    ("logarithm", "log"),
    ("log", "log"),
    ("natural log", "ln"),
    ("ln", "ln"),
    ("exponential", "exp"),
    ("exp", "exp"),
    ("square root", "sqrt"),
    ("sqrt", "sqrt"),
    ("root", "nth_root"),
    ("absolute value", "abs"),
    ("abs", "abs"),
    ("floor", "floor"),
    ("ceil", "ceil"),
    ("fraction", "frac"),
    ("frac", "frac"),
    ("binomial", "binom"),
    ("choose", "binom"),
    ("subscript", "supsub"),
    ("superscript", "supsub"),
    ("sub sup", "supsub"),
    ("cases", "cases"),
    ("piecewise", "cases"),
    ("matrix", "matrix"),
    ("parenthesis matrix", "pmatrix"),
    ("bracket matrix", "bmatrix"),
    ("brace matrix", "Bmatrix"),
    ("determinant", "vmatrix"),
    ("norm", "Vmatrix"),
    ("equation", "equation"),
    ("display equation", "equation_star"),
    ("align", "align"),
    ("gather", "gather"),
    ("multline", "multline"),
    ("split", "split"),
    ("alpha", "alpha"),
    ("beta", "beta"),
    ("gamma", "gamma"),
    ("delta", "delta"),
    ("epsilon", "epsilon"),
    ("theta", "theta"),
    ("lambda", "lambda"),
    ("mu", "mu"),
    ("pi", "pi"),
    ("sigma", "sigma"),
    ("phi", "phi"),
    ("omega", "omega"),
    ("infinity", "infty"),
    ("infty", "infty"),
    ("partial", "partial"),
    ("nabla", "nabla"),
    ("forall", "forall"),
    ("exists", "exists"),
    ("hbar", "hbar"),
    ("dot product", "cdot"),
    ("cross product", "times"),
    ("arrow", "to_arrow"),
    ("mapsto", "mapsto"),
    ("implies", "Rightarrow"),
    ("iff", "Leftrightarrow"),
    // 中文
    ("积分", "integral_definite"),
    ("定积分", "integral_definite"),
    ("不定积分", "integral_indefinite"),
    ("二重积分", "integral_double"),
    ("三重积分", "integral_triple"),
    ("线积分", "integral_line"),
    ("求和", "sum"),
    ("级数", "sum_infinite"),
    ("连乘", "product"),
    ("极限", "limit"),
    ("无穷极限", "limit_infinity"),
    ("导数", "derivative"),
    ("求导", "derivative"),
    ("偏导", "partial_derivative"),
    ("梯度", "gradient"),
    ("散度", "divergence"),
    ("旋度", "curl"),
    ("正弦", "sin"),
    ("余弦", "cos"),
    ("正切", "tan"),
    ("对数", "log"),
    ("自然对数", "ln"),
    ("指数", "exp"),
    ("平方根", "sqrt"),
    ("开方", "nth_root"),
    ("绝对值", "abs"),
    ("下取整", "floor"),
    ("上取整", "ceil"),
    ("分数", "frac"),
    ("二项式", "binom"),
    ("分段", "cases"),
    ("矩阵", "matrix"),
    ("圆括号矩阵", "pmatrix"),
    ("方括号矩阵", "bmatrix"),
    ("行列式", "vmatrix"),
    ("范数", "Vmatrix"),
    ("公式", "equation"),
    ("对齐", "align"),
    ("希腊字母", "alpha"),
    ("无穷", "infty"),
    ("偏导符号", "partial"),
    ("全称量词", "forall"),
    ("存在量词", "exists"),
];

fn knowledge_index(key: &str) -> Option<usize> {
    KNOWLEDGE.iter().position(|e| e.key == key)
}

/// 按自然语言查询知识库（评分去重排序，逐字对表 search()）。
fn knowledge_search(query: &str) -> Vec<&'static KEntry> {
    let q = query.to_lowercase();
    let q = q.trim();

    let mut order: Vec<usize> = Vec::new();
    let mut pos_of: HashMap<usize, usize> = HashMap::new();
    let mut score_of: HashMap<usize, usize> = HashMap::new();

    fn bump(
        kidx: usize,
        sc: usize,
        order: &mut Vec<usize>,
        pos_of: &mut HashMap<usize, usize>,
        score_of: &mut HashMap<usize, usize>,
    ) {
        match pos_of.get(&kidx) {
            Some(_) => {
                if sc > *score_of.get(&kidx).unwrap() {
                    score_of.insert(kidx, sc);
                }
            }
            None => {
                pos_of.insert(kidx, order.len());
                order.push(kidx);
                score_of.insert(kidx, sc);
            }
        }
    }

    for (kw, key) in KEYWORD_MAP {
        if q.contains(kw) {
            if let Some(kidx) = knowledge_index(key) {
                bump(kidx, kw.chars().count(), &mut order, &mut pos_of, &mut score_of);
            }
        }
    }
    for (i, e) in KNOWLEDGE.iter().enumerate() {
        if q.contains(e.key) {
            bump(i, e.key.chars().count() * 2, &mut order, &mut pos_of, &mut score_of);
        }
    }

    let mut pairs: Vec<(usize, usize)> = order.iter().map(|&k| (k, score_of[&k])).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1)); // 稳定排序，同分保首现序（对表 Python stable sorted）
    pairs.into_iter().map(|(k, _)| &KNOWLEDGE[k]).collect()
}

fn cmd_knowledge(rest: &[String]) -> i32 {
    let p = match parse_args(rest, &["format"], &[]) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    if !p.positionals.is_empty() {
        usage_fail("knowledge 不接受位置参数");
    }
    let _format = get_format(&p); // 围堰 handler 忽略 format 恒出 json
    let keys: Vec<&str> = KNOWLEDGE.iter().map(|e| e.key).collect();
    let mut categories: Vec<&str> = Vec::new();
    for e in KNOWLEDGE {
        if !categories.contains(&e.category) {
            categories.push(e.category);
        }
    }
    out_json(&json!({"count": KNOWLEDGE.len(), "keys": keys, "categories": categories}));
    EXIT_OK
}

// ============ suggest.py：LaTeX 语法提示 ============

fn cmd_suggest(rest: &[String]) -> i32 {
    let p = match parse_args(rest, &["limit", "format"], &[]) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    if p.positionals.len() != 1 {
        usage_fail("suggest 需要恰好一个查询位置参数");
    }
    let query = p.positionals[0].clone();
    let limit: i64 = match p.values.get("limit") {
        Some(s) => match s.parse() {
            Ok(n) => n,
            Err(_) => usage_fail("--limit 需要整数"),
        },
        None => 5,
    };
    let format = get_format(&p);

    if query.is_empty() || query.trim().is_empty() {
        if format == "human" {
            println!("[error] empty query");
        } else {
            out_json(&json!({"query": query, "matches": [], "count": 0, "error": "empty query"}));
        }
        return EXIT_OK;
    }

    let found = knowledge_search(&query);
    let lim = if limit < 0 { 0 } else { limit as usize }; // 落差申报三：负值按 0
    let taken: Vec<&KEntry> = found.into_iter().take(lim).collect();

    let matches_json: Vec<Value> = taken
        .iter()
        .map(|m| {
            json!({
                "key": m.key,
                "latex": m.latex,
                "category": m.category,
                "description": m.description,
                "params": m.params,
                "optional": m.optional,
            })
        })
        .collect();
    let count = matches_json.len() as i64;
    out_json(&json!({"query": query, "matches": matches_json, "count": count}));
    EXIT_OK
}

// ============ blocks.py：Markdown 内 LaTeX 代码块创建 ============

/// 模板参数表：(参数名, 是否必填, 默认值)，逐字对表 TEMPLATES params_schema。
fn template_schema(name: &str) -> Option<&'static [(&'static str, bool, &'static str)]> {
    match name {
        "integral" => Some(&[("lower", false, ""), ("upper", false, ""), ("integrand", true, "f(x)"), ("var", false, "x")]),
        "sum" => Some(&[("lower", false, "i=1"), ("upper", false, "n"), ("summand", true, "a_i")]),
        "lim" => Some(&[("var", false, "x"), ("to", true, r"\infty"), ("expr", true, "f(x)")]),
        "frac" => Some(&[("num", true, "a"), ("den", true, "b")]),
        "sqrt" => Some(&[("arg", true, "x"), ("index", false, "")]),
        "matrix" => Some(&[("rows", true, "a & b \\\\ c & d")]),
        "pmatrix" => Some(&[("rows", true, "a & b \\\\ c & d")]),
        "align" => Some(&[("lines", true, "a &= b \\\\ c &= d")]),
        "equation" => Some(&[("body", true, "a = b")]),
        _ => None,
    }
}

fn template_description(name: &str) -> &'static str {
    match name {
        "integral" => "积分（带上下限）",
        "sum" => "求和",
        "lim" => "极限",
        "frac" => "分数",
        "sqrt" => "根式",
        "matrix" => "矩阵（无括号）",
        "pmatrix" => "圆括号矩阵",
        "align" => "对齐多行公式",
        "equation" => "行间公式（带编号）",
        _ => "",
    }
}

/// 模板名清单（sorted 序，对表 list_templates）。
const TEMPLATE_NAMES_SORTED: [&str; 9] =
    ["align", "equation", "frac", "integral", "lim", "matrix", "pmatrix", "sqrt", "sum"];

/// 解析 'k=v,k=v' 形参数串（含引号值支持，逐字对表 parse_params）。
fn parse_params(params_str: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    if params_str.trim().is_empty() {
        return out;
    }
    let s: Vec<char> = params_str.chars().collect();
    let n = s.len();
    let mut i = 0usize;
    while i < n {
        while i < n && (s[i] == ' ' || s[i] == '\t') {
            i += 1;
        }
        if i >= n {
            break;
        }
        // 读 key
        let k_start = i;
        while i < n && s[i] != '=' {
            i += 1;
        }
        let key: String = s[k_start..i].iter().collect();
        let key = key.trim().to_string();
        if key.is_empty() || i >= n {
            break;
        }
        i += 1; // skip '='
        // 读 value
        let value: String;
        if i < n && (s[i] == '"' || s[i] == '\'') {
            let quote = s[i];
            i += 1;
            let v_start = i;
            while i < n && s[i] != quote {
                i += 1;
            }
            value = s[v_start..i].iter().collect();
            i += 1; // skip closing quote
        } else {
            let v_start = i;
            while i < n && s[i] != ',' {
                i += 1;
            }
            let raw: String = s[v_start..i].iter().collect();
            value = raw.trim().to_string();
        }
        if !key.is_empty() {
            out.insert(key, value);
        }
        if i < n && s[i] == ',' {
            i += 1;
        }
    }
    out
}

/// 根据模板名和参数构造 LaTeX 表达式（Err 即 ValueError 对应）。
fn build_block(template: &str, params: &HashMap<String, String>) -> Result<String, String> {
    let schema = match template_schema(template) {
        Some(s) => s,
        None => {
            return Err(format!(
                "unknown template: '{}' (available: {})",
                template,
                TEMPLATE_NAMES_SORTED.join(", ")
            ))
        }
    };
    // 合并：用户提供 > 默认
    let mut final_params: HashMap<&str, String> = HashMap::new();
    for (k, _required, default) in schema {
        let v = params.get(*k).cloned().unwrap_or_else(|| default.to_string());
        final_params.insert(k, v);
    }
    // 检查必填（空串即缺失，对表 Python falsy 判）
    for (k, required, _) in schema {
        if *required && final_params.get(*k).map(|v| v.is_empty()).unwrap_or(true) {
            return Err(format!("missing required param: {k}"));
        }
    }
    let g = |k: &str| final_params.get(k).cloned().unwrap_or_default();
    Ok(match template {
        "integral" => format!("\\int_{{{}}}^{{{}}} {} \\, d{}", g("lower"), g("upper"), g("integrand"), g("var")),
        "sum" => format!("\\sum_{{{}}}^{{{}}} {}", g("lower"), g("upper"), g("summand")),
        "lim" => format!("\\lim_{{{} \\to {}}} {}", g("var"), g("to"), g("expr")),
        "frac" => format!("\\frac{{{}}}{{{}}}", g("num"), g("den")),
        "sqrt" => {
            let index = g("index");
            if !index.is_empty() {
                format!("\\sqrt[{}]{{{}}}", index, g("arg"))
            } else {
                format!("\\sqrt{{{}}}", g("arg"))
            }
        }
        "matrix" => format!("\\begin{{matrix}}\n{}\n\\end{{matrix}}", g("rows")),
        "pmatrix" => format!("\\begin{{pmatrix}}\n{}\n\\end{{pmatrix}}", g("rows")),
        "align" => format!("\\begin{{align}}\n{}\n\\end{{align}}", g("lines")),
        "equation" => format!("\\begin{{equation}}\n{}\n\\end{{equation}}", g("body")),
        _ => unreachable!(),
    })
}

fn cmd_block_create(rest: &[String]) -> i32 {
    let p = match parse_args(rest, &["template", "params", "format"], &["list"]) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    if !p.positionals.is_empty() {
        usage_fail("block-create 不接受位置参数");
    }
    let format = get_format(&p);

    if p.bools.contains(&"list") {
        let templates: Vec<Value> = TEMPLATE_NAMES_SORTED
            .iter()
            .map(|name| json!({"name": name, "description": template_description(name)}))
            .collect();
        if format == "json" {
            out_json(&json!({"templates": templates}));
        } else {
            for name in TEMPLATE_NAMES_SORTED {
                println!("  {:<20}  {}", name, template_description(name));
            }
        }
        return EXIT_OK;
    }

    // 围堰：--template 缺省时 None 参与 unknown template 判，报文含 'None'，json template 键为 null
    let template_opt = p.values.get("template").cloned();
    let template_str = template_opt.clone().unwrap_or_else(|| "None".to_string());
    let params_str = p.values.get("params").cloned().unwrap_or_default();
    let tpl_json = match &template_opt {
        Some(t) => json!(t),
        None => Value::Null,
    };

    match build_block(&template_str, &parse_params(&params_str)) {
        Ok(expr) => {
            let block = format!("```latex\n{expr}\n```");
            if format == "json" {
                out_json(&json!({"template": tpl_json, "params": params_str, "block": block}));
            } else {
                let shown = if params_str.is_empty() { "(defaults)".to_string() } else { params_str };
                print!("Template: {template_str}\nParams:   {shown}\n---\n{block}");
            }
            EXIT_OK
        }
        Err(msg) => {
            out_json(&json!({"error": msg, "template": tpl_json}));
            EXIT_ERROR
        }
    }
}

// ============ validate.py：LaTeX 语法验证（5 类错误检测） ============

struct Issue {
    rule_id: &'static str, // UNCLOSED_BRACKET / MISSING_ENV / UNDEFINED_REF / REQUIRED_PARAM / ENV_MISMATCH
    message: String,
    line: i64,
    column: i64,
    severity: &'static str, // "error" | "warning"
    context: String,
}

impl Issue {
    fn to_value(&self) -> Value {
        json!({
            "rule_id": self.rule_id,
            "message": self.message,
            "line": self.line,
            "column": self.column,
            "severity": self.severity,
            "context": self.context,
        })
    }
}

fn re_esc_bracket() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\\[{}()\[\]]").unwrap())
}
fn re_half_left() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\[\s*([^\]\[()\n]{1,30})\s*,\s*([^\]\[()\n]{1,30})\s*\)").unwrap())
}
fn re_half_right() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\(\s*([^\]\[()\n]{1,30})\s*,\s*([^\]\[()\n]{1,30})\s*\]").unwrap())
}
fn re_begin() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\\begin\{([^}]+)\}").unwrap())
}
fn re_end() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\\end\{([^}]+)\}").unwrap())
}
fn re_declared() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\\(?:label|bibitem|hypertarget)\{([^}]+)\}").unwrap())
}
fn re_ref() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\\(?:ref|eqref|cite|cref|autoref)\{([^}]+)\}").unwrap())
}

/// Python s[a:b] 语义切片（越界钳制，码点计）。
fn py_slice(s: &str, start: isize, end: isize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len() as isize;
    let a = start.clamp(0, n) as usize;
    let b = end.clamp(0, n) as usize;
    if a >= b {
        String::new()
    } else {
        chars[a..b].iter().collect()
    }
}

fn byte_to_char(s: &str, byte_pos: usize) -> usize {
    s[..byte_pos].chars().count()
}

/// 检查未闭合括号（逐字对表 _check_brackets：转义豁免、\left/\right、命令可选段、
/// 标准数学半开区间白名单、字符位栈匹配）。
fn check_brackets(content: &str, line: i64, col_offset: i64, issues: &mut Vec<Issue>) {
    let stripped = re_esc_bracket().replace_all(content, "").into_owned();
    let chars: Vec<char> = stripped.chars().collect();
    let n = chars.len();

    // 半开区间白名单（字符位空间，排序后跳扫）
    let mut spans: Vec<(usize, usize)> = Vec::new();
    for r in [re_half_left(), re_half_right()] {
        for m in r.find_iter(&stripped) {
            let cs = byte_to_char(&stripped, m.start());
            let ce = cs + stripped[m.start()..m.end()].chars().count();
            spans.push((cs, ce));
        }
    }
    spans.sort();

    let mut stack: Vec<(char, usize)> = Vec::new();
    let mut span_idx = 0usize;
    let n_spans = spans.len();
    let mut i = 0usize;
    while i < n {
        while span_idx < n_spans && spans[span_idx].1 <= i {
            span_idx += 1;
        }
        if span_idx < n_spans && spans[span_idx].0 <= i && i < spans[span_idx].1 {
            i = spans[span_idx].1;
            continue;
        }
        let c = chars[i];
        if c == '{' || c == '[' || c == '(' {
            stack.push((c, i));
            i += 1;
        } else if c == '}' || c == ']' || c == ')' {
            if stack.is_empty() {
                issues.push(Issue {
                    rule_id: "UNCLOSED_BRACKET",
                    message: format!("unmatched closing '{c}'"),
                    line,
                    column: col_offset + i as i64,
                    severity: "error",
                    context: py_slice(&stripped, i as isize - 20, i as isize + 20),
                });
                i += 1;
            } else {
                let (open_c, _oc) = *stack.last().unwrap();
                let expected = match open_c {
                    '{' => '}',
                    '[' => ']',
                    _ => ')',
                };
                if c == expected {
                    stack.pop();
                } else {
                    issues.push(Issue {
                        rule_id: "UNCLOSED_BRACKET",
                        message: format!("mismatched: expected '{expected}' for '{open_c}' but found '{c}'"),
                        line,
                        column: col_offset + i as i64,
                        severity: "error",
                        context: py_slice(&stripped, i as isize - 20, i as isize + 20),
                    });
                    stack.pop();
                }
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    for (open_c, oc) in stack {
        issues.push(Issue {
            rule_id: "UNCLOSED_BRACKET",
            message: format!("unclosed '{open_c}'"),
            line,
            column: col_offset + oc as i64,
            severity: "error",
            context: String::new(),
        });
    }
}

/// 跨行检查 \begin{X} / \end{X} 配对（逐字对表 _check_envs_in_text）。
fn check_envs_in_text(text: &str, issues: &mut Vec<Issue>) {
    // 每行起始码点偏移
    let mut line_offsets: Vec<usize> = Vec::new();
    let mut offset = 0usize;
    for line in text.split('\n') {
        line_offsets.push(offset);
        offset += line.chars().count() + 1;
    }
    let line_col = |pos: usize| -> (i64, i64) {
        let mut lo = 0usize;
        let mut hi = line_offsets.len() - 1;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if line_offsets[mid] <= pos {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        (lo as i64 + 1, pos as i64 - line_offsets[lo] as i64 + 1)
    };

    let mut begins: Vec<(usize, String)> = Vec::new();
    for c in re_begin().captures_iter(text) {
        let m = c.get(0).unwrap();
        begins.push((byte_to_char(text, m.start()), c[1].to_string()));
    }
    let mut ends: Vec<(usize, String)> = Vec::new();
    for c in re_end().captures_iter(text) {
        let m = c.get(0).unwrap();
        ends.push((byte_to_char(text, m.start()), c[1].to_string()));
    }

    let mut begin_stack: Vec<(String, i64, i64)> = Vec::new();
    let mut bi = 0usize;
    let mut ei = 0usize;
    while bi < begins.len() || ei < ends.len() {
        let bp = if bi < begins.len() { begins[bi].0 } else { usize::MAX };
        let ep = if ei < ends.len() { ends[ei].0 } else { usize::MAX };
        if bp < ep {
            let (pos, env) = &begins[bi];
            let (ln, co) = line_col(*pos);
            begin_stack.push((env.clone(), ln, co));
            bi += 1;
        } else {
            let (pos, env) = &ends[ei];
            let (ln, co) = line_col(*pos);
            if begin_stack.is_empty() {
                issues.push(Issue {
                    rule_id: "ENV_MISMATCH",
                    message: format!("\\end{{{env}}} without matching \\begin"),
                    line: ln,
                    column: co,
                    severity: "error",
                    context: py_slice(text, *pos as isize - 20, *pos as isize + 30),
                });
            } else {
                let (top_env, tl, _tc) = begin_stack.last().unwrap().clone();
                if top_env != *env {
                    issues.push(Issue {
                        rule_id: "ENV_MISMATCH",
                        message: format!("\\end{{{env}}} does not match \\begin{{{top_env}}} at line {tl}"),
                        line: ln,
                        column: co,
                        severity: "error",
                        context: py_slice(text, *pos as isize - 20, *pos as isize + 30),
                    });
                    begin_stack.pop();
                } else {
                    begin_stack.pop();
                }
            }
            ei += 1;
        }
    }

    for (env, tl, tc) in begin_stack {
        issues.push(Issue {
            rule_id: "MISSING_ENV",
            message: format!("\\begin{{{env}}} has no matching \\end"),
            line: tl,
            column: tc,
            severity: "error",
            context: String::new(),
        });
    }
}

/// 收集全文件已声明引用 key（\label / \bibitem / \hypertarget）。
fn collect_declared(text: &str) -> HashSet<String> {
    re_declared()
        .captures_iter(text)
        .map(|c| c[1].to_string())
        .collect()
}

/// 检查 \ref / \cite 引用未声明（warning 级，逐字对表 _check_refs）。
fn check_refs(content: &str, line: i64, declared: &HashSet<String>, issues: &mut Vec<Issue>) {
    for c in re_ref().captures_iter(content) {
        let m0 = c.get(0).unwrap();
        let cs = byte_to_char(content, m0.start());
        for k in c[1].split(',') {
            let k = k.trim();
            if !k.is_empty() && !declared.contains(k) {
                issues.push(Issue {
                    rule_id: "UNDEFINED_REF",
                    message: format!("reference '{k}' is not declared in this file"),
                    line,
                    column: cs as i64,
                    severity: "warning",
                    context: py_slice(content, cs as isize - 20, cs as isize + 40),
                });
            }
        }
    }
}

/// 缺必填 {} 参数的命令表（保持 dict 序，逐字对表 _REQUIRED_PARAM_COMMANDS）。
const REQUIRED_PARAM_COMMANDS: [(&str, usize); 12] = [
    ("\\frac", 2),
    ("\\binom", 2),
    ("\\sqrt", 1),
    ("\\hat", 1),
    ("\\bar", 1),
    ("\\vec", 1),
    ("\\dot", 1),
    ("\\ddot", 1),
    ("\\tilde", 1),
    ("\\widehat", 1),
    ("\\widetilde", 1),
    ("\\overrightarrow", 1),
];

/// 从 start 起计连续 {...} 块数（逐字对表 _count_braced_args）。
fn count_braced_args(chars: &[char], start: usize) -> usize {
    let mut count = 0usize;
    let mut i = start;
    let n = chars.len();
    while i < n && chars[i] == '{' {
        let mut depth = 1i32;
        i += 1;
        while i < n && depth > 0 {
            if chars[i] == '{' {
                depth += 1;
            } else if chars[i] == '}' {
                depth -= 1;
            }
            i += 1;
        }
        count += 1;
        while i < n && (chars[i] == ' ' || chars[i] == '\t') {
            i += 1;
        }
    }
    count
}

/// 检查 \frac \binom \sqrt 等缺必填参数（逐字对表 _check_required_params；
/// 负向字母前瞻以手工扫描承载）。
fn check_required_params(content: &str, line: i64, col_offset: i64, issues: &mut Vec<Issue>) {
    let chars: Vec<char> = content.chars().collect();
    let n = chars.len();
    for (cmd, n_required) in REQUIRED_PARAM_COMMANDS {
        let cmd_chars: Vec<char> = cmd.chars().collect();
        let mut i = 0usize;
        while i + cmd_chars.len() <= n {
            if chars[i..].starts_with(&cmd_chars) {
                let m_start = i;
                let cmd_end = i + cmd_chars.len();
                let next_ok = cmd_end >= n || !chars[cmd_end].is_ascii_alphabetic();
                if next_ok {
                    let mut after = cmd_end;
                    // \sqrt 特殊：跳过可选 [index]（仅 ']' 递减，逐字对表）
                    if cmd == "\\sqrt" && after < n && chars[after] == '[' {
                        let mut depth = 1i32;
                        let mut j = after + 1;
                        while j < n && depth > 0 {
                            if chars[j] == ']' {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            j += 1;
                        }
                        after = j + 1;
                    }
                    while after < n && (chars[after] == ' ' || chars[after] == '\t') {
                        after += 1;
                    }
                    if after >= n || chars[after] != '{' {
                        issues.push(Issue {
                            rule_id: "REQUIRED_PARAM",
                            message: format!("{cmd} missing required argument"),
                            line,
                            column: col_offset + m_start as i64,
                            severity: "error",
                            context: py_slice(content, m_start as isize - 10, m_start as isize + 30),
                        });
                        i = cmd_end;
                        continue;
                    }
                    let n_args = count_braced_args(&chars, after);
                    if n_args < n_required {
                        issues.push(Issue {
                            rule_id: "REQUIRED_PARAM",
                            message: format!("{cmd} requires {n_required} arguments, got {n_args}"),
                            line,
                            column: col_offset + m_start as i64,
                            severity: "error",
                            context: py_slice(content, m_start as isize - 10, m_start as isize + 40),
                        });
                    }
                    i = cmd_end;
                    continue;
                }
            }
            i += 1;
        }
    }
}

/// 全文本验证（逐字对表 validate_text）：环境跨行、括号/引用/必填项逐行。
fn validate_text_issues(text: &str) -> (Vec<Issue>, Vec<Issue>) {
    let mut errors: Vec<Issue> = Vec::new();
    let mut warnings: Vec<Issue> = Vec::new();

    check_envs_in_text(text, &mut errors);

    let declared = collect_declared(text);
    for (i, content) in text.split('\n').enumerate() {
        let line = i as i64 + 1;
        check_brackets(content, line, 0, &mut errors);
        check_refs(content, line, &declared, &mut warnings);
        check_required_params(content, line, 0, &mut errors);
    }
    (errors, warnings)
}

fn cmd_validate(rest: &[String]) -> i32 {
    let p = match parse_args(rest, &["file", "text", "format"], &[]) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    if !p.positionals.is_empty() {
        usage_fail("validate 不接受位置参数");
    }
    let format = get_format(&p);
    let file = p.values.get("file").cloned();
    let text = p.values.get("text").cloned();

    // text 优先于 file（对表 cli.py cmd_validate）
    let (json_v, human, ne) = if let Some(t) = text {
        let (errs, warns) = validate_text_issues(&t);
        assemble_validate_result(None, errs, warns)
    } else if let Some(f) = file {
        if !Path::new(&f).exists() {
            out_json(&json!({"error": format!("file not found: {f}")}));
            return EXIT_ERROR;
        }
        match fs::read_to_string(&f) {
            Ok(content) => {
                let (errs, warns) = validate_text_issues(&content);
                assemble_validate_result(Some(f), errs, warns)
            }
            Err(err) => {
                out_json(&json!({"error": format!("unexpected: {err}")}));
                return EXIT_ERROR;
            }
        }
    } else {
        out_json(&json!({"error": "must provide --file or --text"}));
        return EXIT_ERROR;
    };

    if format == "human" {
        println!("{human}");
    } else {
        out_json(&json_v);
    }
    if ne > 0 {
        EXIT_VIOLATION
    } else {
        EXIT_OK
    }
}

fn assemble_validate_result(
    file: Option<String>,
    errs: Vec<Issue>,
    warns: Vec<Issue>,
) -> (Value, String, i64) {
    let ne = errs.len() as i64;
    let nw = warns.len() as i64;
    let errors: Vec<Value> = errs.iter().map(|i| i.to_value()).collect();
    let warnings: Vec<Value> = warns.iter().map(|i| i.to_value()).collect();
    let json_v = match &file {
        Some(f) => json!({"file": f, "errors": errors, "warnings": warnings, "summary": {"errors": ne, "warnings": nw}}),
        None => json!({"errors": errors, "warnings": warnings, "summary": {"errors": ne, "warnings": nw}}),
    };
    // human 形逐字对表 validate.format_human
    let mut lines: Vec<String> = Vec::new();
    match &file {
        Some(f) => lines.push(format!("File: {f}")),
        None => lines.push("Validation:".to_string()),
    }
    lines.push(format!("Errors: {ne}, Warnings: {nw}"));
    lines.push(String::new());
    for e in &errs {
        lines.push(format!("  E [{}] line {}:{}: {}", e.rule_id, e.line, e.column, e.message));
    }
    for w in &warns {
        lines.push(format!("  W [{}] line {}:{}: {}", w.rule_id, w.line, w.column, w.message));
    }
    (json_v, lines.join("\n"), ne)
}

// ============ autofix.py：LaTeX 自动修复 ============

struct Fix {
    rule_id: &'static str,
    line: i64,
    column: i64,
    action: &'static str, // "added" | "manual_required"
    detail: String,
}

impl Fix {
    fn to_value(&self) -> Value {
        json!({
            "rule_id": self.rule_id,
            "line": self.line,
            "column": self.column,
            "action": self.action,
            "detail": self.detail,
        })
    }
}

struct AutofixOut {
    original_text: String,
    fixed_text: String,
    fixes: Vec<Fix>,
    changed: bool,
    added: usize,
    manual: usize,
}

fn esc_bracket_re(c: char) -> String {
    format!("\\{c}")
}

/// 未闭合括号行尾补全（\left 缺 \right 时补 \right 形，逐字对表
/// _fix_unclosed_brackets）。
fn fix_unclosed_brackets(text: &str, issues: &[Issue]) -> (String, Vec<Fix>) {
    let mut lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    let mut line_fixes: Vec<(usize, Vec<String>)> = Vec::new(); // 保持首现序（对表 Python dict 插入序）

    for issue in issues {
        if issue.rule_id != "UNCLOSED_BRACKET" {
            continue;
        }
        for c in ['{', '[', '('] {
            if issue.message.contains(&format!("unclosed '{c}'")) {
                let closer = match c {
                    '{' => '}',
                    '[' => ']',
                    _ => ')',
                };
                let mut use_right = false;
                if issue.line > 0 && (issue.line as usize) <= lines.len() {
                    let line_content = &lines[issue.line as usize - 1];
                    let left_re = Regex::new(&format!(r"\\left\s*\\?{}", esc_bracket_re(c))).unwrap();
                    let right_re = Regex::new(&format!(r"\\right\s*\\?{}", esc_bracket_re(closer))).unwrap();
                    if left_re.is_match(line_content) && !right_re.is_match(line_content) {
                        use_right = true;
                    }
                }
                let add = if use_right { format!("\\right{closer}") } else { closer.to_string() };
                match line_fixes.iter_mut().find(|(l, _)| *l == issue.line as usize) {
                    Some((_, v)) => v.push(add),
                    None => line_fixes.push((issue.line as usize, vec![add])),
                }
            }
        }
    }

    if line_fixes.is_empty() {
        return (text.to_string(), Vec::new());
    }

    let mut fixes: Vec<Fix> = Vec::new();
    for (line_no, closers) in line_fixes {
        let idx = line_no - 1;
        // 反向排列：先关内层
        let mut addition = String::new();
        for c in closers.iter().rev() {
            addition.push_str(c);
        }
        let trimmed = lines[idx].trim_end().to_string();
        lines[idx] = format!("{trimmed}{addition}");
        fixes.push(Fix {
            rule_id: "UNCLOSED_BRACKET",
            line: line_no as i64,
            column: lines[idx].chars().count() as i64,
            action: "added",
            detail: format!("appended '{addition}' to close bracket on line {line_no}"),
        });
    }
    (lines.join("\n"), fixes)
}

/// 缺失 \end 末尾补全（逐字对表 _fix_missing_env）。
fn fix_missing_env(text: &str, issues: &[Issue]) -> (String, Vec<Fix>) {
    let mut end_inserts: Vec<(i64, String)> = Vec::new();
    for issue in issues {
        if issue.rule_id == "MISSING_ENV" {
            if let Some(c) = re_begin().captures(&issue.message) {
                end_inserts.push((issue.line, c[1].to_string()));
            }
        }
    }
    if end_inserts.is_empty() {
        return (text.to_string(), Vec::new());
    }
    let mut new_text = format!("{}\n", text.trim_end());
    let mut fixes: Vec<Fix> = Vec::new();
    for (line_no, env) in end_inserts {
        new_text.push_str(&format!("\\end{{{env}}}\n"));
        fixes.push(Fix {
            rule_id: "MISSING_ENV",
            line: line_no,
            column: 0,
            action: "added",
            detail: format!("appended \\end{{{env}}} to close \\begin{{{env}}} from line {line_no}"),
        });
    }
    (new_text, fixes)
}

/// 复杂问题报人工（逐字对表 _classify_manual；自动可修两类排除）。
fn classify_manual(issues: &[Issue]) -> Vec<Fix> {
    issues
        .iter()
        .filter(|i| i.rule_id != "UNCLOSED_BRACKET" && i.rule_id != "MISSING_ENV")
        .map(|i| Fix {
            rule_id: i.rule_id,
            line: i.line,
            column: i.column,
            action: "manual_required",
            detail: i.message.clone(),
        })
        .collect()
}

/// 修复 LaTeX 文本（逐字对表 autofix_text：先括号补全，重验后环境补全与人工分类）。
fn autofix_text(text: &str) -> AutofixOut {
    let (e1, w1) = validate_text_issues(text);
    let all: Vec<Issue> = e1.into_iter().chain(w1).collect();

    let (text_after_brackets, fixes_brackets) = fix_unclosed_brackets(text, &all);

    let (e2, w2) = validate_text_issues(&text_after_brackets);
    let remaining: Vec<Issue> = e2.into_iter().chain(w2).collect();

    let (text_after_env, fixes_env) = fix_missing_env(&text_after_brackets, &remaining);

    let fixes_manual = classify_manual(&remaining);

    let mut fixes = fixes_brackets;
    fixes.extend(fixes_env);
    fixes.extend(fixes_manual);
    let changed = text_after_env != text;
    let added = fixes.iter().filter(|f| f.action == "added").count();
    let manual = fixes.iter().filter(|f| f.action == "manual_required").count();

    AutofixOut {
        original_text: text.to_string(),
        fixed_text: text_after_env,
        fixes,
        changed,
        added,
        manual,
    }
}

fn cmd_autofix(rest: &[String]) -> i32 {
    let p = match parse_args(rest, &["file", "text", "format"], &["write"]) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    if !p.positionals.is_empty() {
        usage_fail("autofix 不接受位置参数");
    }
    let format = get_format(&p);
    let file = p.values.get("file").cloned();
    let text = p.values.get("text").cloned();
    let write = p.bools.contains(&"write");

    // text 优先于 file（对表 cli.py cmd_autofix）
    let (json_v, human, exit_code) = if let Some(t) = text {
        let r = autofix_text(&t);
        let code = if r.added == 0 && r.manual == 0 { EXIT_OK } else { EXIT_VIOLATION };
        (autofix_json(&r, None, false), autofix_human(&r, None, false), code)
    } else if let Some(f) = file {
        if !Path::new(&f).exists() {
            out_json(&json!({
                "file": f,
                "error": "file not found",
                "summary": {"added": 0, "manual_required": 0},
            }));
            return EXIT_ERROR;
        }
        let content = match fs::read_to_string(&f) {
            Ok(c) => c,
            Err(err) => {
                out_json(&json!({"error": format!("unexpected: {err}")}));
                return EXIT_ERROR;
            }
        };
        let r = autofix_text(&content);
        let written = write && r.changed;
        if written {
            if let Err(err) = fs::write(&f, &r.fixed_text) {
                out_json(&json!({"error": format!("unexpected: {err}")}));
                return EXIT_ERROR;
            }
        }
        let code = if r.added == 0 && r.manual == 0 { EXIT_OK } else { EXIT_VIOLATION };
        (autofix_json(&r, Some(f.clone()), written), autofix_human(&r, Some(f), written), code)
    } else {
        out_json(&json!({"error": "must provide --file or --text"}));
        return EXIT_ERROR;
    };

    if format == "human" {
        println!("{human}");
    } else {
        out_json(&json_v);
    }
    exit_code
}

/// autofix 结果 JSON（键序对表 Python dict 插入序：original_text → … → file → written）。
fn autofix_json(r: &AutofixOut, file: Option<String>, written: bool) -> Value {
    let fixes: Vec<Value> = r.fixes.iter().map(|f| f.to_value()).collect();
    let mut v = json!({
        "original_text": r.original_text,
        "fixed_text": r.fixed_text,
        "fixes": fixes,
        "changed": r.changed,
        "summary": {"added": r.added, "manual_required": r.manual},
    });
    if let Some(f) = file {
        v["file"] = json!(f);
        v["written"] = json!(written);
    }
    v
}

/// autofix human 形（逐字对表 autofix.format_human；Changed 用 Python bool 形 True/False）。
fn autofix_human(r: &AutofixOut, file: Option<String>, written: bool) -> String {
    let mut lines: Vec<String> = Vec::new();
    if let Some(f) = &file {
        lines.push(format!("File: {f}"));
    }
    let changed_repr = if r.changed { "True" } else { "False" };
    lines.push(format!("Changed: {changed_repr}"));
    lines.push(format!("Auto-fixed: {}, Manual required: {}", r.added, r.manual));
    if written {
        lines.push("Written: yes".to_string());
    } else {
        lines.push("Written: no (check mode)".to_string());
    }
    lines.push(String::new());
    lines.push("Fixes:".to_string());
    for f in &r.fixes {
        let marker = if f.action == "added" { "FIX" } else { "MAN" };
        lines.push(format!("  [{}] {} line {}: {}", marker, f.rule_id, f.line, f.detail));
    }
    if r.fixes.is_empty() {
        lines.push("  (none)".to_string());
    }
    lines.join("\n")
}

// ============ compute.py：数学计算（未移植，落差申报一） ============

fn cmd_compute(rest: &[String]) -> i32 {
    let p = match parse_args(
        rest,
        &["operation", "expression", "var", "lower", "upper", "to", "order", "at", "n", "rows", "format"],
        &[],
    ) {
        Ok(p) => p,
        Err(e) => usage_fail(&e),
    };
    for int_flag in ["order", "n"] {
        if let Some(v) = p.values.get(int_flag) {
            if v.parse::<i64>().is_err() {
                usage_fail(&format!("--{int_flag} 需要整数"));
            }
        }
    }
    // 兼容两种调用形: compute OP EXPR / compute --operation OP --expression EXPR（旗标优先）
    let op = p
        .values
        .get("operation")
        .cloned()
        .or_else(|| p.positionals.first().cloned())
        .filter(|s| !s.is_empty());
    let expr = p
        .values
        .get("expression")
        .cloned()
        .or_else(|| p.positionals.get(1).cloned())
        .filter(|s| !s.is_empty());

    let (op_str, expr_ok) = match (&op, &expr) {
        (Some(o), Some(_)) => (o.clone(), true),
        _ => (String::new(), false),
    };
    if !expr_ok {
        out_json(&json!({
            "error": "operation and expression required (positional or --operation/--expression)",
        }));
        return EXIT_ERROR;
    }
    // SymPy CAS 依赖不在零新增依赖面（bin 头注落差申报一），参数面保留、调用即申报退出码 2。
    out_json(&json!({
        "error": "compute 未移植：围堰 SymPy CAS 依赖不在零新增 Cargo 依赖面（latextool.rs 头注落差申报一）",
        "operation": op_str,
    }));
    EXIT_ERROR
}

// ============ main ============

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.iter().any(|a| a == "--version") {
        println!("latextool {VERSION}");
        exit(EXIT_OK);
    }
    if argv.is_empty() {
        usage_fail("需要子命令");
    }
    let code = match argv[0].as_str() {
        "suggest" => cmd_suggest(&argv[1..]),
        "block-create" => cmd_block_create(&argv[1..]),
        "validate" => cmd_validate(&argv[1..]),
        "autofix" => cmd_autofix(&argv[1..]),
        "compute" => cmd_compute(&argv[1..]),
        "knowledge" => cmd_knowledge(&argv[1..]),
        other => usage_fail(&format!("未知子命令: {other}")),
    };
    exit(code);
}
