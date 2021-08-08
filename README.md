# rust_the_book
[The Rust Programming Language](https://doc.rust-jp.rs/book-ja/title-page.html)をやる
## Environment setup
```shell
$ git clone https://github.com/mitty1293/rust_the_book.git
$ sh ./rust_the_book/setup.sh
$ docker exec -it rust_the_book /bin/bash
```
## 3.2. データ型
Rustのデータ型は主に以下。
* スカラー型
    * 整数（ex. `i32`, `u64`, ...）
    * 浮動小数点数（`f32`, `f64`）
    * 論理値（`true`, `false`）
    * 文字（`char`）
        * シングルクォートで表される
* 複合型
    * タプル
        * 各要素の型は同じ型である必要はない
        * 要素へのアクセスは `x.0`, `x.1`,...
        * 分配して各要素を個別の変数に格納可能
        * 長さは固定長。伸ばしも縮めもできない。
    * 配列
        * 各要素の型は同じ型である必要がある
        * 要素へのアクセスは `x[0]`, `x[1]`,...
        * 長さは固定長。伸ばしも縮めもできない。
## 3.3. 関数
* 関数定義において、引数の型は宣言必須