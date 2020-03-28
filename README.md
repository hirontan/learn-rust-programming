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

### データ型
- コンパイル時にすべての変数の型を知っている
- コンパイラは使用するタイプを推測している

##### スカラー型
- 整数、浮動小数点数、ブール値、文字

- 整数型
  - 符号あり(Signed)と符号なし(Unsigned)がある
  - ex)
    - i8: -128〜127
    - u8: 0〜255
  - 視覚的な区切り文字として「_(アンダースコア)」を使用できる

| Length | Signed | Unsigned |
|:------:|:------:|:--------:|
| 8-bit | i8 | u8 |
| 16-bit | i16 | u16 |
| 32-bit | i32 | u32 |
| 64-bit | i64 | u64 |
| 128-bit | i128 | u128 |
| arch | isize | usize |

  - 整数リテラル

| Number literals | Example |
|:---------------:|:-------:|
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

- 浮動小数点型
  - f32とf64がある
  - デフォルトはf64

```
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

- 数値演算

```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

- ブール型
  - ブール値のサイズは1バイト

```
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

- キャラクター型
  - char型のサイズは4バイトであり、Unicodeスカラー値を表す
  - Unicodeスカラー値の範囲は、U+0000〜U+D7FFおよびU+E000〜U+10FFFF

```
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

##### 複合型
- 複数の値を1つの型にグループ化できる
- タプルと配列という2つのプリミティブな複合型がある

- タプル型
  - さまざまな型の多数の値を1つの複合型にグループ化できる
  - タプルから個々の値を取得するには、パターンマッチングを使用してタプル値を非構造化する
  - ピリオド（.）に続いてアクセスする値のインデックスを使用して、タプル要素に直接アクセスできる
    - タプルの最初のインデックスは0

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of tup.1 is: {}", tup.1);
}
```

- 配列型
  - 配列のすべての要素は同じ型の必要がある
  - タプルのように固定長（他の言語と違うところ）
  - データをヒープではなくスタックに割り当てる、または常に要素の数を固定したい場合に役立つ
    - 配列はベクター型ほど柔軟でない
    - ベクターは標準ライブラリによって提供され、サイズの拡大縮小ができる
      - 配列かベクターか利用を迷う場合は、ベクターを利用することになりそう
  - 要素のタイプ、配列内の要素数を[]内に指定できる
  - 初期値を指定し、その後にセミコロンを指定すると、同じ値が入力される
  - []内にインデックスを入力することでアクセスできる
  - もし配列数を超えて、数値指定した場合、エラーになる
```
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[1] is: {}", a[1]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("The value of months[1] is: {}", months[1]);

    let a = [3; 5];
    println!("The value of a[1] is: {}", a[1]);
}
```

### 関数
- スネークケースで命名

```
fn main() {
    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

##### パラメータ（引数）
- `x`パラメータ
  - 必須：型を指定する`i32`
```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

##### 関数本体にはステートメントと式がある
- ステートメントは値を返さない
  - letステートメントを別の変数に割り当てることはできません
  - CやRubyなどでは、`x = y = 6` で x と y 両方に 6 を割り当てられるが、Rustは割り当てられない

```
fn main() {
    let x = (let y = 6); # → letステートメントで別の変数を割り当てている。Rustではエラーになる。
}
```

- 式はステートメントの一部にすることができる
- ブロック{}はletステートメントで扱える。

```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
}
```

##### 戻り値を持つ関数
- `->` の後に型を宣言
- 最後の式を暗黙的に返す
  - returnキーワードを使用して値を指定することもできる
```
fn main() {
    plus_one(5);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

### コメント
- `//` でコメントになる
- 各行でコメントをしたい場合は、複数行で扱う

### 制御フロー
- 条件が真かどうかに応じてコードを実行するかどうかを決定すること

##### `if`
- 条件に応じてコードを分岐できる
- `bool`でないとエラーが出る

```
fn main() {
    let number = 3;
    use_if(number);
}


fn use_if(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

##### `else if`で複数の条件
- 最初の真の条件に対してのみブロックを実行し、残りのチェックも行わない
```
fn main() {
    let number = 3;
    use_elseif(number);
}

fn use_elseif(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

##### letステートメントでifを使用
- 変数は、if式の結果に基づいて値にバインドされる
- ifの各アームからの結果である可能性がある値は、同じ型であること
  - コードのブロックは最後の式で評価
```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

##### ループによる繰り返し
- loop、while、forの3種類のループがある


- `loop`
  - ループを停止するために使用する`break`の後に、戻りたい値を追加できる

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

- `while` を使用した条件付きループ
  - 条件が真でなくなると、`break`を呼び出し、ループを停止
  - if、else、breakの組み合わせ実装もできる

##### whileで、コレクションの要素をループ
- `for`と比べて、すべての要素の条件チェックを実行するため、処理が遅くなる
```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

##### forでコレクションをループする
- コードを特定の回数実行する必要がある場合、forを利用する
```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

- revを使用して、反転させることも可能
```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Understanding Ownership（所有権）
- ガベージコレクタを必要とせずにメモリの安全性を保証できる
- 所有権といくつかの関連機能について説明
  - メモリの貸し借り
  - スライス

### What Is Ownership?
- すべてのプログラムは、実行中にコンピュータのメモリを使用する方法を管理する必要がある
  - 一部の言語では、プログラムの実行時に使用されなくなったメモリを探すガベージコレクションがある
  - それ以外は、明示的にメモリを割り当てて解放する必要がある
- Rustは、コンパイラーがコンパイル時にチェックし一連のルールを持って所有権の仕組みを通じて管理
  - 所有権機能は、プログラム実行中に速度を低下させることはない。
- 所有権のルールに慣れれば、安全で効率的なコードを自然に開発できるようになる

##### スタックとヒープ
- 多くのプログラミング言語は、スタックとヒープについて頻繁に考える必要はない
- Rustのようなシステムプログラミング言語は、値がスタックかヒープかを考える
- スタックは値を取得した順序で格納し、逆の順序で値を削除する。後入れ先出し
  - データの追加はスタックへのプッシュと呼ぶ
  - データの削除はスタックからのポップと呼ぶ
- スタックに格納されるすべてのデータは、既知の固定サイズでなければならない
  - コンパイル時に不明なサイズまたは変更される可能性のあるサイズのデータは、代わりにヒープに格納する必要がある
  - ヒープはデータ整理がされていない
  - ヒープにデータを配置すると、一定量のスペースが要求される
  - ヒープ内の十分な空の場所を見つけ、使用中としてマークし、その場所のアドレスであるポインタを返す
    - このプロセスはヒープ上での割り当てと呼ばれる
  - 実際のデータが必要な場合は、ポインタに従う必要がある
- 格納
  - スタックへのプッシュは、OSが新しいデータの格納場所を探す必要がないため、ヒープに割り当てるよりも高速
  - ヒープへの割り当ては、最初にデータを保持するのに十分なスペースを見つけてから、次の割り当てに備えて実行する必要があるため、多くのプロセスが必要
- アクセス
  - ヒープ内のデータへのアクセスは、スタック上のデータへのアクセスよりも遅い
    - ポインターに従う必要があるため
- 所有者が対処する問題
  - コードのどの部分がヒープ上のどのデータを使用しているかを追跡し、ヒープ上の重複データの量を最小限に抑え、スペースが不足しないようにヒープ上の未使用のデータをクリーンアップすること
  - 所有権を理解したら、スタックとヒープについて頻繁に考える必要はない
  - ただし、ヒープデータを管理することが所有権が存在する理由であることを理解すること

##### 所有権のルール
- 各値には、その所有者と呼ばれる変数がある
- 一度に所有者は1人
- 所有者が範囲外になると、値は削除される

##### 変数のスコープ
- スコープと変数が有効なときの関係は、他のプログラミング言語と同様

##### 文字列型
- 文字列リテラルは便利ですが、テキストを使用したいすべての状況に適しているわけではない
  - 理由その１：不変であること
  - 理由その２：コードを作成するときにすべての文字列値がわかるわけではないこと

- 2番目の文字列型String
  - ヒープに割り当てられるため、コンパイル時に不明なテキストを格納できる
  - from関数を扱う

```
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
``` 

##### メモリと割り当て
- 文字列リテラルが高速で効率的
  - 文字列リテラルの場合、コンパイル時に内容がわかるため、テキストは最終的な実行可能ファイルに直接ハードコードされる
  - ただし、これらのプロパティは、文字列リテラルの不変性に由来する
  - コンパイル時にサイズが不明であり、プログラムの実行中にサイズが変化する可能性があるテキストの各部分について、メモリの塊をバイナリに入れることはできない
- String型の場合、可変で拡張可能なテキストを保持するために、コンパイル時に不明な量のメモリをヒープに割り当てる必要がある
  - 実行時にオペレーティングシステムからメモリを要求する必要がある
    - `String:: from`を呼び出すと、その実装は必要なメモリを要求します。 これはプログラミング言語ではほぼ普遍的。
  - 文字列を使い終わったら、このメモリをオペレーティングシステムに戻す必要がある
    - ガベージコレクター（GC）を備えた言語では、GCは追跡し、使用されなくなったメモリをクリーンアップするため、考える必要はない
    - GCがなければ、メモリが使用されなくなったときを識別し、コードを呼び出して明示的に行う必要がある
      - 歴史的に難しいプログラミング問題
      - 忘れてしまうとメモリを無駄にする
      - 早すぎると、変数が無効になる
      - 1つの割り当てと1つの空きをペアにする必要がある
- Rustはメモリを所有する変数がスコープ外になると、メモリは自動的に返される
```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        println!("{}", s);
    }                              // this scope is now over, and s is no
                                   // longer valid
```
- 変数がスコープ外になると、特別な関数ドロップを呼び出す
  - 注：C ++では、アイテムの有効期間の終わりにリソースを割り当て解除するこのパターンは、リソース獲得は初期化（RAII）と呼ばれることがあります。 Rustのドロップ機能は、RAIIパターンを使用したことがあればおなじみです。

##### 変数とデータが相互作用する方法：移動
```
let x = 5;
let y = x;
```
- 複数の変数が同じデータと異なる方法で相互作用できる
- 整数は既知の固定サイズの値であり、これら2つの5の値はスタックにプッシュされる


```
let s1 = String::from("hello");
let s2 = s1;
```
- 前のコードと非常によく似ているので、動作方法は同じであると想定できるが、同じ動きをしない
- 文字列は、ポインタ・長さ・容量で構成されている。この三つは、スタックに格納される
- s1をs2に割り当てると、文字列データがコピーされる。スタック上にあるポインター・長さ・容量がコピーされる。ポインターが参照するヒープ上のデータはコピーしない

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
- ダブルフリーエラー
  - 変数がスコープ外になると、自動的にdrop関数を呼び出し、その変数のヒープメモリをクリーンアップするため、同じ場所を指す両方のデータポインターを示している場合、同じメモリを開放しようとしてしまうこと
  - メモリを2回解放すると、メモリの破損につながり、セキュリティの脆弱性につながる可能性がある
- 他の言語を使用しているときに浅いコピーと深いコピーという用語を聞いた場合、データをコピーせずにポインタ、長さ、容量をコピーするという概念は、浅いコピーを作成するように聞こえる
  - Rustは最初の変数も無効にするため、浅いコピーと呼ばれるのではなく、移動と呼ぶ
  - データの「深い」コピーを自動的に作成することはありません

##### 変数とデータが相互作用する方法：クローン
```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```
- スタックデータだけでなく、文字列のヒープデータを深くコピーしたい場合は、クローンと呼ばれる一般的なメソッドを使用できる
  - cloneの呼び出しを見ると、いくつかの任意のコードが実行されており、そのコードには負荷がかかる可能性がある
- ヒープデータもコピーした場合になる

##### スタックのみのデータ：コピー
```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

- クローンを呼び出す呼び出しはありませんが、xはまだ有効であり、yに移動されていない
  - コンパイル時に既知のサイズを持つ整数などの型は完全にスタックに格納されるため、実際の値のコピーをすばやく作成できるため
  - ここではディープコピーとシャローコピーに違いはないため
- コピーであるタイプの一部
  - u32などのすべての整数型
  - ブール型の値で、値はtrueとfalse
  - f64などのすべての浮動小数点型
  - 文字タイプchar
  - タプル（コピーでもあるタイプのみを含む場合） たとえば、（i32、i32）はコピーですが、（i32、String）はそうではありません

##### 所有権と関数
```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
- takes_ownershipの呼び出し後にsを使用しようとすると、Rustはコンパイル時エラーになる

##### 戻り値とスコープ
- 書き方の例のみあげておきます
```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

- タプルを利用して複数の値を返すこともできる
```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

### 参照と借用
- 変数宣言のすべてのタプルコードと関数の戻り値がなくなるように、`calculate_length`を変更
- Stringではなく＆Stringを使用
  - ＆String 構文を使用すると、string の値を参照するがそれを所有しない参照を作成できる
  - 所有していないので、参照が範囲外になったときに、値はドロップされない
  - 関数パラメーターのスコープと同じだが、所有権がないためスコープが外れたときに参照したものを削除しない
  - 関数が実際の値の代わりにパラメーターとして参照を持つ場合、所有権がなかったため
  - 参照を関数パラメータの借用と呼びます。
```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_string_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_string_length(s: &String) -> usize {
    s.len()
}
```

##### 可変参照
- ＆mutを使用して可変参照を作成し、some_string：＆mut Stringを使用して可変参照を受け入れる必要がある
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- 特定のスコープ内の特定のデータへの変更可能な参照は1つだけ
  - 下記のようにコードを書くことができない
```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

- この制限の利点は、コンパイル時のデータ競合を防止できること
- データの競合は3つの動作が発生したとき
  - 2つ以上のポインターが同じデータに同時にアクセス
  - 少なくとも1つのポインターがデータへの書き込みに使用されている
  - データへのアクセスを同期するために使用されるメカニズムがない
- 新しいスコープを作成し、同時参照ではなく複数の可変参照を許可できる

```
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```

- 変更可能な参照と不変の参照を組み合わせるための同様のルールがある
  - 下記はエラーになる
    - 不変の参照がある間は、変更可能な参照を持つことはできない
    - 複数の不変参照は問題ない
```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

- 参照の範囲は、その参照が最後に使用されたときまで続くことに注意
  - 下記は問題なく動作する
  - 不変参照r1およびr2のスコープはprintln!で終了する
  - コンパイラが潜在的なバグを早期に（実行時ではなくコンパイル時に）指摘してくれる
```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

##### ダングリングリファレンス
- ポインターを使用する言語では、ダングリングポインター（他のユーザーに与えられた可能性のあるメモリー内の場所を参照するポインター）を誤って作成し、そのメモリーへのポインターを保持しながらメモリを解放することは簡単
- Rustでは、コンパイラーは参照から参照になることは決してないことを保証する
  - データ参照がある場合、コンパイラーはデータ参照が行われる前に、データがスコープから外れないことを保証する

- ダングリングリファレンスの例
  - この関数の戻り値の型には借用した値が含まれていますが、値がありませんとエラーが出る
  - dangleのコードが終了すると、sは割り当て解除されているが、、参照を返そうとして無効な文字列を指してしまっている
```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

- 直接文字列を返すことで解決される
```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

##### 参照のルール
- 1つの変更可能な参照、または任意の数の不変の参照を持つことができる
- 参照は常に有効である必要がある

### スライス型
- 所有権を持たないもう1つのデータ型はスライス
  - コレクション全体ではなく、コレクション内の要素の連続したシーケンスを参照できる

```
fn first_word(s: &String) -> usize {
    // 文字列をバイトの配列に変換
    let bytes = s.as_bytes();

    // バイトの配列に対してイテレーターを作成
    // enumerateから返されるタプルの最初の要素はインデックスで、2番目の要素は要素への参照
    for (i, &item) in bytes.iter().enumerate() {
        // スペースが見つかったら、位置を返す
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

##### 文字列スライス
```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```
- 文字列全体への参照ではなく、文字列の一部への参照
- [starting_index..ending_index]を指定することで、スライスを作成できる
  - 内部的に、スライスデータ構造は、スライスの開始位置と長さを格納
  - let world =＆s [6..11];の場合、worldは、長さが5のsの7番目のバイト（1から数えて）へのポインターを含むスライス
  - 「..」範囲構文を使用して、最初のインデックス（ゼロ）から開始する場合は、「..」の前の値を削除
    - スライスに文字列の最後のバイトが含まれている場合は、末尾の数字を削除
    - `&s[..];` のように全体を取得することもできる

##### 文字リテラルはスライス
```
let s = "Hello, world!";
```
- ここでのsのタイプは＆str
  - バイナリの特定のポイントを指すスライス
  - 文字列リテラルが不変である理由
  -  ＆strは不変の参照

##### パラメータとしての文字列スライス
- ＆String値と＆str値の両方で同じ関数を使用できる
```
fn first_word(s: &str) -> &str {
```

- 文字列スライスがある場合は、直接渡せる
  - 文字列がある場合は、文字列全体のスライスを渡すことができる
  -  文字列への参照の代わりに文字列スライスを取得する関数を定義すると、APIは機能を失わない

```
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

##### その他のスライス
- 配列の一部を参照

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

## Using Structs to Structure Related Data

- 構造体は、構成する複数の関連する値に名前を付けてパッケージ化できるカスタムデータ型
- オブジェクト指向言語に精通している場合、構造体はオブジェクトのデータ属性のようなもの
- コンパイル時の型チェックを最大限に活用するためにプログラムのドメインで新しい型を作成するための構成要素

### 構造の定義とインスタンス化
- 構造体はタプルに似ている
- 各データに名前を付け、値の意味が明確に
- 構造体はタプルよりも柔軟性がある
- インスタンスの値を指定して、値にアクセスする
- データの順序に依存しない
- 構造体を定義
  - キーワード`struct`と入力し、構造体全体に名前を付ける
  - 構造体の名前は、グループ化されるデータの重要性を説明するものである必要がある
  - `{}`の内側で、データの名前とタイプを定義。これらをフィールドと呼ぶ

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

- 構造体の使用
  - 各フィールドに具体的な値を指定して、その構造体のインスタンスを作成
  - key：valueペアを含む`{}`を追加
  - keyはフィールドの名前。値はこれらのフィールドに格納するデータ
  - 構造体で宣言した同順序でフィールドを指定する必要はない
    - 構造体の定義は型の一般的なテンプレートのようなもの

```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

- 構造体から値を取得するには、ドット表記を使用
- インスタンスが変更可能な場合、ドット表記を使用して値を変更し、特定のフィールドに割り当てることができる
```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

- 特定のフィールドのみを変更可能

```
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

##### 変数とフィールドの名前が同じ場合のフィールド初期化の省略形の使用
- フィールド名と変数を繰り返さない省略形がある
```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

##### Struct Update構文を使用して他のインスタンスからインスタンスを作成する
- 古いインスタンスの値のほとんどを使用し、一部を変更する構造体の新しいインスタンスを作成すると便利
- 構造体更新構文を使用

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

- `..`構文は、明示的に設定されていない残りのフィールドが、指定されたインスタンスのフィールドと同じ値であることを指定する

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

##### 名前付きフィールドのないタプル構造を使用して異なるタイプを作成
- タプル構造体と呼ばれる、タプルに似た構造体を定義することもできる
- タプルには、構造名が提供する意味が追加されていますが、フィールドに関連付けられた名前はない
  - フィールドの型を持っているだけ
- タプル構造体は、タプル全体に名前を付け、他のタプルとは異なる型にしたい場合に役立つ
- タプル構造体の定義
  - 構造体キーワードと構造体名で始まり、その後にタプル内の型が続く

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

- 構造体内のフィールドは同じ型ですが、定義する各構造は独自の型
  - 両方の型が3つの`i32`値で構成されている場合でも、引数として違う関数を受け取れない
- タプル構造体インスタンスはタプルのように動作
- インスタンスを個別の部分に分解・使用できる。 その後に、個々の値にアクセスするためのインデックスが続く

##### フィールドのないユニットのような構造体
- フィールドを持たない構造体を定義することもできる
- ユニットタイプ（）と同様に動作するため、ユニットのような構造体と呼ばれる

###### 構造データの所有権
- User構造体の定義では、＆str文字列スライスタイプではなく、所有されている文字列タイプを使用した
- 構造体のインスタンスがすべてのデータを所有し、構造体全体が有効である限りそのデータが有効である
- 構造体が別のものが所有するデータへの参照を格納することは可能だが、ライフタイムを使用する必要がある
  - 下記は機能しない

```
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

### 構造体を使用したサンプルプログラム
- 長方形の面積を計算するプログラム
- 単一の変数から始めて、構造体を使用するまでプログラムをリファクタリングする

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

- 幅と高さをグループ化すると、読みやすく、管理しやすくなる

##### タプルを使用したリファクタリング

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

- タプルを使用すると、構造を追加でき、引数を1つだけ渡せる
  - タプルは要素に名前を付けていないため、タプルの部分にインデックスを付ける必要があるため、計算がさらに複雑になる
  - インデックスの番号を覚えておく必要がある → 値を忘れやすくなる

##### 構造体によるリファクタリング：意味を追加する
- 構造体を使用して、データにラベルを付けることで意味を追加する
  - 造体を所有するのではなく借用
  - 関数シグネチャで＆を使用する
  - 値にわかりやすい名前を付ける
```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

##### 派生した特性を持つ有用な機能の追加
```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

- 上記は機能しない。エラーは下記に

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

- `println!`は`std::fmt::Display`を利用している
  - プリミティブ型は、表示する方法が1つしかないため問題ない
  - 構造体は、多くの表示の可能性があるため、出力のフォーマットはあまり明確ではない。曖昧さを許容しないようにしている
- マクロ呼び出し（構造体を呼び出す）：`{:?}`を利用する
  - `Debug`の出力形式で利用できる
  - `#[derive(Debug)]`と明示する必要がある

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

- 構造体が大きい場合、出力を少し読みやすくする方法
  - `{:?}`を`{:#?}`に変更する

### メソッド構文
- メソッド
  - 関数に似ている
  - `fn`キーワードとその名前で宣言される
  - パラメーターと戻り値を持てる
  - 実行コードが存在する
  - 関数とは異なり、構造体のコンテキスト内で定義され、最初のパラメーターは常に`self`
  - メソッドが呼び出されている構造体のインスタンス

##### メソッドの定義
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

- 変更前
  - `area`関数を呼び出して`rect1`を引数として渡していた
- 変更後
  - メソッド構文を使用して、`Rectangle`インスタンスの`area`メソッドを呼び出す
  - メソッド構文はインスタンスの後に続く
- メソッドが`impl Rectangle`コンテキスト内にあるため、`self`が`Rectangle`であることを認識している
- メソッドは`self`の所有権を取得、借用できる
  - `self` 、`&self` 、`&mut self`
  - `self`を使用して、インスタンスの所有権を取得
  - `&self`は、所有権を取得するのではなく、構造体のデータを読み取る
  - `&mut self`は、呼び出したインスタンスを変更する場合に利用する

##### Where’s the `->` Operator?
- CおよびC++のアロー演算子
  - `*`：ポインタの指すデータへのアクセス
  - `.`：構造体のメンバへのアクセス
  - `->`：`*`と`.`を両方担える
- Rustの自動参照と逆参照
  - CおよびC++のアロー演算子に相当するものはない
  - メソッドを呼び出すと、自動的に＆、＆mut、または*を追加する
  - メソッドが読み取り（＆self）、変更（＆mut self）、または消費（self）のいずれであるかを明確に把握できる

##### パラメータが多いメソッド
- メソッドは、selfパラメータの後にシグネチャに追加する複数のパラメータを取ることができる
```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

ft main(){
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

##### 関連関数
- `self`を引数に取らない関数を定義
- 構造体の新しいインスタンスを返すコンストラクターによく使用される
- 関連関数を呼び出すには、`::`構文と構造体名を使用する

```
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

##### 複数`impl`ブロック
- 下記のように分かれて活用することもできる
```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```


