use crate::{helper::safe_add, parser};
use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap, mem};

type VarToType = BTreeMap<String, Option<parser::TypeExpr>>;

/// 型環境
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TypeEnv {
    env_lin: TypeEnvStack, // lin用
    env_un: TypeEnvStack,  // un用
}

impl TypeEnv {
    pub fn new() -> TypeEnv {
        TypeEnv {
            env_lin: TypeEnvStack::new(),
            env_un: TypeEnvStack::new(),
        }
    }

    /// 型環境をpush
    fn push(&mut self, depth: usize) {
        self.env_lin.push(depth);
        self.env_un.push(depth);
    }

    /// 型環境をpop
    fn pop(&mut self, depth: usize) -> (Option<VarToType>, Option<VarToType>) {
        let t1 = self.env_lin.pop(depth);
        let t2 = self.env_un.pop(depth);
        (t1, t2)
    }

    /// 型環境へ変数と型をpush
    fn insert(&mut self, key: String, value: parser::TypeExpr) {
        if value.qual == parser::Qual::Lin {
            self.env_lin.insert(key, value);
        } else {
            self.env_un.insert(key, value);
        }
    }

    /// linとunの型環境からget_mutし、depthが大きい方を返す
    fn get_mut(&mut self, key: &str) -> Option<&mut Option<parser::TypeExpr>> {
        match (self.env_lin.get_mut(key), self.env_un.get_mut(key)) {
            (Some((d1, t1)), Some((d2, t2))) => match d1.cmp(&d2) {
                Ordering::Less => Some(t2),
                Ordering::Greater => Some(t1),
                Ordering::Equal => panic!("invalid type environment"),
            },
            (Some((_, t1)), None) => Some(t1),
            (None, Some((_, t2))) => Some(t2),
            _ => None,
        }
    }
}

/// 型環境のスタック
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct TypeEnvStack {
    vars: BTreeMap<usize, VarToType>,
}

impl TypeEnvStack {
    fn new() -> TypeEnvStack {
        TypeEnvStack {
            vars: BTreeMap::new(),
        }
    }

    // 型環境をpush
    fn push(&mut self, depth: usize) {
        self.vars.insert(depth, BTreeMap::new());
    }

    // 型環境をpop
    fn pop(&mut self, depth: usize) -> Option<VarToType> {
        self.vars.remove(&depth)
    }

    // スタックの最も上にある型環境に変数名と型を追加
    fn insert(&mut self, key: String, value: parser::TypeExpr) {
        if let Some(last) = self.vars.iter_mut().next_back() {
            last.1.insert(key, Some(value));
        }
    }

    // スタックを上からたどっていき、はじめに見つかる変数の型を取得
    fn get_mut(&mut self, key: &str) -> Option<(usize, &mut Option<parser::TypeExpr>)> {
        for (depth, elm) in self.vars.iter_mut().rev() {
            if let Some(e) = elm.get_mut(key) {
                return Some((*depth, e));
            }
        }
        None
    }
}

type TResult<'a> = Result<parser::TypeExpr, Cow<'a, str>>;

/// 型付け関数
/// 式を受け取り、型を返す
pub fn typing<'a>(expr: &parser::Expr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    match expr {
        parser::Expr::App(e) => typing_app(e, env, depth),
        parser::Expr::QVal(e) => typing_qval(e, env, depth),
        parser::Expr::Free(e) => typing_free(e, env, depth),
        parser::Expr::If(e) => typing_if(e, env, depth),
        parser::Expr::Split(e) => typing_split(e, env, depth),
        parser::Expr::Var(e) => typing_var(e, env),
        parser::Expr::Let(e) => typing_let(e, env, depth),
    }
}

/// 関数適用の型付け
fn typing_app<'a>(expr: &parser::AppExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    // 関数部分
    let t1 = typing(&expr.expr1, env, depth)?;
    let t_arg;
    let t_ret;
    match t1.prim {
        parser::PrimType::Arrow(a, b) => {
            t_arg = a; // 引数の型
            t_ret = b; // 返り値の型
        }
        _ => return Err("関数型でない".into()),
    }

    // 引数部分
    let t2 = typing(&expr.expr2, env, depth)?;

    // 引数の型が一致しているかチェック
    if *t_arg == t2 {
        Ok(*t_ret)
    } else {
        Err("関数適用時における引数の型が異なる".into())
    }
}

/// 修飾子付き値の型付け
fn typing_qval<'a>(expr: &parser::QValExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    // プリミティブ型を計算
    let p = match &expr.val {
        parser::ValExpr::Bool(_) => parser::PrimType::Bool,
        parser::ValExpr::Pair(e1, e2) => {
            // 式e1とe2をtypingにより型付け
            let t1 = typing(e1, env, depth)?;
            let t2 = typing(e2, env, depth)?;

            // expr.qualがUnであり、
            // e1か、e2の型にlinが含まれていた場合、型付けエラー
            if expr.qual == parser::Qual::Un
                && (t1.qual == parser::Qual::Lin || t2.qual == parser::Qual::Lin)
            {
                return Err("un型のペア内でlin型を利用している".into());
            }

            // ペア型を返す
            parser::PrimType::Pair(Box::new(t1), Box::new(t2))
        }
        parser::ValExpr::Fun(e) => {
            // 関数の型付け

            // un型の関数内では、lin型の自由変数をキャプチャできないため
            // lin用の型環境を置き換え
            let env_prev = if expr.qual == parser::Qual::Un {
                Some(mem::take(&mut env.env_lin))
            } else {
                None
            };

            // depthをインクリメントしてpush
            let mut depth = depth;
            safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;
            env.push(depth);
            env.insert(e.var.clone(), e.ty.clone());

            // 関数中の式を型付け
            let t = typing(&e.expr, env, depth)?;

            // スタックをpopし、popした型環境の中にlin型が含まれていた場合、型付けエラー
            let (elin, _) = env.pop(depth);
            for (k, v) in elin.unwrap().iter() {
                if v.is_some() {
                    return Err(format!("関数定義内でlin型の変数\"{k}\"を消費していない").into());
                }
            }

            // lin用の型環境を復元
            if let Some(ep) = env_prev {
                env.env_lin = ep;
            }

            // 関数型を返す
            parser::PrimType::Arrow(Box::new(e.ty.clone()), Box::new(t))
        }
    };

    // 修飾子付き型を返す
    Ok(parser::TypeExpr {
        qual: expr.qual,
        prim: p,
    })
}

/// free式の型付け
fn typing_free<'a>(expr: &parser::FreeExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    if let Some((_, t)) = env.env_lin.get_mut(&expr.var) {
        if t.is_some() {
            *t = None;
            return typing(&expr.expr, env, depth);
        }
    }
    Err(format!(
        "既にfreeしたか、lin型ではない変数\"{}\"をfreeしている",
        expr.var
    )
    .into())
}

/// if式の型付け
fn typing_if<'a>(expr: &parser::IfExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    let t1 = typing(&expr.cond_expr, env, depth)?;
    // 条件の式の型はbool
    if t1.prim != parser::PrimType::Bool {
        return Err("ifの条件式がboolでない".into());
    }

    let mut e = env.clone();
    let t2 = typing(&expr.then_expr, &mut e, depth)?;
    let t3 = typing(&expr.else_expr, env, depth)?;

    // thenとelse部の型は同じで、
    // thenとelse部評価後の型環境は同じかをチェック
    if t2 != t3 || e != *env {
        return Err("ifのthenとelseの式の型が異なる".into());
    }

    Ok(t2)
}

/// split式の型付け
fn typing_split<'a>(expr: &parser::SplitExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    if expr.left == expr.right {
        return Err("splitの変数名が同じ".into());
    }

    let t1 = typing(&expr.expr, env, depth)?;
    let mut depth = depth;
    safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;

    match t1.prim {
        parser::PrimType::Pair(p1, p2) => {
            env.push(depth);
            // ローカル変数の型を追加
            env.insert(expr.left.clone(), *p1);
            env.insert(expr.right.clone(), *p2);
        }
        _ => {
            return Err("splitの引数がペア型でない".into());
        }
    }

    let ret = typing(&expr.body, env, depth);

    // ローカル変数を削除
    let (elin, _) = env.pop(depth);

    // lin型の変数を消費しているかチェック
    for (k, v) in elin.unwrap().iter() {
        if v.is_some() {
            return Err(format!("splitの式内でlin型の変数\"{k}\"を消費していない").into());
        }
    }

    ret
}

/// 変数の型付け
fn typing_var<'a>(expr: &str, env: &mut TypeEnv) -> TResult<'a> {
    let ret = env.get_mut(expr);
    if let Some(it) = ret {
        // 定義されている
        if let Some(t) = it {
            // 消費されていない
            if t.qual == parser::Qual::Lin {
                // lin型
                let eret = t.clone();
                *it = None; // linを消費
                return Ok(eret);
            } else {
                return Ok(t.clone());
            }
        }
    }

    Err(format!(
        "\"{}\"という変数は定義されていないか、利用済みか、キャプチャできない",
        expr
    )
    .into())
}

/// let式の型付け
fn typing_let<'a>(expr: &parser::LetExpr, env: &mut TypeEnv, depth: usize) -> TResult<'a> {
    // 変数束縛
    let t1 = typing(&expr.expr1, env, depth)?;
    // 束縛変数の型をチェック
    if t1 != expr.ty {
        return Err(format!("変数\"{}\"の型が異なる", expr.var).into());
    }

    // 関数内
    let mut depth = depth;
    safe_add(&mut depth, &1, || "変数スコープのネストが深すぎる")?;
    env.push(depth);
    env.insert(expr.var.clone(), t1); // 変数の型をinsert
    let t2 = typing(&expr.expr2, env, depth)?;

    // lin型の変数を消費しているかチェック
    let (elin, _) = env.pop(depth);
    for (k, v) in elin.unwrap().iter() {
        if v.is_some() {
            return Err(format!("let式内でlin型の変数\"{k}\"を消費していない").into());
        }
    }

    Ok(t2)
}
