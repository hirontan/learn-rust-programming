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
