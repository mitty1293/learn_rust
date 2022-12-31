// 3.5. フロー制御 -> loopでコードを繰り返す
// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// 3.5. フロー制御 -> whileで条件付きループ
// fn main() {
//     let mut number = 3;

//     while number !=0 {
//         println!("{}!", number);
//         number = number - 1;
//     }
//     println!("LIFTOFF!!!");
// }


// 3.5. フロー制御 -> forでコレクションを覗き見る
// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     while index < 5 {
//         println!("the value is: {}", a[index]);
//         index = index + 1;
//     }
// }


// 3.5. フロー制御 -> forでコレクションを覗き見る
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("the value is: {}", element);
//     }
// }


// 3.5. フロー制御 -> forでコレクションを覗き見る
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}