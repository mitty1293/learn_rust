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
- 式(expression): 値を返す
    - ex. `5+6`, `6`, 関数呼び出し, マクロ呼び出し, `{}`
    - 終端にセミコロンを含まない
- 文(statement): 値を返さない
    - 終端にセミコロンを含む
-  開始コード ステートメントは、左余白からスペース 4 つ分インデントされます。
- コードがセミコロンで終わっていない場合、開始ステートメントが完了する前に次のコード行を実行する必要があることが認識されます。実行する次のコード行は、さらにスペース 4 つ分インデントされます。
# todo! マクロ
[Rust の最初のステップ / Rust プログラムを初めて作成する / Rust プログラムの基本的な構造を理解する](https://docs.microsoft.com/ja-jp/learn/modules/rust-create-program/1-program-structure)
- Rust のマクロは、可変個の入力引数を受け取る関数のようなものです。
- `todo!` マクロは、Rust プログラムで未完了のコードを識別するために使用されます。
    - このマクロは、プロトタイプを作成するときや、完了していない動作を示す場合に便利です。
# println! マクロ
- `println!` マクロには、画面または "`println!`" に表示される 1 つ以上の入力引数が必要です。
-  `println!` マクロにより、テキスト文字列内の中かっこ `{}` の各インスタンスが、リスト内の次の引数の値に置き換えられます。
```Rust
fn main() {
    // Call println! with three arguments: a string, a value, a value
    println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
}
```