## 9. Error Handling（エラーハンドリング）
- コードがコンパイルされる前にエラーの可能性を認識してくれる
- エラーを、`recoverable`か`unrecoverable`なエラーが主要なカテゴリ
  - `recoverable`：ファイルが見つからないエラーなど、操作を再試行する問題がなくなる状態
  - `unrecoverable`：配列の末尾を超えた場所にアクセスしようとした場合など、バグのある状態
  - ほとんどの言語ではこの二つを区別していない
- Rustには例外がない。その代わりとして、二種類用意されている
  - `recoverable`： `Result<T, E>`型
  - `unrecoverable`：実行を停止する`panic!`

### `panic!`で`unrecoverable`なエラー
- `panic!`マクロ
  - 実行されると、プログラムは失敗メッセージを表示し、スタックを巻き戻してクリーンアップした後、終了する
  - バグが検出された場合に最も一般的に発生するもの

##### パニックに対応したスタックの巻き戻しや中止
- パニック発生時のデフォルト対応
  - プログラムは巻き戻しを開始し、スタックに戻って各関数からデータをクリーンアップする
- 別の方法として、クリーンアップを行わずにプログラムを終了させることができる
  - プログラムが使用していたメモリは、オペレーティングシステムによってクリーンアップされる必要がある
  - 結果として得られるバイナリをできるだけ小さくする必要がある場合は、`Cargo.toml`ファイルの適切な`[profile]`セクションに`panic = 'abort'`を追加
    - パニックが発生したときに巻き戻しから中止に切り替えられる
- ex) リリースモードでパニック時に中止したい場合は下記のように記述
```
[profile.release]
panic = 'abort'
```

- `panic!`を呼び出してみる
```
fn main() {
    panic!("crash and burn");
}
```

##### `panic!`を使ってバックトレース
- ベクターの終端を超えてアクセスすると`panic!`が呼ばれる
  - 向こうなインデックスを渡した場合、返す要素がない → パニックになる
```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

- C言語
  - データ構造体の終端を越えて読み込もうとすると、未定義の動作になる
  - メモリがその構造体に属していなくても、データ構造体のその要素に対応するメモリの位置にあるものは何でも取得できるかもしれない
    - バッファオーバーリードと呼ばれ、攻撃者がデータ構造体の後に格納されている許可されていないデータを読み取るような方法でインデックスを操作することができた場合、セキュリティ上の脆弱性につながる可能性がある
      - この脆弱性からプログラムを守るため、Rustは実行を停止して続行を拒否する

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```
- Rustのsliceの実装
  - このエラーは `libcore/slice/mod.rs`という書き込んでいないファイルを指している
  - ベクトル`v`で`[]`を使用したときに実行されるコードは`libcore/slice/mod.rs`にあり、`panic!`が実際に起こっている箇所

- 環境変数`RUST_BACKTRACE`を設定して、エラーの原因となったことのバックトレースを取得できる
  - バックトレースは、この時点に至るまでに呼び出されたすべての関数のリスト

-  環境変数`RUST_BACKTRACE`が設定されているときに`panic!`の呼び出しによって生成されたバックトレース

```
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:84
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:61
   4: core::fmt::ArgumentV1::show_usize
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1426
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:65
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:50
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:193
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:471
  11: rust_begin_unwind
             at src/libstd/panicking.rs:375
  12: core::panicking::panic_fmt
             at src/libcore/panicking.rs:84
  13: core::panicking::panic_bounds_check
             at src/libcore/panicking.rs:62
  14: <usize as core::slice::SliceIndex<[T]>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806
  15: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2657
  16: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/liballoc/vec.rs:1871
  17: panic::main
             at src/main.rs:4
  18: std::rt::lang_start::{{closure}}
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  19: std::rt::lang_start_internal::{{closure}}
             at src/libstd/rt.rs:52
  20: std::panicking::try::do_call
             at src/libstd/panicking.rs:292
  21: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:78
  22: std::panicking::try
             at src/libstd/panicking.rs:270
  23: std::panic::catch_unwind
             at src/libstd/panic.rs:394
  24: std::rt::lang_start_internal
             at src/libstd/rt.rs:51
  25: std::rt::lang_start
             at /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libstd/rt.rs:67
  26: panic::main
```

- バックトレースを取得するために、デバッグシンボルを有効にする必要がある
  - デバッグシンボルはデフォルトで有効になっている
  - `--release`をつけずに`cargo run`もしくは`cargo build`を使う場合、デバッグシンボルは有効になる

### Recoverable エラーと`Result`
- ほとんどのエラーは、プログラムを完全に停止させるほど深刻なものではありません
  - 失敗したとき、それはあなたが簡単に解釈して対応できるような理由であることもある
    - ex) ファイルを開こうとして、その操作がファイルが存在しないために失敗した場合、プロセスを終了させるのではなく、ファイルを作成したいと思うかもしれない

- `Ok`と`Err`の2つを定義されている列挙型
  - `T`と`E`は汎用型パラメータ
  - `T`は`Ok`内で成功した場合に返される値の型を表す
  - `E`は`Err`内で失敗した場合に返されるエラーの型を表す
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- 関数が失敗する可能性があるので、`Result`値を返す関数を呼び出す
  - ex) ファイルを開く

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

- `File::open`が`Result`を返すことをどうやって知るのか？
  - https://doc.rust-lang.org/std/fs/struct.File.html
  - コンパイラは`File::open`の戻り値の型が`u32`ではないことを教えてくれる
- `let f`を`let f: u32`に変更して`cargo run`
  - `File::open`関数の戻り値の型が`Result<T, E>`
```
$ cargo run
   Compiling error_handling v0.1.0 (file://error_handling)
error[E0308]: mismatched types
 --> src/main.rs:5:18
  |
5 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `std::result::Result<std::fs::File, std::io::Error>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `error_handling`.

To learn more, run the command again with --verbose.
```

- `File::open`が返す値に応じて異なるアクションを実行する
  - `Result enum`などはスコープに入っているので、`Ok`と`Err`は`Result::`を指定する必要がない
  - 結果が`Ok`の場合、内部のファイル値を返す
  - もう一方のアームは、`File::open`から`Err`値を取得した場合を処理
    - ex) `panic!`マクロを呼び出す
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

- 実行すると、`panic!`が呼び出される
```
$ cargo run
   Compiling error_handling v0.1.0 (file://error_handling)
warning: unused variable: `f`
 --> src/main.rs:6:9
  |
6 |     let f = match f {
  |         ^ help: consider prefixing with an underscore: `_f`
  |
  = note: `#[warn(unused_variables)]` on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/error_handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

##### 異なるエラーでのマッチング
- ファイルが存在せず`File::open`が失敗した場合、ファイルを作成して新しいファイルのハンドルを返す
  - ファイルを開く権限がなかったなどの理由で`File::open`が失敗した場合、`panic!`内部のマッチ式を追加する
- 異なる種類のエラーを異なる方法で処理する
  - `File::open`が`Err`内で返す値の型は、標準ライブラリが提供する構造体である`io::Error`
    - `io::ErrorKind`値を取得するために呼び出すことができるメソッドの種類がある
    - `enum io::ErrorKind`は標準ライブラリで提供
    - `ErrorKind::NotFound`で、開こうとしているファイルがまだ存在しないことを示す
      - `error.kind()`でもマッチする
  - `File::create`でファイルを作成
    - `File::create`も失敗する可能性があるので、内側のマッチ式に二つ目のアームが必要
    - ファイルを作成できない場合は、別のエラーメッセージが表示
    - ファイルが作成できない以外のエラーが発生してもプログラムは`panic!`になる

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

- `match`式は非常に便利だが、原始的
  - `Result<T, E>`型にはクロージャを受け入れるメソッドがあり、`match`式を使用して実装されている
  - コードをより簡潔にできる
  - `unwrap_or_else`
    - https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

##### エラー時にパニックになるためのショートカット：`unwrap`と`expect`
- `Result<T, E>`型には、さまざまなタスクを実行するために定義された多くのヘルパーメソッドがあある
- `unwrap`
  - `Ok`の場合、`Ok`の内部の値を返す
  - `Err`の場合、`panic!`マクロを呼び出す
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
- `expect`
  - `panic!`エラーメッセージを選択することもできる
  - 良いエラーメッセージを提供することで、意図を伝わり、原因を追求しやすくなる

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

##### エラーの伝搬
- エラーを伝搬する
  - 実装が何かを呼び出して失敗する可能性のある関数を書いている場合、その関数内でエラーを処理する代わりに、呼び出し元のコードにエラーを返して、呼び出し元のコードが何をすべきかを判断できるようにする
  - コードのコンテキストで利用できるものよりもエラーがどのように処理されるべきかを指示する情報もしくはロジックがある場合、呼び出し側のコードにより多くのコントロールを与える
  - ex) ファイルが存在しないか読み込めない場合、この関数はこの関数を呼び出したコードにこれらのエラーを返す
    - 関数の戻り値の型：`Result<String, io::Error>`
      - `Result<T, E>`型の値を返していることを意味している
      - 成功した場合、`String`を保持する`Ok`値を受け取る
      - 問題が発生した場合、問題のある詳細な情報を含む`io::Error`のインスタンスを保持する`Err`値を受け取る
        - 戻り値の型として`io::Error`を選択
          - `File::open`関数と`read_to_string`メソッドの両方で失敗する可能性のある操作から返されるエラー値の型

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

##### エラーを伝播するためのショートカット：`?`演算子
- 先ほどの`read_username_from_file`を`?`演算子を用いる
  - `? Result`の値が`Ok`の場合、`Ok`の内側から値を返される
  - `Err`の場合、関数全体から返される（エラー値は呼び出しコードに伝搬される）
```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

- `?`演算子を使用することで、多くのボイラプレートが削除され、実装がよりシンプルになる
```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```
- さらに短くする
  - 「ファイルを文字列に読み込む」
```
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

##### `Result`を返す関数で使用できる`?`演算子
- `?`演算子
  - `Result`のリターン型を持つ関数で使うことができる
  - マッチのうち`Result`の戻り値の型を必要とする部分は`return Err(e)`なので、関数の戻り値の型は互換性のある`Result`にできる
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
- 実行してみる
  - `Result`や`Option`、または`std::ops::Try`を実装した別の型を返す関数でしか使えないとエラーが出る
```
$ cargo run
   Compiling error-handling v0.1.0 (file://error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
 --> src/main.rs:4:13
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling`.

To learn more, run the command again with --verbose.
```
- `Result<T, E>`を返す他の関数を呼び出すときに`?`を使いたい場合
  - 関数の戻り値の型を `Result<T, E>`に変更する
  - `match`または`Result<T, E>`メソッドの一つを使用して、`Result<T, E>`を処理すること
- `Box<dyn Error>`型はtraitオブジェクト
```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```


