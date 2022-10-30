//! 実行すると5GB以上のメモリを消費するため注意すること。
//! メモリが足りない場合は、NUMの値を減らしてください。

const NUM: usize = 200000000; // 生成する乱数の合計数

/// xorshift
struct XOR64 {
    x: u64,
}

impl XOR64 {
    fn new(seed: u64) -> XOR64 {
        XOR64 {
            x: seed ^ 88172645463325252,
        }
    }

    /// 乱数生成関数
    fn next(&mut self) -> u64 {
        let x = self.x;
        let x = x ^ (x << 13);
        let x = x ^ (x >> 7);
        let x = x ^ (x << 17);
        self.x = x;
        return x;
    }
}

fn main() {
    single_threaded();
    multi_threaded();
}

/// 乱数値を要素に持つVecを生成
fn randomized_vec() -> (Vec<u64>, Vec<u64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    let mut generator = XOR64::new(1234);

    // 疑似乱数生成
    for _ in 0..NUM {
        let x1 = generator.next();
        let x2 = generator.next();
        v1.push(x1);
        v2.push(x2);
    }

    (v1, v2)
}

fn single_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now(); // 開始時間

    v1.sort(); // 順番にソート
    v2.sort();

    let end = start.elapsed(); // 経過時間
    println!(
        "single_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}

fn multi_threaded() {
    let (mut v1, mut v2) = randomized_vec();

    let start = std::time::Instant::now(); // 開始時間

    // スレッドを生成してソート
    let handler1 = std::thread::spawn(move || {
        v1.sort();
        v1
    });
    let handler2 = std::thread::spawn(move || {
        v2.sort();
        v2
    });
    let _v1 = handler1.join().unwrap();
    let _v2 = handler2.join().unwrap();

    let end = start.elapsed(); // 経過時間
    println!(
        "multi_threaded: {}.{:03}秒",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}
