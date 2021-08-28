# Chapter04. 所有権を理解する
## 4.3. スライス型
所有権を持たない方法として参照があるが、同様に所有権を持たないデータ型としてスライスがある。  
コレクション全体ではなく、その内の一部の要素を参照することができる。
### 文字列スライス
文字列スライスとは`String`の一部への参照であり（全体への参照ではない）、以下のように生成される。
```Rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```
`[starting_index..ending_index]`と指定することでスライスを生成できる。
* starting_index: スライスの最初の位置
* ending_index: スライスの終端位置よりも1大きい値（starting_index + スライスの長さ）

内部的にスライスは、開始地点と長さを保持している。  
`let world = &s[6..11];`では、`world`は`s`の（1から数えて）7バイト目へのポインタと5という長さを保持する。

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-06.svg" width="25%" title="文字列スライス">