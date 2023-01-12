use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    /// リストを消費して、そのリストの先頭にdataを追加したリストを返す
    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }
}

fn main() {
    // リストを生成
    let list = List::new().cons(1).cons(2).cons(3);

    // JSONにシリアライズ
    let js = serde_json::to_string(&list).unwrap();
    println!("JSON: {} bytes", js.len());
    println!("{js}");

    // YAMLにシリアライズ
    let yml = serde_yaml::to_string(&list).unwrap();
    println!("YAML: {} bytes", yml.len());
    println!("{yml}");

    // MessagePackにシリアライズ
    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());

    // JSONからデシリアライズ
    let list = serde_json::from_str::<List<i32>>(&js).unwrap();
    println!("{:?}", list);

    // YAMLからデシリアライズ
    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}", list);

    // MessagePackからデシリアライズ
    let list = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    println!("{:?}", list);

    write_to_file();
    read_from_file();
}

fn write_to_file() {
    use std::{fs::File, io::prelude::*, path::Path};

    // リストを生成し、YAMLにシリアライズ
    let list = List::new().cons(1).cons(2).cons(3);
    let yml = serde_yaml::to_string(&list).unwrap();

    // ファイルに書き込み
    let path = Path::new("test.yml");
    let mut f = File::create(path).unwrap(); // 新規ファイルを生成
    f.write_all(yml.as_bytes()).unwrap();
}

fn read_from_file() {
    use std::{fs::File, io::prelude::*, path::Path};

    // ファイルからYAML読み込み
    let path = Path::new("test.yml");
    let mut f = File::open(path).unwrap(); // 既存のファイルをオープン
    let mut yml = String::new();
    f.read_to_string(&mut yml).unwrap();

    // YAMLからデシリアライズ
    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("{:?}", list);
}
