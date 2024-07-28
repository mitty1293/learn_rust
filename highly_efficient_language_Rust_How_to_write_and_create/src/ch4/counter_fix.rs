// Counter構造体
struct Counter {
    value: i64,
}
impl Counter {
    fn new() -> Self {
        Counter { value: 0 }
    }
    fn inc(&mut self) {
        self.value += 1;
        println!("value={}", self.value);
    }
}

// Counter構造体を引数に取る関数
fn count(counter: Option<&mut Counter>) {
    match counter {
        None => return,
        Some(c) => c.inc(),
    };
}

fn main() {
    // Counterオブジェクトを引数にする
    let mut a = Counter::new();
    count(Some(&mut a));
    count(Some(&mut a));
    // Noneを引数にする
    let a = None;
    count(a);
}
