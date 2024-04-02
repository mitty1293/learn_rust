use std::env; // コマンドライン引数読み込みのため
use std::fs; // ファイル読み込みのため

fn main() {
    // 引数をベクターとして得る
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるか調べる
    if args.len() < 2 {
        println!("入力ファイルを指定してください。");
        return;
    }
    // 添え字1の要素を得る
    let filename = &args[1];
    // ファイルを読んで表示
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}
