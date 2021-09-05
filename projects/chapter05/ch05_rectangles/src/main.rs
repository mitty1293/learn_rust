// // 5.2. 構造体を使ったプログラム例 -> 単一の変数を用いて長方形の面積を求める例
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of th rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // 5.2. 構造体を使ったプログラム例 -> タプルでリファクタリングする
// 幅と高さは関連性があるためグループ化する
// グループ化はされたがデータの意味が不明瞭になってしまった
// fn main () {
//     let rect1 = (30, 50);
//     println!(
//         "The area of th rectangle is {} square pixels.",
//         area(rect1)
//     );
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // 5.2. 構造体を使ったプログラム例 -> 構造体でリファクタリングする
// タプルはスタックに保持されるが、構造体は所有権を奪わずに借用する方が良い（ref.chapter04）
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {width: 30, height: 50};
//     println!(
//         "The area of th rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// // 5.2. 構造体を使ったプログラム例 -> トレイトの導出で有用な機能を追加する
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    // rect1は{}です
    println!("rect1 is {:#?}", rect1);
}