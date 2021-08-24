# Chapter04. 所有権を理解する
## 4.2. 参照と借用
関数に任意の値を引数として与えると所有権が値から関数へムーブする。  
呼び出し元に所有権を戻さない場合、関数呼び出し後に値が使えなくなってしまう。  
これを解決するため、所有権をムーブする代わりに引数としてオブジェクトへの参照を取る。  
以下が参照を用いる例。
```Rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-05.svg" width="25%" title="String s1を指す&String sの図表">

`&s1`により、`s1`の値を参照する参照を生成できる。  
所有していないため、参照がスコープを抜けてもドロップされない。

関数の引数でも同様。`&`を使用して引数`s`の型が参照であることを示している。  
所有権はないので、`s`がスコープを抜けても、 参照が指しているものをドロップすることはない。  
関数の引数に参照を取ることを借用と呼ぶ。

借用している何かを変更することはできない。変数が標準で不変なのと全く同様に、参照も不変である。  
(以下コードはエラーとなる)
```Rust
fn main() {
    let s = String::from("hello");
    change(&s);
}
fn change(some_string: &String) {
    some_string.push_str(", world");
}
```