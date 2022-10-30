//! # 第一見出し
//!
//! テキストを書く。
//!
//! ## 第二見出し
//!
//! ### 第三見出し
//!
//! - 箇条書き1
//! - 箇条書き2
//!
//! 1. 番号付きリスト1
//! 2. 番号付きリスト2
//!
//! > 引用
//! > 文字列
//!
//! [KSPUB](https://www.kspub.co.jp/)
//!
//! `println!("Hello, world!");`
//!
//! ```
//! println!("Hello, world!");
//! ```
//!

mod my_module {
    //! これはモジュールのドキュメントです。
    //!
    //! # 利用例
}

/// my_funcは私独自の関数です。
///
/// # 利用例
///
/// ```
/// use markdown::my_func;
/// let n = my_func().unwrap();
/// ```
pub fn my_func() -> Option<u32> {
    Some(100)
}

/// nの一つ前の数字を返す
/// nが0の場合はNoneを返す
pub fn pred(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n - 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_func() {
        assert_eq!(my_func(), Some(100));
    }

    #[test]
    #[should_panic]
    fn test_pred() {
        pred(0).unwrap();
    }
}
