# learn-rust-programming

## 1. Getting Started
- `Mac`で実行

### Installing rustup on Linux or macOS
```
$ curl https://sh.rustup.rs -sSf | sh
  > 選択：1) Proceed with installation (default)
```

### パスにRustを追加
```
$ source $HOME/.cargo/env
```

### Updating and Uninstalling
##### Update
```
$ rustup update
```

##### Unintall
```
$ rustup self uninstall
```

### Rustが正しくインストールされているか確認
```
$ rustc --version
```

### Hello, World!

##### ディレクトリ作成
```
$ mkdir hello_world
$ cd hello_world
````

##### プログラムの作成と実行
- ファイルの拡張子：`.rs`
- ファイル名に複数の単語を扱う場合は、アンダースコアを利用して区切る


今回の作成は`main.rs`で作成
```
fn main() {
    println!("Hello, world!");
}
```
- Rustのスタイルは、4つのスペースでインデントする
- `println！`は、マクロ。関数を呼び出した場合、`println`として入力

コンパイルと実行
```
$ rustc main.rs
$ ./main
```

##### 自動フォーマッターの実行
- 空白を削除してみて `--check` オプションで変更されたことを確認しています
```
$ rustfmt --check main.rs
Diff in /Users/hironori/work/learn-rust-programming/hello_world/main.rs at line 1:
 fn main() {
- println!("Hello, world!");
+    println!("Hello, world!");
 }
```

### Hello, Cargo!
- Cargo：Rust のビルドシステムおよびパッケージマネージャー
  - コードに依存関係が必要なライブラリを呼び出す

##### Cargo がインストールされているか確認
```
$ cargo --version
```

##### Cargo を利用したプロジェクト作成
```
$ cargo new hello_cargo
$ cd hello_cargo
```

##### TOMLファイルをみる
```
$ cat Cargo.toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["hirontan <hirontan@sora.flights>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

- package：ステートメント
  - 名前、バージョン、作成者、使用するエディションの記載
- dependencies：プロジェクトの依存関係リスト
  - コードのパッケージはクレートと呼ばれる

##### Cargo について補足
- ソースファイルが`src`ディレクトリ内にある
- 最上位のプロジェクトディレクトリは、READMEファイル、ライセンス情報、構成ファイル、およびコードに関係のないその他のものである
- Cargoを利用しないプロジェクトで始めた場合、ソースコードを`src`ディレクトリに置き直し、TOMLファイルを作成する

##### ビルドと実行
```
$ cargo build
```

- `target/debug/hello_cargo`に実行ファイルができる

```
$ ./target/debug/hello_cargo
```

- `cargo build` を実行すると、Cargo.lock ファイルができる
  - 依存関係を記載してくれる

##### もう一つの実行方法
- ソースコードをコンパイルし、実行ファイルをすべて1つのコマンドで行う方法もある

```
$ cargo run
```

- ファイルが変更されていないと判断されると、コンパイルが実行されない

##### コンパイルをして、実行ファイルを生成なし
```
$ cargo check
```

##### リリース用ビルド
```
$ cargo build --release
```

- `target/release` 配下にファイルができる
- 最適化によりコード実行が高速になるかわりに、コンパイルに時間がかかる
- 実行時間のベンチマークをする場合、`target/release`の実行ファイルを実行する

## 2. Programming a Guessing Game

### 概要
- 一般的なRustの基礎を学ぶ
  - let
  - match
  - methods
  - 関連する機能
  - 外部クレートの使用
    - など
- 推測ゲームを実装
  - 1〜100のランダムな整数を生成
  - プレーヤーが数値を入力して、生成された数値よりもは数値が高いか低いかを掲示
  - 生成された数値と入力した数値が等しければ、お祝いのメッセージを出力して終了

### コード作成

##### プロジェクト作成
```
$ cargo new guessing_game
$ cd guessing_game
```

##### プレイヤーの入力を実装
- `src/main.rs` の編集
```
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

- io ライブラリをスコープにいれる
  - 標準ライブラリの`std`に入っている
- useステートメントを利用して、ライブラリをスコープにいれる
- `let mut guess = String::new();`：変数を格納する場所を作成
  - `mut`をつけることで、可変変数と、明示できる
- `use std::io;` の記述がなければ、`std::io::stdin()` で呼び出せる
- `.read_line(&mut guess)` で利用されている `&`は、この引数が参照であると示す。
  - コードの複数の部分が1つのデータにアクセスできるようになり、そのデータを複数回メモリにコピーする必要がなくなる。

##### ここまでで実行
```
$ cargo run
   Compiling guessing_game v0.1.0 (........./work/learn-rust-programming/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
1
You guessed: 1
```
##### 1〜100の乱数を発生させる
- `rand` クレートを利用する

Cargo.toml の`dependencies`セクションにかきを記述
```
[dependencies]
rand = "0.5.5"
```

ビルドを行う
```
$ cargo build
```

最新バージョンのクレートを見つけて、インストール
```
$ cargo update
```

もし、`0.6.0` か `0.6.x` のバージョンを利用する場合は、下記のようにかく
```
[dependencies]
rand = "0.6.0"
```

- 乱数を発生するコードを`main.rs`に記述`

```
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

- 実行してみる
```
$ cargo run
   Compiling guessing_game v0.1.0 (.............../work/learn-rust-programming/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 53
Please input your guess.
2
You guessed: 2
```

##### 入力番号と比較できるように
- `std::cmp::Ordering`をスコープにいれる
  - `Ordering`は`Result`と同様、列挙型

- `main.rs`を編集
```
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- 一度実行してみるがエラーになる
  - `guess` は `String`型、`secret_number` は数値`i32`型（デフォルト）であるから、比較ができない
```
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `std::string::String`, found integer
   |
   = note: expected reference `&std::string::String`
              found reference `&{integer}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game`.

To learn more, run the command again with --verbose.
```

- 下記に変更する
```
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

- 実行してみる
```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 43
Please input your guess.
1
You guessed: 1
Too small!
```

##### Loopにしてみる
```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

##### 正しい推測ができれば終了する
```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

##### 無効な入力処理をする
- `Result`は`Ok`・`Err`を持つ列挙型
  - `num`は数値であった場合
  - `_`は全て処理する
```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

### クレートのドキュメントを開く
```
$ cargo doc --open
```

## 3. Common Programming Concepts

### 変数と可変性

##### 準備
```
$ cargo new variables
$ cd variables
```

##### `src/main.rs`の編集
```
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

##### 実行してみるとエラーが出る
- 不変変数`x`（immutable）に2回代入してはならない
- コンパイル時にエラーが出る
- 値の変化しない変数はわかるようになる
- 変数はデフォルト不変変数
```
$ cargo run
   Compiling variables v0.1.0
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables`.

To learn more, run the command again with --verbose.
```

##### 可変変数を利用する
- `src/main.rs`の編集
```
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

##### 変数と定数の違い
- mutを定数とともに使用することは許可されてない
- 定数はデフォルトでは不変であるだけでなく、常に不変
- `let`の代わりに`const`キーワードを使用して定数を宣言
- 値の型に注釈を付ける必要がある
- 定数の命名規則は、大文字 + アンダースコアで構成

```
const MAX_POINTS: u32 = 100_000;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
```

##### シャドーイング
- 既存の変数と同名の変数を定義して、そのスコープで既存の変数にアクセスできなくする
- `let`を繰り返し利用することで可能

```
const MAX_POINTS: u32 = 100_000;
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);


    let x = x + 1;

    let x = x * 2;
    println!("The value of x is: {}", x);
}
```

- letキーワードを再度使用すると効果的に新しい変数を作成する
```
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {}", spaces);

    // 実行不可
    let mut spaces = "   ";
    spaces = spaces.len();
```


