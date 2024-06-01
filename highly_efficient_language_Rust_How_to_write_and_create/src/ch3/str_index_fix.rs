fn main() {
    let s = "こんにちは";
    // 先頭の1文字を取り出して表示
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch);
    // 0から数えて2文字目を表示
    let ch = s.chars().nth(2).unwrap();
    println!("{}", ch);
}
