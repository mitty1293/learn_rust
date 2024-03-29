// 配列をシャッフルするのに必要な宣言
use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数値を配列に代入
    let mut nums = [0; 75];
    for i in 1..=75 {
        nums[i-1] = i;
    }

    // シャッフル
    let mut rng = rand::thread_rng(); // 乱数生成器のオブジェクト作成
    nums.shuffle(&mut rng);

    // カードを表示
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                print!(" *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
