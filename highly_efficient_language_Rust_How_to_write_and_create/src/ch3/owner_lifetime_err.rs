// 問題のあるプログラム
// メッセージを生成してその参照を返そうとする関数
fn gen_message() -> &str {
    let msg = String::from("過ちを見過ごす人は美しい");
    return &msg;
    // gen_messageのスコープを抜けるとmsgは破棄されるので, msgの値を参照できない
}

fn main() {
    let m = gen_message();
    println!("{}", m);
}
