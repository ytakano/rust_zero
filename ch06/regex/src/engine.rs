//! 正規表現エンジン
mod codegen;
mod evaluator;
mod parser;

use crate::helper::DynError;
use std::fmt::{self, Display};

/// 命令列
#[derive(Debug)]
pub enum Instruction {
    Char(char),
    Match,
    Jump(usize),
    Split(usize, usize),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Char(c) => write!(f, "char {}", c),
            Instruction::Match => write!(f, "match"),
            Instruction::Jump(addr) => write!(f, "jump {:>04}", addr),
            Instruction::Split(addr1, addr2) => write!(f, "split {:>04}, {:>04}", addr1, addr2),
        }
    }
}

/// 正規表現をパースしてコード生成し、
/// ASTと命令列を標準出力に表示。
///
/// # 利用例
///
/// ```
/// use regex;
/// regex::print("abc|(de|cd)+");
/// ```
///
/// # 返り値
///
/// 入力された正規表現にエラーがあったり、内部的な実装エラーがある場合はErrを返す。
pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let ast = parser::parse(expr)?;
    println!("AST: {:?}", ast);

    println!();
    println!("code:");
    let code = codegen::get_code(&ast)?;
    for (n, c) in code.iter().enumerate() {
        println!("{:>04}: {c}", n);
    }

    Ok(())
}

/// 正規表現と文字列をマッチング。
///
/// # 利用例
///
/// ```
/// use regex;
/// regex::do_matching("abc|(de|cd)+", "decddede", true);
/// ```
///
/// # 引数
///
/// exprに正規表現、lineにマッチ対象とする文字列を与える。
/// is_depthがtrueの場合は深さ優先探索を、falseの場合は幅優先探索を利用。
///
/// # 返り値
///
/// エラーなく実行でき、かつマッチングに**成功**した場合はOk(true)を返し、
/// エラーなく実行でき、かつマッチングに**失敗**した場合はOk(false)を返す。
///
/// 入力された正規表現にエラーがあったり、内部的な実装エラーがある場合はErrを返す。
pub fn do_matching(expr: &str, line: &str, is_depth: bool) -> Result<bool, DynError> {
    let ast = parser::parse(expr)?;
    let code = codegen::get_code(&ast)?;
    let line = line.chars().collect::<Vec<char>>();
    Ok(evaluator::eval(&code, &line, is_depth)?)
}
