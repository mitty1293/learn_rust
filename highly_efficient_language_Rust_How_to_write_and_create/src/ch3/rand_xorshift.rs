use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 乱数を初期化
    let mut seed = rand_init();
    // 100個の乱数を表示
    for _ in 0..100 {
        // 乱数を生成
        let v = rand(&mut seed, 1, 6);
        println!("{}", v);
    }
}

// 乱数を初期化する関数
fn rand_init() -> u32 {
    // 現在時刻を利用して乱数を初期化
    SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_millis() as u32
}

// start から end の乱数を生成する関数
// 引数seedは変更可能な参照なので, 毎回値が更新される. rand関数内で値が更新される.
fn rand(seed: &mut u32, start: u32, end: u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    return *seed % (end - start + 1) + start;
}
