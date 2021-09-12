# Chapter06. Enumとパターンマッチング
列挙型（`enum`）を用いると、取り得る値をすべて列挙して独自の型を定義できる。  
その値は列挙子（`variant`）のいずれか1つの値を取る。
```Rust
// IPアドレスがv4,v6どちらかであることを表すとして・・・
// 定義
enum IpAddrKind {
    V4,
    V6,
}
// インスタンス生成
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```
`IpAddrKind::V4`, `IpAddrKind::V6`はどちらも`IpAddrKind`型である。  
引数に`IpAddrKind`型をとる関数やメソッドを定義できる。

`enum`の列挙子には様々なデータ（文字列、数値、構造体、・・）が格納できる。  
列挙子同士の型は異なっても良い。
```Rust
struct Ipv4Addr {
    // 省略
}
struct Ipv6Addr {
    // 省略
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```
構造体のようにメソッド定義も可能。