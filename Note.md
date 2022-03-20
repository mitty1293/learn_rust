# Cargo を使用してプロジェクトを作成して管理する
[Rust の最初のステップ / Rust とは / Rust 固有の機能](https://docs.microsoft.com/ja-jp/learn/modules/rust-introduction/3-rust-features)
- `cargo new` コマンドを使用して、新しいプロジェクト テンプレートを作成する。
- `cargo build` コマンドを使用して、プロジェクトをビルドする。
- `cargo run` コマンドを使用して、プロジェクトをビルドして実行する。
- `cargo test` コマンドを使用して、プロジェクトをテストする。
- `cargo check` コマンドを使用して、プロジェクトの種類を確認する。
- `cargo doc` コマンドを使用して、プロジェクトのドキュメントをビルドする。
- `cargo publish` コマンドを使用して、crates.io にライブラリを発行する。
- Cargo.toml ファイルにクレート名を追加して、依存クレートをプロジェクトに追加する。Rustのマニフェストファイル。プロジェクトのメタデータとすべての依存関係をここに保存する。
# コードインデント
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