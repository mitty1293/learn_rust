fn main() {
    // Stringオブジェクトを生成してg1に代入
    // オブジェクトの所有者はg1, つまりg1がオブジェクトの所有権を持っている
    let g1 = String::from("穏やかな心は体に良い");
    // Stringオブジェクトの所有権をg1からg2に移動（move）
    let g2 = g1;
    println!("{}", g2);
}
