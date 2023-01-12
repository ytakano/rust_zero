# 『ゼロから学ぶRust』の正誤表

下記の誤りがありました。お詫びして訂正いたします。

本ページに掲載されていない誤植など間違いを見つけた方は、pull request、またはissueで報告いただけます。

## 第1刷

|頁    | 誤     | 正   |
| ---- | ----- | ---- |
| 2.1節。P.29。図 2.6 | 中央の四角中のテキスト「`data: 40`」 | 「`data: 10`」 |
| 3.3節。P.68。ソースコードの2行目。 | `let b: &i32 = &b;` | `let b: &i32 = &a;` |
| 3.4.2項。P.72。3段落目1行目。 | 可燃参照 | 可変参照 |
| 3.4節。P.75。上のソースコードの下から2行目。 | `*elm * *elm` | `*elm + *elm` |
| 4.4節。P.90。上のソースコードの6行目。 | `&js` | `&yml` |
| 6.3節。P.141。ソースコード。 | 「`return Err(Box::new(..))`」となっている箇所 | このソースコード中でリターンしている箇所では、`Box::new()`は不要です |
| 7.3.4項。P.184。1行目。| `vec![("echo", vec!["hello"]), ("less", vec![])]` | `vec![("echo", vec!["echo", "hello"]), ("less", vec!["less"])]` |
| 7.3.4項。P.196。ソースコード | 「`self.is_group_empty()`」を呼び出している箇所の`unwrap()` | `self.is_group_empty()`には、`unwrap()`は不要です |
| 9.1.4項。P.244。下から1行目、および一番下の証明木の最終行。| `un (lin bool -> lin (un bool * un bool))` | `un (un bool -> lin (un bool * un bool))` |
| 9.3.1項。P.255。`Expr`型 | | `#[derive(Debug)] `が必要です |

### 3.4.2項。P.75。構造体のフィールドを借用しコンパイルエラーとなる例

このページあるコードとエラーの説明は誤りです。

以下のように、`&mut self`と構造体のフィールドへの参照をとるメソッドを定義すると、コンパイルエラーとなります。本ページはこのような場合を想定した説明となります。

```rust
#[derive(Debug)]
struct XY {
    x: Vec<i32>,
    y: Vec<i32>,
}

fn main() {
    let mut xy = XY {
        x: vec![1, 2, 3],
        y: Vec::new(),
    };
    
    xy.update(&xy.x); // コンパイルエラー
    
    println!("{:?}", xy);
}

impl XY {
    fn update(&mut self, x: &[i32]) {
        for elm in x.iter() {
            self.y.push(*elm * *elm);
        }
    }
}
```

### 第6章。正規表現エンジンのVM実装について

`(a*)*`のような正規表現でスタックオーバーフローとなるそうですが、`(a*)*`を`a*`と変換することでスタックオーバーフローを回避可能だそうです。詳細は、下記PRを御覧ください。

- PRs
  - https://github.com/ytakano/rust_zero/pull/4
  - https://github.com/ytakano/rust_zero/pull/7

