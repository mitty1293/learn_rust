// num_bigintクレートのBigIntを利用する宣言
use num_bigint::BigInt;

fn main() {
    // 数値1234を表すBigintのオブジェクトを作る
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678));
}
