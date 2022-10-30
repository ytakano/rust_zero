mod helper;
mod parser;
mod typing;

use nom::error::convert_error;
use std::{env, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数の検査
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("以下のようにファイル名を指定して実行してください\ncargo run codes/ex1.lin");
        return Err("引数が不足".into());
    }

    // ファイル読み込み
    let content = fs::read_to_string(&args[1])?;

    let ast = parser::parse_expr(&content); // パース
    println!("AST:\n{:#?}\n", ast);
    match ast {
        Ok((_, expr)) => {
            let mut ctx = typing::TypeEnv::new();
            println!("式:\n{content}");

            // 型付け
            let a = typing::typing(&expr, &mut ctx, 0)?;
            println!("の型は\n{a}\nです。");
        }
        Err(nom::Err::Error(e)) => {
            let msg = convert_error(content.as_str(), e);
            eprintln!("パースエラー:\n{msg}");
            return Err(msg.into());
        }
        _ => (),
    }

    Ok(())
}
