# Cargoコマンド
[Rust の最初のステップ / Rust とは / Rust 固有の機能](https://docs.microsoft.com/ja-jp/learn/modules/rust-introduction/3-rust-features)
- `cargo new` コマンドを使用して、新しいプロジェクト テンプレートを作成する。
- `cargo build` コマンドを使用して、プロジェクトをビルドする。
- `cargo run` コマンドを使用して、プロジェクトをビルドして実行する。
- `cargo test` コマンドを使用して、プロジェクトをテストする。
- `cargo check` コマンドを使用して、プロジェクトの種類を確認する。
- `cargo doc` コマンドを使用して、プロジェクトのドキュメントをビルドする。
- `cargo publish` コマンドを使用して、crates.io にライブラリを発行する。
- Cargo.toml ファイルにクレート名を追加して、依存クレートをプロジェクトに追加する。Rustのマニフェストファイル。プロジェクトのメタデータとすべての依存関係をここに保存する。
# 式と文
[Rust の最初のステップ / Rust プログラムを初めて作成する / Rust プログラムの基本的な構造を理解する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/1-program-structure)
- 式(expression): 返り値を評価する。値を返す。
    - ex. `5+6`, `6`, 関数呼び出し, マクロ呼び出し, `{}`
    - 終端にセミコロンを含まない
- 文(statement): 処理を実行するが値を返さない。
    - 終端にセミコロンを含む
-  開始コード ステートメントは、左余白からスペース 4 つ分インデントされます。
- コードがセミコロンで終わっていない場合、開始ステートメントが完了する前に次のコード行を実行する必要があることが認識されます。実行する次のコード行は、さらにスペース 4 つ分インデントされます。
# todo! マクロ
[Rust の最初のステップ / Rust プログラムを初めて作成する / Rust プログラムの基本的な構造を理解する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/1-program-structure)
- Rust のマクロは、可変個の入力引数を受け取る関数のようなものです。
- `todo!` マクロは、Rust プログラムで未完了のコードを識別するために使用されます。
    - このマクロは、プロトタイプを作成するときや、完了していない動作を示す場合に便利です。
# println! マクロ
[Rust の最初のステップ / Rust プログラムを初めて作成する / Rust プログラムの基本的な構造を理解する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/1-program-structure)
- `println!` マクロには、画面または "`println!`" に表示される 1 つ以上の入力引数が必要です。
-  `println!` マクロにより、テキスト文字列内の中かっこ `{}` の各インスタンスが、リスト内の次の引数の値に置き換えられます。
```Rust
fn main() {
    // Call println! with three arguments: a string, a value, a value
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
}
```
# 変数のシャドウ処理
[Rust の最初のステップ / Rust プログラムを初めて作成する / Rust で変数を作成して使用する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/2-variables)
- 既存の変数の名前を使って新しい変数を宣言できる。
- 前の変数は引き続き存在するが、このスコープでは参照できなくなる。（シャドウされる）
- 以下ではまず`shadow_num`を宣言し、新しく`shadow_num`を宣言する際に前の変数バインドをシャドウしている。新しい`shadow_num`が作成されている。
```Rust
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num" 
let shadow_num = shadow_num + 5; 

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2; 

println!("The number is {}.", shadow_num);
```
- 以下はシャドウ処理ではないが、同じ結果をエラー無く出力できる。
```Rust
fn main() {
    let mut shadow_num = 5;

    shadow_num = shadow_num + 5; 

    shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);
}
```
# テキスト: 文字と文字列
[Rust の最初のステップ / Rust プログラムを初めて作成する / 数値、テキスト、true/false 値のデータ型を調べる](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/3-basic-data-types)
https://zenn.dev/toga/books/rust-atcoder/viewer/23-string
- Rustでは、2つの基本的な文字列型と1つの文字型がサポートされている。
- 文字は 1 つの項目で、文字列は一連の文字である。
## 文字型
### `char`型
- 項目を単一引用符で囲んで指定する。
```Rust
let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😃';
```
## 文字列型
### `str`型(文字列リテラル)
- 文字列処理において，スライス `[T]` に相当する型
- 項目を二重引用符""で囲んで指定する。
- `&str`のような参照を用いることがほとんど。
- コンパイル時に中身が判明している必要があり、変更できない文字列データへのポインタと考えることができる。
- 簡単にいうと、プログラムの実行時に変更されないテキスト データの不変ビュー。

### `String`型
- 文字列処理において，ベクタ `Vec<T>` に相当する型
- ヒープに割り当てられる。
- コンパイル時に文字数を把握しておく必要がない。
- 簡単にいうと、プログラムの実行時に変更される可能性があるテキストデータ。
# デバッグステートメント
[Rust の最初のステップ / Rust プログラムを初めて作成する / 複合データに列挙型バリアントを使用する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/5-enum-variants)
- `#[derive(Debug)]` 構文を使用すると、コードの実行中に、標準出力では見ることのできない特定の値を確認できる。
- `println!` マクロでデバッグ データを表示するには、構文 `{:#?}` を使用して、読み取り可能な方法でデータを書式設定する。
# 関数
- Rustでの一般的な方法では、関数の最後のコード行を返す値と同じにすることで、関数の終了時に値を返す。
```Rust
fn divide_by_5(num: u32) -> u32 {
    num / 5
    // a = num/5
    // return a
    // とかしなくて良い。返す値を最終行にしておくだけで良い。
    // 式なのでセミコロンを付与してはいけない。付与すると値が評価されずに返らなくなってしまう。
}

fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
```
# 配列
メモリに連続して格納された同じ型のオブジェクトのコレクション
## 特徴
- すべての要素は同じデータ型。(タプルは異なる型でも良い)
- サイズは固定。長さが変更されることはない。
- 要素の値だけが変更される可能性がある。
## 参照
- array[0], array[1]のようにインデックス番号で要素に参照可能。
- 長さ以上のインデックスを指定するとコンパイルエラー
## 定義
```Rust
// 値のコンマ区切りで定義可能
let days = ["Sunday", "Monday", "Tuesday"];

// [初期値; 長さ] で定義可能
let bytes = [0; 5];

// シグネチャ[T; size]で定義可能
let array: [i64; 5];
```
# ベクター
配列と同様に同じ型の複数の値を格納するコレクション
## 特徴
- すべての要素は同じデータ型。
- サイズはいつでも拡大縮小可能。
## 要素の追加削除
push(value), pop(value)メソッドで末尾の追加削除が可能。  
異なる型の要素を追加しようとするとエラー。
```Rust
// 末尾に追加
fruit.push("Apple");
// 末尾を削除
fruit.pop()
```
## 参照と要素の変更
- `vec[0]`, `vec[1]`のようにインデックス番号で要素に参照可能。
- `index_vec[1] = index_vec[1] + 5;` のようにインデックスでアクセスして値の変更が可能。
- 長さ以上のインデックスを指定するとコンパイルはできるが実行時にパニックする。
## 定義
一般的には`vec!`マクロを使用して定義する。
```Rust
// 値のコンマ区切りで定義可能
let three_nums = vec![15, 3, 46];
println!("Initial vector: {:?}", three_nums);  

// [初期値; 長さ] で定義可能
let zeroes = vec![0; 5];
println!("Zeroes: {:?}", zeroes); 

// 空ベクターの作成
let mut fruit = Vec::new();
let mut car = Vec::<i32>::new()

// シグネチャVec<T>で定義可能
let v: Vec<u32>;
```
## 型
```Rust
// データ型が不明なときはジェネリック型Tを用いる
let v: Vec<T>;
// 型がわかるときには以下のように既知の型。
Vec<u32>
Vec<String>
```