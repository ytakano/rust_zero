//! # パフォーマンス計測
//!
//! ## 計測方法
//! a?^n a^nという正規表現を、a^nという文字列にマッチさせる。
//! ただし、a?^nとa^nは、a?とaのn回の繰り返し。
//! 計測は幅優先と深さ優先で行う。
//!
//! ## n = 3の場合の例
//!
//! - 正規表現: a?a?a?aaa
//! - str: aaa
//!
//! ## 実行方法
//!
//! cargo-criterionをインストール後、cargo criterionと実行。
//!
//! ```text
//! $ cargo install cargo-criterion
//! $ cargo criterion
//! ```
//!
//! 実行後は、target/criterion/reports/index.htmlというファイルが生成されるため、
//! それをWebブラウザで閲覧する。
use criterion::{criterion_group, criterion_main, Criterion};
use regex::do_matching;
use std::time::Duration;

/// (計測のid、a?^n a^nという正規表現、文字列)というタプル
const INPUTS: &[(&str, &str, &str)] = &[
    ("n = 2", "a?a?aa", "aa"),
    ("n = 4", "a?a?a?a?aaaa", "aaaa"),
    ("n = 6", "a?a?a?a?a?a?aaaaaa", "aaaaaa"),
    ("n = 8", "a?a?a?a?a?a?a?a?aaaaaaaa", "aaaaaaaa"),
    ("n = 10", "a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa", "aaaaaaaaaa"),
    (
        "n = 12",
        "a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaa",
        "aaaaaaaaaaaa",
    ),
    (
        "n = 14",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaa",
    ),
    (
        "n = 16",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaa",
    ),
    (
        "n = 18",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaaaa",
    ),
    (
        "n = 20",
        "a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaa",
        "aaaaaaaaaaaaaaaaaaaa",
    ),
];

fn depth_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Depth First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| do_matching(args.0, args.1, true))
        });
    }
}

fn width_first(c: &mut Criterion) {
    let mut g = c.benchmark_group("Width First");
    g.measurement_time(Duration::from_secs(12));

    for i in INPUTS {
        g.bench_with_input(i.0, &(i.1, i.2), |b, args| {
            b.iter(|| do_matching(args.0, args.1, false))
        });
    }
}

criterion_group!(benches, width_first, depth_first);
criterion_main!(benches);
