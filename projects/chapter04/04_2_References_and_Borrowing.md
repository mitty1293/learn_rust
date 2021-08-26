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
### 可変な参照
借用している変数を変更したい場合、
* 変数宣言時に`mut`で可変なことを宣言し、  
* `&mut`で可変な参照を生成し、
* `some_string: &mut String`で引数に可変な参照を受け入れる
```Rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
可変な参照の制約として、
* 特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てない
    * データ競合を防ぐため
    ```Rust
    // 以下はsを可変として2回借用しようとしておりエラーになる
    // r1前後に{}を配置して、r1がスコープを抜けた後ならr2で可変な参照を持てる。
    let r1 = &mut s;
    let r2 = &mut s;
    ```
* 不変な参照をしている間は可変な参照をすることはできない
    * 不変な参照の使用者は、それ以降に値が変わることを予想していないため
    * 複数の不変参照は可能。データ読み込みに対して影響は与えないため。
    ```Rust
    let mut s = String::from("hello");
    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！
    ```
### 宙に浮いた参照
Rustではダングリングポインタが発生しないよう保証してくれる。
* ダングリングポインタとは
    * 無効なメモリ領域を差すポインタ
    * あるメモリ領域が無効化されたにも関わらず、その領域を参照し続けているポインタ

以下コードを実行すると、コンパイラはエラーを発生しダングリングを未然に防いでくれる。
```Rust
fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { // dangleはStringへの参照を返す
    let s = String::from("hello"); // sは新しいString
    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 呼び出し元（reference_to_nothing）は、sが解放されたことを知らずにsへの参照を差し続けてしまう。
```
解決策は、以下のように`String`を直接返せばよい。  
所有権は呼び出し元（reference_to_nothing）にムーブし、何も解放されることは無い。
```Rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

### まとめ
* 任意のタイミングで、1つの可変参照 **か** 不変な参照いくつでも の**どちらか** を行える。