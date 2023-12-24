// // Rustで九九の表（行末のカンマを削ったもの）
fn main() {
    for y in 1..10 {
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}
