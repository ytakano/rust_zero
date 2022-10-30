use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

fn main() {
    // 美術館を初期化
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
    gallery.insert("ミュシャ", "黄道十二宮");

    // RwLockとArcを利用して共有可能に
    let gallery = Arc::new(RwLock::new(gallery));

    let mut hdls = Vec::new(); // joinハンドラ
    for n in 0..3 {
        // 客を表すスレッドを生成
        let gallery = gallery.clone(); // 参照カウンタをインクリメント
        let hdl = std::thread::spawn(move || {
            for _ in 0..8 {
                {
                    let guard = gallery.read().unwrap(); // リードロック
                    if n == 0 {
                        // 美術館の内容を表示
                        for (key, value) in guard.iter() {
                            print!("{key}:{value}, ");
                        }
                        println!();
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });
        hdls.push(hdl);
    }

    // 美術館スタッフ
    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            // 展示内容入れ替え
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap(); // ライトロック
                guard.clear();
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                let mut guard = gallery.write().unwrap(); // ライトロック
                guard.clear();
                guard.insert("葛飾北斎", "富嶽三十六景 神奈川沖浪裏");
                guard.insert("ミュシャ", "黄道十二宮");
            }
            sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();
}
