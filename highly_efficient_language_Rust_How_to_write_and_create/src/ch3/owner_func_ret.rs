fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1); // 所有権が移動する
    println!("{}", g1); // 所有権を戻したのでg1は使える
}

fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}
