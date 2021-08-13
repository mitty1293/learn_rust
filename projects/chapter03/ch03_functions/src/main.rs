// 3.3. 関数
// fn main() {
//     println!("Hello, world!");
//     another_finction();
// }

// fn another_finction() {
//     println!("Another function.");
// }


// 3.3. 関数 -> 関数の引数
// fn main() {
//     another_finction(5, 6);
// }

// fn another_finction(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }


// 3.3. 関数 -> 関数本体は、文と式を含む
// 以下はコンパイルできない。let y = 6 という文は値を返さないためxに束縛するものがないため。
// fn main() {
//     let x = (let y = 6);
// }


// 3.3. 関数 -> 関数本体は、文と式を含む
// fn main() {
//     let x = 5;
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {}", y);
// }


// 3.3. 関数 -> 戻り値のある関数
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();
//     println!("The value of x is: {}", x);
// }


// 3.3. 関数 -> 戻り値のある関数
fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}