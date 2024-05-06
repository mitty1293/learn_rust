// メッセージを生成してその参照を返そうとする関数
fn gen_message() -> String {
    let msg = String::from("過ちを見過ごす人は美しい");
    return msg;
    // gen_messageのスコープを抜けるとmsgは破棄される
    // 破棄される前にmsgの所有権が関数の呼び出し元に移動するのでエラーが出ない
}

fn main() {
    let m = gen_message(); // 所有権はmに移動する
    println!("{}", m);
}
