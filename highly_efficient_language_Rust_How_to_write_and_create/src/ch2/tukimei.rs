use::std::collections::HashMap;
fn main() {
    // 旧暦月名の一覧
    let tuki = ["睦月", "如月", "弥生", "卯月", "皐月", "水無月", "文月", "葉月", "長月", "神無月", "霜月", "師走"];
    // HashMap初期化
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    // 月名をHashMapに追加
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i+1);
    }

    // 要素を選んで表示（教科書通り）
    // println!("水無月 = {}月", tuki_map["水無月"]);
    // println!("神無月 = {}月", tuki_map["神無月"]);
    // println!("師走 = {}月", tuki_map["師走"]);

    // 要素を選んで表示（自作した表示関数を使う）
    // tuki_mapをそのまま引数にするとprint_tukiに所有権がムーブしてしまうので&をつけて参照している
    print_tuki("水無月", &tuki_map);
    print_tuki("神無月", &tuki_map);
    print_tuki("師走", &tuki_map);
}

fn print_tuki(tukimei: &str, tuki_map: &HashMap<&str, usize>) {
    // tuki_mapをそのまま引数にするとprint_tukiに所有権がムーブしてしまうので&をつけて参照している
    println!("{} = {}月", tukimei, tuki_map[tukimei]);
}
