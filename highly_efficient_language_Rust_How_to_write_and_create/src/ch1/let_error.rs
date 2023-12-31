// 問題のあるプログラム
fn main() {
    let a = 100;
    a = a + 1;
    println!("a is {}", a)
}

// 以下のエラーが出る
// error[E0384]: cannot assign twice to immutable variable `a`
//  --> let_error.rs:3:5
//   |
// 2 |     let a = 100;
//   |         -
//   |         |
//   |         first assignment to `a`
//   |         help: consider making this binding mutable: `mut a`
// 3 |     a = a + 1;
//   |     ^^^^^^^^^ cannot assign twice to immutable variable

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0384`.