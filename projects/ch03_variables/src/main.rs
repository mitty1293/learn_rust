// 3.1. 変数と可変性
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }


// 3.1. 変数と可変性 -> シャドーイング
// fn main() {
//     let x = 5;
    
//     let x = x + 1;

//     let x = x * 2;

//     println!("The value of x is: {}", x);
// }


// 3.2. データ型 -> 浮動小数点型
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }


// 3.2. データ型 -> 数値演算
// fn main() {
//     // 足し算
//     let sum = 5 + 10;
//     println!("sum = {}", sum);
//     // 引き算
//     let difference = 95.5 - 4.3;
//     println!("difference = {}", difference);
//     // 掛け算
//     let product = 4 * 30;
//     println!("product = {}", product);
//     // 割り算
//     let quotient = 56.7 / 32.2;
//     println!("quotient = {}", quotient);
//     // 余り
//     let remainder = 43 % 5;
//     println!("remainder = {}", remainder);
// }


// 3.2. データ型 -> 論理値型
// fn main() {
//     let t = true;
//     println!("t = {}", t);

//     let f: bool = false;
//     println!("f = {}", f);
// }


// 3.2. データ型 -> 文字型
// fn main() {
//     let c = 'z';
//     println!("c = {}", c);
//     let z = 'ℤ';
//     println!("z = {}", z);
//     let heart_eyed_cat = '😻';
//     println!("heart_eyed_cat = {}", heart_eyed_cat);
// } 


// 3.2. データ型 -> タプル型
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    
}