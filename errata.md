# 『ゼロから学ぶRust』の正誤表

下記の誤りがありました。お詫びして訂正いたします。

本ページに掲載されていない誤植など間違いを見つけた方は、pull request、またはissueで報告いただけます。

## 第1刷

|頁    | 誤     | 正   |
| ---- | ----- | ---- |
| 2.1.6項。P.22。1行目。 | 4つの配列を持つ配列の値を定義できる | 4つの値を持つ配列を定義できる |
| 2.1節。P.29。図 2.6。 | 中央の四角中のテキスト「`data: 40`」 | 「`data: 10`」 |
| 2.2節。P.54。表 2.6。 `it.next()` | 次の要素へのイテレータを返す | 次の要素を返す |
| 2.2節。P.54。表 2.6。 `it.last()` | 最後の要素へのイテレータを返す | 最後の要素を返す |
| 2.2節。P.54。表 2.6。 `it.nth(&n)` | n番目の要素へのイテレータを返す | n番目の要素を返す |
| 3.3節。P.68。ソースコードの2行目。 | `let b: &i32 = &b;` | `let b: &i32 = &a;` |
| 3.4.2項。P.72。3段落目1行目。 | 可燃参照 | 可変参照 |
| 3.4節。P.75。上のソースコードの下から2行目。 | `*elm * *elm` | `*elm + *elm` |
| 4.3節。P.88。図 4.3。 | `{"top": {"left": {"left": "left", "right": "leaf}, "right": {"left: "leaf", "right": "leaf"}}` | `{"top": {"left": {"left": "left", "right": "leaf"}, "right": {"left": "leaf", "right": "leaf"}}}` |
| 4.4節。P.90。上のソースコードの6行目。 | `&js` | `&yml` |
| 6.3節。P.141。ソースコード。 | 「`return Err(Box::new(..))`」となっている箇所 | このソースコード中でリターンしている箇所では、`Box::new()`は不要です |
| 7.3.4項。P.184。1行目。| `vec![("echo", vec!["hello"]), ("less", vec![])]` | `vec![("echo", vec!["echo", "hello"]), ("less", vec!["less"])]` |
| 7.3.4項。P.196。ソースコード | 「`self.is_group_empty()`」を呼び出している箇所の`unwrap()` | `self.is_group_empty()`には、`unwrap()`は不要です |
| 9.1.4項。P.240~241。型に対するUn述語とLin述語の例。| `Un(un bool) = true`<br>`Un(lin <lin bool, lin bool>) = false`<br>`Lin(un bool) = true`<br>`Lin(un <un bool, un bool>) = true`<br>`Lin(lin <un bool, lin bool>) = true` | `Un(un bool) = true`<br>`Un(lin (lin bool * lin bool)) = false`<br>`Lin(un bool) = true`<br>`Lin(un (un bool * un bool)) = true`<br>`Lin(lin (un bool * lin bool)) = true` |
| 9.1.4項。P.241。型環境に対するUn述語とLin述語の例。| `Un(x:un <un bool, un bool>, y:lin bool)`<br>`= Un(un <un bool, un bool>) ∧ Un(un bool)`<br>`= true`<br>`Un(x:un bool, y:un bool)`<br>`= Un(un bool) ∧ Un(lin bool)`<br>`= false` | `Un(x:un (un bool * un bool), y:un bool)`<br>`= Un(un (un bool * un bool)) ∧ Un(un bool)`<br>`= true`<br>`Un(x:un bool, y:lin bool)`<br>`= Un(un bool) ∧ Un(lin bool)`<br>`= false` |
| 9.1.4項。P.244。下から1行目、および一番下の証明木の最終行。| `un (lin bool -> lin (un bool * un bool))` | `un (un bool -> lin (un bool * un bool))` |
| 9.3.1項。P.255。`Expr`型 | | `#[derive(Debug)] `が必要です |

### 3.4.2項。P.75。構造体のフィールドを借用しコンパイルエラーとなる例

このページあるコードとエラーの説明は誤りです。
正しい説明を以下に記載します。

構造体フィールドの値に対して何らかの定型処理を行いたい場合があります。
たとえば、`XY`という構造体の`selector`の値に応じて、
`XY::x`か`XY::y`を変更したいとします。
具体的には以下のようなコードとなります。

```rust
#[derive(Debug)]
struct XY {
    x: Vec<i32>,
    y: Vec<i32>,
    selector: bool,
    scaler: i32,
}

fn main() {
    let mut xy = XY {
        x: vec![1, 2, 3],
        y: vec![4, 5, 6],
        selector: true,
        scaler: 3,
    };
    
    let v = xy.get_vec();
    xy.update(v); // `xy`は借用されているためコンパイルエラー
    
    println!("{:?}", xy);
}

impl XY {
    /// `selector`の応じて、`x`か`y`を返す
    fn get_vec(&mut self) -> &mut [i32] {
        if self.selector {
            &mut self.x
        } else {
            &mut self.y
        }
    }

    /// `v`になんらかの定型処理を行う
    fn update(&mut self, v: &mut [i32]) {
        for elm in v.iter_mut() {
            *elm *= self.scaler;
        }
    }
}
```

ここでは、`get_vec()`メソッドが`selector`の値に応じて`x`か`y`への可変参照を返しています。
このコードは単純ですが、実際にはもっと複雑な処理の結果として`x`か`y`が返されると考えてください。
やりたいことは、`get_vec()`で返された値に対して何らかの定型処理、ここでは`update()`メソッドを適用を行うとします。

このような処理の場合、以下のように、`get_vec`メソッドと`update`メソッドを利用したくなります。

```rust
let mut xy = XY { /* 省略 */ };
let v = xy.get_vec();
xy.update(v); // `xy`は借用されているためコンパイルエラー
```

しかし、このコードはコンパイルエラーとなります。
なぜなら、`get_vec()`で返される可変参照が`v`に借用されているため、`update()`で必要な`&mut self`が借用できないからです。
これを解決するために、分配束縛などが利用できます。

### 第6章。正規表現エンジンのVM実装について

`(a*)*`のような正規表現でスタックオーバーフローとなるそうですが、`(a*)*`を`a*`と変換することでスタックオーバーフローを回避可能だそうです。詳細は、下記PRを御覧ください。

- PRs
  - https://github.com/ytakano/rust_zero/pull/4
  - https://github.com/ytakano/rust_zero/pull/7

