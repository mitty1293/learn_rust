# Chapter05. 構造体を使用して関係のあるデータを構造化する
構造体は、複数の関連した値をまとめ意味のあるグループを形成し、名前付けできる独自のデータ型のこと。  
構造体とenumは自分プログラムで新しい型を定義できる要素である。
## 5.1. 構造体を定義し、インスタンス化する
構造体はタプルと似て一部を異なる型にできる。一方、各フィールドに名前を付けられ、意味が明確になる。  
以下のように定義する。
* `struct`キーワードで構造体全体の名前（ex.`User`）を付ける。
* 波括弧内にデータ片の名前と型を定義する。フィールドと呼ばれる。
```Rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
構造体を定義後に使用するには、具体的な値を指定してインスタンスを生成する。　　
* 構造体名（ex.`User`）を記述する。
* 波括弧内に`フィールド名:格納したい値`のペアを記述する。
* フィールドの順番は定義と異なっても良い。
```Rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```
構造体定義とは 型に対する一般的な雛形であり、インスタンスは雛形を特定のデータで埋め、その型の値を生成すると言える。

構造体インスタンスから値を得るにはドット記法を用いる。（ex.`user1.email`, `user1.username`）  
インスタンスが可変であれば、以下のように特定のフィールドに代入することで値を変更できる。
```Rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
```
このときインスタンス全体が可変でなければならず、Rustにおいて一部のフィールドのみ可変にはできない。

新規の構造体インスタンスを関数内の最後の式として生成し、そのインスタンスを返り値とすることができる。  
以下の関数では`email`と`username`を引数とし、それを値として`User`インスタンスを生成し返す。
```Rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
### フィールドと変数が同名の時にフィールド初期化省略記法を使う
仮引数名と構造体フィールド名が同じ場合、フィールド初期化省略記法が使える。  
例えば以下のように、`email:email`と書かずに`email`と書くだけで済む。
```Rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
### 構造体更新記法で他のインスタンスからインスタンスを生成する
別のインスタンスの値を使用しつつ、一部を変更する形で新しいインスタンスを生成できる。  
`..`という記法により、明示的にセットされていないフィールドが与えられたインスタンスと同じ値になるよう指定できる。
```Rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
インスタンス`user2`では、`email`と`username`は独自の値、それ以外のフィールドは`user1`と同じ値となる。
### 異なる型を生成する名前付きフィールドのないタプル構造体を使用する
フィールド名が無く、フィールドの型だけのタプル構造体を定義することができる。  
タプル全体に名前を付け、他のタプルと異なる型にしたい時に有用である。  
以下のように定義できる。
* `struct`キーワードでタプル構造体全体の名前（ex.`Color`, `Point`）を付ける。
* 括弧内にデータの型を定義する。
```Rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
`black`と`origin`は異なるタプル構造体のインスタンスであるため、型が異なる。  
例えば`Color`型を引数にとる関数は`Point`を引数にとることはできない。  
（たとえ両者の型が3つの`i32`からできていようとも）

タプル構造体のインスタンスはタプルと同様に振る舞う。
* 分配して個々の部品にする
* `.` と添え字を用いて個々の値にアクセスする、等々