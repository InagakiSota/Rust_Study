# Rustで学んだことをここに書いてく
---
## 型
### 数値型
`型+サイズ`で構成される
- `i○○`：符号あり整数型（i8,i16,i32,i64,i128）
- `u○○`：符号なし整数型（u8,u16,u32,u64,u128）
- `f○○`：浮動小数点数型(**f32**と**f64**のみ)
</br>

### 文字列型
- `str`：文字列スライス[^1]
- `String`：標準ライブラリで定義されている文字列型

どちらもUTF-8でエンコードされた文字列データを格納している
strとStringで相互変換ができる
[^1]: スライスとは、メモリ上に存在する文字列データのスタート地点と長さをしめすもの
``` Rust
let s1: String = String::from("Hollow Word");
let s2: &str = &s1;                 // String -> str
let s3: String = s2.to_string();    // str -> String
```
</br> 

### タプル
異なる方を収めることができる集合
関数から複数の値を返す際にタプルで返すことがある
内部にアクセスする際は **.0** や **.1** のようにドットと番号で指定する
``` Rust
let mut t = (1, "hello");
t.0 = 2;
t.1 = "world";
```
</br>

### 配列
C++とおおむね同じ
`変数名: [型, 要素数]` で宣言する
``` Rust
let mut a: [i32, 3] = [0, 1, 2];
let b: [i32, 3] = [0, 3];           // 配列の中を0で埋める
a[1] = b[1];
```
</br>

### 構造体（Struct）
C++と概ね同じ　インスタンス時の初期化が若干クセあり
``` Rust
// 構造体の宣言
struct Person {
    name: String,
    age: i32,
}

// 初期化
let p = Person {
    name: String::from("Kisida"),
    age: 40,
}

```
</br>

### 列挙型（Enum）
列挙子にさらにデータを付与することができる
付与するデータや型構造はまったく違うものを指定できる（わけわからん）
``` Rust
// 宣言
enum Event {
    Quit, 
    KeyDow(u8),
    MouseDown(x : i32, y: i32),
}

let e1 = Event:Quit;
let e2 = Event:MouseDown(13,12);
```
</br>

### Option
データが存在する場合と、存在しない場合を表現できる列挙型
``` Rust
// 標準ライブラリ内の定義
pub enum Option<T> {
    None,
    Some(T),
}
```
データが存在しないときは`None`、存在するときは `Some(T)` と表現されている(???)
``` Rust
fn get_value(v: bool) -> Option<usize> {
    if v {
        Some(100)
    } else {
        None
    }
}

fn main() {
    let mut result = 0;

    // get_value()の返り値Some(100)を受け取って出力している
    // つまり出力は"Success: 100"
    match get_value(true) {
        Some(result) => println!("Success: {}", result),
        None => println!("failure"),
    }
}
```
</br>

### Result
処理の結果が成功か失敗かを表現できる列挙体
``` Rust
// 標準ライブラリ内の定義
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
処理が成功した場合は、任意の型Tのデータを持つ`Ok(T)`を使う
処理が失敗した場合は、任意の型Eのデータを持つ`Err(E)`を使う
**Result<i32, String>** にすると、成功時は数値型、失敗時は文字列を取得できる

#### パターンマッチ
Result型を取得した際には、**match**や**if let**を使いパターンマッチで処理するのが一般的
``` Rust
let result: Result<i32, String> = Ok(200);

// resultがOk(~)という構造をしているときに、
// ～に該当する部分をローカル変数codeに束縛してブロック内で使用する
// こういった記述の仕方を"パターンマッチ"と呼ぶらしい
match result {
    Ok(code) => println!("code: {}", code),
    Err(err) => println!("Err: {}", err),
};
```
``` rust
let resutl: Result<i32, String> = Ok(200);

if let Ok(code) = result {
    println!("code: {}", code);
}
```

</br>

#### unwrap_or()
↑の書き方だとネストが深くなって冗長
`Ok()`の場合はそのまま展開し、`Err()`の場合は引数で与えた値を返す
似たもので、`unwarp_or_else()` もある
``` rust
let result: Result<i32, String> = Ok(200);
println!("code: {}", result.unwrap_or(-1)); // => "code: 200"
let result: Result<i32, String> = Err("Error".to_string());
println!("code: {}", result.unwrap_or(-1)); // => "code: Error"
```
</br>

#### and_then
Ok()だった場合に指定の関数を実行する
``` Rust
fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}

let result: Result<i32, String> = Ok(200);
let next_result = result.and_then(func);    // func()は実行される
let result: Result<i32, String> = Err("Error");
let next_result = result.and_then(func);    // func()は実行されない
```
</br>

#### ?演算子
Ok()だった場合は値を展開、Err()だった場合はErr()をそのままreturnする
主に関数内で使用される　エラーが起きた時に、そのエラーを関数内で処理せずに呼び出し元にエラー処理を委譲させる際に有効
``` rust
fn error_handling(result: Result<i32, String>) -> Result<i32, Strting> {
    let code = result?;     // エラーの場合はここで return result;
    println!("code: {}", code);
    Ok(100)
}
```
</br>

#### lety-els構文
最初に`let`による束縛を試し、マッチしなかった場合に`else`の処理に流れていく
`else`の処理の最後では、`break` `return` `panic!`などので処理を中断させる必要がある



Resultにはさまざまな**シンタックスシュガー**[^2]が用意されている
[^2]:複雑でわかりにくい書き方を、意味はそのままにより簡単に書けるようにする構文










