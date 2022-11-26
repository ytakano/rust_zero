//! # 線形型言語のパーサ
//!
//! λ計算に線形型システムを適用した独自の線形型言語のパーサ。
//! 独自言語の構文は以下を参照。
//!
//! ## 構文
//!
//! ```text
//! <VAR>   := 1文字以上のアルファベットから成り立つ変数
//!
//! <E>     := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL>
//!
//! <LET>   := let <VAR> : <T> = <E>; <E>
//! <IF>    := if <E> { <E> } else { <E> }
//! <SPLIT> := split <E> as <VAR>, <VAR> { <E> }
//! <FREE>  := free <E>; <E>
//! <APP>   := ( <E> <E> )
//!
//! <Q>     := lin | un
//!
//! 値
//! <QVAL>  := <Q> <VAL>
//! <VAL>   := <B> | <PAIR> | <FN>
//! <B>     := true | false
//! <PAIR>  := < <E> , <E> >
//! <FN>    := fn <VAR> : <T> { <E> }
//!
//! 型
//! <T>     := <Q> <P>
//! <P>     := bool |
//!            ( <T> * <T> )
//!            ( <T> -> <T> )
//! ```

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, multispace1},
    error::VerboseError,
    sequence::delimited,
    IResult,
};
use std::fmt;

/// 抽象構文木
///
/// ```text
/// <E> := <LET> | <IF> | <SPLIT> | <FREE> | <APP> | <VAR> | <QVAL>
/// ```
#[derive(Debug)]
pub enum Expr {
    Let(LetExpr),     // let式
    If(IfExpr),       // if式
    Split(SplitExpr), // split式
    Free(FreeExpr),   // free文
    App(AppExpr),     // 関数適用
    Var(String),      // 変数
    QVal(QValExpr),   // 値
}

/// 関数適用
///
/// ```text
/// <APP> := ( <E> <E> )
///
/// (expr1 expr2)
/// ```
#[derive(Debug)]
pub struct AppExpr {
    pub expr1: Box<Expr>,
    pub expr2: Box<Expr>,
}

/// if式
///
/// ```text
/// <IF> := if <E> { <E> } else { <E> }
///
/// if cond_expr {
///     then_expr
/// } else {
///     else_expr
/// }
/// ```
#[derive(Debug)]
pub struct IfExpr {
    pub cond_expr: Box<Expr>,
    pub then_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

/// split式
///
/// ```text
/// <SPLIT> := split <E> as <VAR>, <VAR> { <E> }
///
/// split expr as left, right {
///     body
/// }
/// ```
#[derive(Debug)]
pub struct SplitExpr {
    pub expr: Box<Expr>,
    pub left: String,
    pub right: String,
    pub body: Box<Expr>,
}

/// let式
///
/// ```text
/// <LET>   := let <VAR> : <T> = <E> { <E> }
///
/// let var : ty = expr1 { expr2 }
/// ```
#[derive(Debug)]
pub struct LetExpr {
    pub var: String,
    pub ty: TypeExpr,
    pub expr1: Box<Expr>,
    pub expr2: Box<Expr>,
}

/// 値。真偽値、関数、ペア値などになる
///
/// ```text
/// <VAL>  := <B> | <PAIR> | <FN>
/// <B>    := true | false
/// <PAIR> := < <E> , <E> >
/// <FN>   := fn <VAR> : <T> { <E> }
/// ```
#[derive(Debug)]
pub enum ValExpr {
    Bool(bool),                 // 真偽値リテラル
    Pair(Box<Expr>, Box<Expr>), // ペア
    Fun(FnExpr),                // 関数（λ抽象）
}

/// 修飾子
///
/// ```text
/// <Q> := lin | un
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Qual {
    Lin, // 線形型
    Un,  // 制約のない一般的な型
}

/// 修飾子付き値
///
/// ```
/// <QV> := <Q> <VAL>
/// ```
#[derive(Debug)]
pub struct QValExpr {
    pub qual: Qual,
    pub val: ValExpr,
}

/// 関数
///
/// ```text
/// <FN> := fn <VAR> : <T> { <E> }
///
/// fn var : ty { expr }
/// ```
#[derive(Debug)]
pub struct FnExpr {
    pub var: String,
    pub ty: TypeExpr,
    pub expr: Box<Expr>,
}

/// free文
///
/// ```text
/// <FREE> := free <E>; <E>
///
/// free var; expr
/// ```
#[derive(Debug)]
pub struct FreeExpr {
    pub var: String,
    pub expr: Box<Expr>,
}

/// 修飾子付き型
///
/// ```text
/// <QV> := <Q> <VAL>
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TypeExpr {
    pub qual: Qual,
    pub prim: PrimType,
}

impl fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.qual == Qual::Lin {
            write!(f, "lin {}", self.prim)
        } else {
            write!(f, "un {}", self.prim)
        }
    }
}

/// プリミティブ型
///
/// ```text
/// <P> := bool |
///        ( <T> * <T> )
///        ( <T> -> <T> )
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PrimType {
    Bool,                                // 真偽値型
    Pair(Box<TypeExpr>, Box<TypeExpr>),  // ペア型
    Arrow(Box<TypeExpr>, Box<TypeExpr>), // 関数型
}

impl fmt::Display for PrimType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimType::Bool => write!(f, "bool"),
            PrimType::Pair(t1, t2) => write!(f, "({t1} * {t2})"),
            PrimType::Arrow(t1, t2) => write!(f, "({t1} -> {t2})"),
        }
    }
}

pub fn parse_expr(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace0(i)?;
    let (i, val) = alt((alpha1, tag("(")))(i)?;

    match val {
        "let" => parse_let(i),
        "if" => parse_if(i),
        "split" => parse_split(i),
        "free" => parse_free(i),
        "lin" => parse_qval(Qual::Lin, i),
        "un" => parse_qval(Qual::Un, i),
        "(" => parse_app(i),
        _ => Ok((i, Expr::Var(val.to_string()))),
    }
}

/// 関数適用をパース。
fn parse_app(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace0(i)?;
    let (i, e1) = parse_expr(i)?; // 適用する関数

    let (i, _) = multispace1(i)?;

    let (i, e2) = parse_expr(i)?; // 引数

    let (i, _) = multispace0(i)?;
    let (i, _) = char(')')(i)?;

    Ok((
        i,
        Expr::App(AppExpr {
            expr1: Box::new(e1),
            expr2: Box::new(e2),
        }),
    ))
}

/// free文をパース。
fn parse_free(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, var) = alpha1(i)?; // 解放する変数
    let (i, _) = multispace0(i)?;
    let (i, _) = char(';')(i)?;

    let (i, e) = parse_expr(i)?; // 続けて実行する式
    Ok((
        i,
        Expr::Free(FreeExpr {
            var: var.to_string(),
            expr: Box::new(e),
        }),
    ))
}

/// split式をパース。
fn parse_split(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, e1) = parse_expr(i)?; // 分解するペア

    let (i, _) = multispace1(i)?;
    let (i, _) = tag("as")(i)?;
    let (i, _) = multispace1(i)?;

    let (i, v1) = parse_var(i)?; // 一つめの変数

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, v2) = parse_var(i)?; // 二つめの変数
    let (i, _) = multispace0(i)?;

    // { <E> }というように、波括弧で囲まれた式をパース
    let (i, e2) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    Ok((
        i,
        Expr::Split(SplitExpr {
            expr: Box::new(e1),
            left: v1,
            right: v2,
            body: Box::new(e2),
        }),
    ))
}

/// if式をパース。
fn parse_if(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, e1) = parse_expr(i)?; // 条件
    let (i, _) = multispace0(i)?;

    // 条件が真の時に実行する式
    let (i, e2) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    let (i, _) = multispace0(i)?;
    let (i, _) = tag("else")(i)?;
    let (i, _) = multispace0(i)?;

    // 条件が偽の時に実行する式
    let (i, e3) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    Ok((
        i,
        Expr::If(IfExpr {
            cond_expr: Box::new(e1),
            then_expr: Box::new(e2),
            else_expr: Box::new(e3),
        }),
    ))
}

/// let式をパース。
fn parse_let(i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;

    let (i, var) = parse_var(i)?; // 束縛する変数

    let (i, _) = multispace0(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, ty) = parse_type(i)?; // 変数の型

    let (i, _) = multispace0(i)?;
    let (i, _) = char('=')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, e1) = parse_expr(i)?; // 変数の値
    let (i, _) = multispace0(i)?;

    let (i, _) = char(';')(i)?;
    let (i, e2) = parse_expr(i)?; // 実行する式

    Ok((
        i,
        Expr::Let(LetExpr {
            var,
            ty,
            expr1: Box::new(e1),
            expr2: Box::new(e2),
        }),
    ))
}

/// ペアをパース。
fn parse_pair(i: &str) -> IResult<&str, ValExpr, VerboseError<&str>> {
    let (i, _) = multispace0(i)?;

    let (i, v1) = parse_expr(i)?; // 一つめの値

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, v2) = parse_expr(i)?; // 二つめの値

    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?; // 閉じ括弧

    Ok((i, ValExpr::Pair(Box::new(v1), Box::new(v2))))
}

/// linとun修飾子をパース。
fn parse_qual(i: &str) -> IResult<&str, Qual, VerboseError<&str>> {
    let (i, val) = alt((tag("lin"), tag("un")))(i)?;
    if val == "lin" {
        Ok((i, Qual::Lin))
    } else {
        Ok((i, Qual::Un))
    }
}

/// 関数をパース。
fn parse_fn(i: &str) -> IResult<&str, ValExpr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, var) = parse_var(i)?; // 引数

    let (i, _) = multispace0(i)?;
    let (i, _) = char(':')(i)?;
    let (i, _) = multispace0(i)?;

    let (i, ty) = parse_type(i)?; // 引数の型
    let (i, _) = multispace0(i)?;

    // { <E> }というように、波括弧で囲まれた式をパース
    let (i, expr) = delimited(
        char('{'),
        delimited(multispace0, parse_expr, multispace0),
        char('}'),
    )(i)?;

    Ok((
        i,
        ValExpr::Fun(FnExpr {
            var,
            ty,
            expr: Box::new(expr),
        }),
    ))
}

/// 真偽値、関数、ペアの値をパース。
fn parse_val(i: &str) -> IResult<&str, ValExpr, VerboseError<&str>> {
    let (i, val) = alt((tag("fn"), tag("true"), tag("false"), tag("<")))(i)?;
    match val {
        "fn" => parse_fn(i),
        "true" => Ok((i, ValExpr::Bool(true))),
        "false" => Ok((i, ValExpr::Bool(false))),
        "<" => parse_pair(i),
        _ => unreachable!(),
    }
}

/// 修飾子付き値をパース。
fn parse_qval(q: Qual, i: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let (i, _) = multispace1(i)?;
    let (i, v) = parse_val(i)?;

    Ok((i, Expr::QVal(QValExpr { qual: q, val: v })))
}

/// 変数をパース。変数は1文字以上のアルファベットから成り立つ。
fn parse_var(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    let (i, v) = alpha1(i)?;
    Ok((i, v.to_string()))
}

/// 真偽値、関数、ペア型をパース。
fn parse_type(i: &str) -> IResult<&str, TypeExpr, VerboseError<&str>> {
    let (i, q) = parse_qual(i)?; // 修飾子
    let (i, _) = multispace1(i)?;
    let (i, val) = alt((tag("bool"), tag("(")))(i)?;
    if val == "bool" {
        // bool型
        Ok((
            i,
            TypeExpr {
                qual: q,
                prim: PrimType::Bool,
            },
        ))
    } else {
        // 関数型かペア型
        let (i, _) = multispace0(i)?;
        let (i, t1) = parse_type(i)?; // 一つめの型
        let (i, _) = multispace0(i)?;

        // ->か*をパース
        // ->の場合は関数型で、場合はペア型
        let (i, op) = alt((tag("*"), tag("->")))(i)?;

        let (i, _) = multispace0(i)?;
        let (i, t2) = parse_type(i)?; // 二つめの型
        let (i, _) = multispace0(i)?;

        let (i, _) = char(')')(i)?;

        Ok((
            i,
            TypeExpr {
                qual: q,
                prim: if op == "*" {
                    PrimType::Pair(Box::new(t1), Box::new(t2))
                } else {
                    PrimType::Arrow(Box::new(t1), Box::new(t2))
                },
            },
        ))
    }
}
