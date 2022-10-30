mod dbg;
mod helper;

use dbg::{State, ZDbg};
use helper::DynError;
use rustyline::{error::ReadlineError, Editor};
use std::env;

fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let msg = format!("引数が必要です\n例 : {} 実行ファイル [引数*]", args[0]);
        return Err(msg.into());
    }

    run_dbg(&args[1])?;
    Ok(())
}

fn run_dbg(filename: &str) -> Result<(), DynError> {
    let debugger = ZDbg::new(filename.to_string());
    let mut state = State::NotRunning(debugger);
    let mut rl = Editor::<()>::new()?;

    loop {
        match rl.readline("zdbg > ") {
            Ok(line) => {
                let trimed = line.trim(); // 行頭と行末の空白文字を削除
                let cmd: Vec<&str> = trimed.split(' ').filter(|c| !c.is_empty()).collect(); // 空文字を削除
                state = match state {
                    State::Running(r) => r.do_cmd(&cmd)?,
                    State::NotRunning(n) => n.do_cmd(&cmd)?,
                    _ => break,
                };
                if let State::Exit = state {
                    break;
                }
                rl.add_history_entry(line);
            }
            Err(ReadlineError::Interrupted) => eprintln!("<<終了はCtrl+D>>"),
            _ => {
                if let State::Running(r) = state {
                    // 子プロセスが実行中の場合はkill
                    r.do_cmd(&["exit"])?;
                };
                break;
            }
        }
    }

    Ok(())
}
