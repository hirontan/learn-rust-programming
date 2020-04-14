## 6. Enums and Pattern Matching（列挙型とパターンマッチング）

### Defining an Enum
- 列挙型が構造体よりも有用な場合もある
  - 例）IPアドレスにはIPv4とIPv6の主要な標準がある

```
enum IpAddrKind {
    V4,
    V6,
}
```

##### 列挙値
```
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

- 識別子の下に名前空間がある
- 両方の値が同じタイプになっている

```
fn route(ip_kind: IpAddrKind) { }
```

下記のどちらかで呼び出せる
```
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

- 2つのフィールドを持つIpAddr構造体を定義
- `home`や`loopback`にそれぞれ値を読み込んでいる
```
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

- 列挙型だけを使用して、同じ概念をより簡潔に表すことができる
```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

- 列挙型は、関連するデータ型と情報量が異なる場合、有用

```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

- IPアドレスを格納・エンコードすることは、一般的であるため、標準ライブラリで備わっている

```

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

- 列挙型に任意の種類のデータ（文字列、数値型、構造体など）を配置できる

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

- 構造体と列挙型は似ている

```
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

- それぞれがタイプを持つさまざまな構造体を使用した場合、メッセージを受け取る関数を簡単に定義できません
- implを使用してstructにメソッドを定義できるのと同じように、enumにメソッドを定義することもできる

```
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

##### 列挙型オプションとNull値に対するその利点
- 標準ライブラリで定義されている列挙型であるオプション
- 型で表現すると、コンパイラーは処理する必要があるすべてのケースを確認する
- `Rust`にはNullがない
  - Nullは、そこに値がないことを意味する値
  - Nullを使用する言語では、変数は常に2つの状態（nullまたはnot-null）のいずれか

- 「Null References：The Billion Dollar Mistake」で、nullの発明者であるTony Hoareが説明していること
  - 10億ドルの間違い
  - 全てのリファレンスが問題のないことを確認するため、コンパイラでチェックを自動化することが目標であった
  - null参照をこれに含めるのは、非常に簡単であった
  - 数え切れないほどのエラーや脆弱性、システムクラッシュの原因となり、10億ドル単位の損害や苦労を引き起こしてきた

- Rustにはnullはありませんが、存在するまたは存在しない値の概念をエンコードできる列挙型がある

```
enum Option<T> {
    Some(T),
    None,
}
```

- Option <T>列挙型は明示的にスコープに入れる必要はない
- 直接SomeおよびNoneを使用できる

る

```
enum Option<T> {
    Some(T),
    None,
}
```

- Option <T>列挙型は明示的にスコープに入れる必要はない
- 直接SomeおよびNoneを使用できる
- <T>はOption列挙型のSomeが任意の型のデータを1つ保持できることを意味している

```
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

- Option <T>とT（Tは任意の型）は異なる型
- 下記はエラーになる

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

- エラーの出力

```
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

- i8の型の値がある場合、コンパイラーは常に有効な値を持っていることを認識している
  - その値は、nullをチェックする必要がない
- Option <i8>は、値がない可能性も考慮する必要がある
  - Option <T>をTに変換してからT演算を実行する必要がある
  - nullでないものも想定している
- nullの普及を制限し、コードの安全性を高めるという意図的な設計決定
  - nullの可能性がある値を取得するには、その値のタイプをOption <T>にして明示的にオプトインする必要がある
- Option<T> enum

### `match`フロー制御演算子
- `match` フロー制御演算子
  - 一連のパターンに値を比較し、マッチしたパターンに応じてコードを実行する
- `{}`の中をArmという

- 数え上げ装置と同じ要領でコインを一枚取り、 どの種類のコインなのか決定し、その価値を返す関数を示す

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- `if`と`match`の大きな違い
  - `if`：論理値を返す必要がある
  - `match`：どんな型でも良い

```
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

##### 値に結びつけるパターン
- `match`のもう一つの機能、パターンに一致する値の部分に結びつけること
- 列挙型から値を取り出すことができる
- 例) アメリカ50州のクォーターコインのデザインがあった
  - `Quarter`を変更して内部に格納されている`UsState`に値を含めることで、この情報を列挙型に追加できるようにする

```
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

- 50州のコインを集めるとする
  - コインの種類分類
  - 関連づけられている州名を呼び出す
  - コレクションに追加できる
- 例) 上記のようなものを例にする

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

##### `Option<T>`と`match`
- `match`を使って`Option<T>`を扱う

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- `plus_one(five)`が呼び出された時
  - `Some(i) => Some(i + 1),`の処理が使われる
- 2回目の`plus_one`が呼び出された時
  - `None => None,` が呼び出される

##### `match`は包括的
- 一点バグがありコンパイルできないバージョン
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

- Noneの場合を扱っていないため、バグを生む
- ただし、このケースはコンパイラが捕捉できるバグ
- このコードのコンパイルを試みたエラー表記の例
```
error[E0004]: non-exhaustive patterns: `None` not covered
(エラー: 包括的でないパターン: `None`がカバーされてません)
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

- Rustにおける`match`は包括的
  - 全てのあらゆる可能性を網羅し尽くさなければ、コードは有効にならない
  - 特に`Option<T>`の場合、明示的に`None`を処理するのを忘れないようにしてくれる
  - 10億ドルの失敗を犯さないよう

##### `_`というプレイスホルダー
- 全ての可能性を列挙したくない時に使用できるパターン
```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```
- `_`というパターンは、どんな値にもマッチする

### `if let`で簡潔なフロー制御
- 冗長性の少ない方法で組み合わせ、残りを無視しつつ、一つのパターンにマッチする値を扱うことができる
- `Some(3)`の時だけ実行を行う
```
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

- `if let`を用いて短くできる

```
if let Some(3) = some_u8_value {
    println!("three");
}
```

- `if let`
  - メリット：タイプ数が減り、インデントも少なくなり、定型コードも減る
  - デメリット：包括的なチェックができなくなる
    - パターンにマッチした時以外は無視する

- `if let - else`
  - elseに入るコードブロックは、 if letとelseに等価なmatch式の`_`の場合に入るコードブロックと同じ

- `match`の場合
```

let mut count = 0;
match coin {
    // {:?}州のクォーターコイン
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```
- `if let - else`の場合
```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
