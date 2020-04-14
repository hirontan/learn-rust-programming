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
