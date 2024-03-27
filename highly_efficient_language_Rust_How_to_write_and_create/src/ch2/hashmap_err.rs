// （注意）学習のためエラーをわざと発生させている
use std::collections::HashMap;
fn main() {
    // HashMapを生成して初期化
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // 存在しないキーを取得してエラーを発生させてみる
    let d = map["D"];
    println!("{}", d);
}
