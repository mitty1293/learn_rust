fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(&g1); // 参照を渡す
    println!("{}", g1); // 所有権は移動していないので使える
}

fn show_message(message: &String) { // 参照を受け取る
    println!("{}", message);
}
