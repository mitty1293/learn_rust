// 3.3. 関数
// fn main() {
//     println!("Hello, world!");
//     another_finction();
// }

// fn another_finction() {
//     println!("Another function.");
// }


// 3.3. 関数 -> 関数の引数
fn main() {
    another_finction(5, 6);
}

fn another_finction(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}