//! 哈希工具模块：sha256_hex 公共函数位。
//!
//! 正典指针：DEC-031（hashutil/cliargs 归并决策，评估档 REC-008 登记
//! `DEC-031-hashutil-cliargs-merge.md`）之 hashutil 半；libwave 批施工件。
//!
//! 等价核验在案（libwave 批前置步骤）：13 个 bin（acceptor、cascade、formatter、
//! basemgr、confledger、identity、incubation、lease、gauge、projsnap、locator、
//! parser、tally）本地 sha256_hex 函数体逐对 diff 与 md5 单一指纹
//! （836350a8fb7d6fd70c3135e512ca5c5d ×13）逐字节全等，取公共函数体立此，
//! 行为逐字节保真（同算法同输入同输出）。
//!
//! 零依赖新增：sha2 与 hex 已在 Cargo.toml（DEC-031 前置承诺）。

use sha2::{Digest, Sha256};

/// 计算字节切片的 SHA-256 十六进制小写摘要（原 13 bin 公共函数体原样上收）。
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex::encode(h.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 空输入对表：sha256("") 的既知十六进制值。
    #[test]
    fn empty_input_known_vector() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    /// 非空输入对表：sha256("abc") 的既知十六进制值。
    #[test]
    fn abc_input_known_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
