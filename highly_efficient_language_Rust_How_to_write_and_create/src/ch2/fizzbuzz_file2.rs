use std::fs::File;
use std::io::Write;

fn main() {
    // 出力ファイル名を指定
    let filename = "fizzbuzz_file2_result.txt";
    let data = get_fizzbuzz(100);
    // ファイル生成
    let mut fp = File::create(filename).unwrap();
    // ファイルに書き込み
    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap();
}

// FizzBuzzの結果をmaxまで求める
fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();
    for i in 1..=max {
        if (i % 3 == 0) && (i % 5 == 0) {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Fizz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }
    result
}
