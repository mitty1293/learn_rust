# Chapter06. Enumとパターンマッチング
## 6.1. Enumを定義する
列挙型（`enum`）を用いると、取り得る値をすべて列挙して独自の型を定義できる。  
その値は列挙子（`variant`）のいずれか1つの値を取る。
```Rust
// 定義
enum WebEvent {
    // データ型またはデータが関連付けられていない。
    WELoad,
    // データ型がStringおよびcharである2つのフィールドがある。
    WEKeys(String, char),
    // 名前付きフィールド x と y、およびそれらのデータ型 (i64) を持つ匿名構造体が含まれている。
    WEClick { x: i64, y: i64 }
}
```
`enum`の列挙子には様々なデータ（文字列、数値、構造体、・・）が格納できる。  
列挙子同士の型は異なっても良い。  
`enum`を使わずに表現も可能だが、異なる構造体は各々自身の型があるため。それらをいずれもとる関数を定義することが難しい。  
`enum`を使えば、異なる構造体が単一の`enum`となるため扱いやすい。  
また、構造体のようにメソッド定義も可能。

列挙型の定義で特定のバリアントにアクセスするには、二重コロン `::` を含む構文 `<enum>::<variant>` を使用する。
```Rust
// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick { x: i64, y: i64 }

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent { 
    WELoad(bool), 
    WEClick(MouseClick), 
    WEKeys(KeyPress) 
}
```
```Rust
// インスタンス生成
let we_load = WebEvent::WELoad(true);

// Instantiate a MouseClick struct and bind the coordinate values
let click = MouseClick { x: 100, y: 250 };
// Set the WEClick variant to use the data in the click struct
let we_click = WebEvent::WEClick(click);

// Instantiate a KeyPress tuple and bind the key values
let keys = KeyPress(String::from("Ctrl+"), 'N');
// Set the WEKeys variant to use the data in the keys tuple
let we_key = WebEvent::WEKeys(keys);
```
### Option enumとNull値に勝る利点
Rustには`null`が無い。  
代わりに、値が存在するか不在かを表すenum`Option`がある。  
```Rust
// 標準ライブラリ上のOptionの定義
enum Option<T> {
    Some(T),
    None,
}
```
便利すぎるので明示的に導入しなくても、`Option::`の接頭辞なしでいきなり使える。  
`<T>`は、`Option`の`Some`列挙子が あらゆる型のデータを1つだけ持つことができることを意味する。
```Rust
// Someは値をTに格納して使える。
let some_number = Some(5);
let some_string = Some("a string");

// Noneを指定するときはTの型が必要。
// Someは値が入るので推測可能だが、Noneは値が無いので推測できないから。
let absent_number: Option<i32> = None;
```
`Option<T>`型は、存在する場合と空の場合がある値を操作する際に使う。  
値がnullになる可能性のあるとき、その値の型を`Option<T>`にして明示的に示す。  
そしてnullであった場合を明示的に処理すると、nullでないことを安全に想定できる。
