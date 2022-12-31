// matchは全可能性を網羅していないとエラー発生
// match内でNoneを扱っていないため包括的エラー発生
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             //None => None,
//             Some(i) => Some(i + 1),
//         }
//     }
    
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
// }

// "u8"は0から255までを取るが、以下ではプレースホルダ― "_" が1,3,5,7以外にマッチしてくれる。
//  "_ => ()," をコメントアウトすると1,3,5,7以外の可能性を網羅していないのでエラーが発生する
// つまり、matchはOption<T>に限らず網羅していないパターンがあるとエラーが発生する
fn main() {
    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
