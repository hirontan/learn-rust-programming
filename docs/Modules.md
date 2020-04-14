## extra. Modules（モジュール）
- モジュールを使用してコードを体系化し、再利用する
- モジュールは関数や型定義を含む名前空間
  - 定義がモジュールの外からも扱えるようにするか否か選択できる（Public / Private）
- 概要
  - `mod`キーワードで新規モジュールを宣言。モジュール内のコードは、この宣言の直後の`{}`内か、 別のファイルに存在する
  - 標準では、関数/型/定数/モジュールは非公開(private)。pubキーワードで要素は公開され、 名前空間の外からも扱えるようになる
  - `use`キーワードでモジュールやモジュール内の定義をスコープに入れることができる

### `mod`とファイルシステム
- 何らかの一般的なネットワーク機能を提供するライブラリの骨格を作成する
- ライブラリ生成：`--bin`の代わりに`--lib`オプションを利用する
  - `src/main.rs`の代わりに`src/lib.rs`が生成される
```
$ cargo new communicator --lib
$ cd communicator
```

- `src/lib.rs`
  - `src/main.rs`ファイルがないから、`cargo run`で実行できない
  - `cargo build`でコードをコンパイルして利用する
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

##### モジュール定義
- `communicator`ネットワークライブラリ
  - `connect`関数を含む`network`モジュールを定義
  - 関数を`network`モジュール外のスクリプトから呼び出したい場合、 モジュールを指定し、名前空間記法`::`を利用し`network::connect()`と指定する必要がある
```
mod network {
    fn connect() {
    }
}
```

- 複数のモジュールを並べる
  - 異なるモジュールに存在するので、関数がお互いに衝突しない
  - `network::connect`と`client::connect`の関数が生成される
- モジュール内にモジュールを書くことも可能
```
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```
- `client`モジュールを`network`モジュール内に移動させる
  - `network::connect`と`network::client::connect`の関数が生成される
```
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

##### モジュールを別ファイルに移す
- `src/lib.rs`に三つのモジュールを生成 `client`、`network`、`network::server`
```
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

- 関数の中身となるコードが長くなるので、分割する

- `src/lib.rs`から`client`モジュールの中身を抽出する
  - `client`モジュール
    - ブロックを`;`で置換したことで、 `client`モジュールのスコープのコードは別の場所を探すようにコンパイラに指示している
```
// src/lib.rs
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

- clientモジュールのconnect関数を別ファイルに作成

```
// src/client.rs
fn connect() {
}
```

- ビルドしてみる

```
$ cargo build
   Compiling communicator v0.1.0 (................./communicator)
warning: function is never used: `connect`
 --> src/client.rs:1:4
  |
1 | fn connect() {
  |    ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function is never used: `connect`
  --> src/lib.rs:22:8
   |
22 |     fn connect() {
   |        ^^^^^^^

warning: function is never used: `connect`
  --> src/lib.rs:26:12
   |
26 |         fn connect() {
   |            ^^^^^^^

    Finished dev [unoptimized + debuginfo] target(s) in 0.41s
```

- `network`モジュールも単独のファイルに抽出する

```
// src/lib.rs
mod client;

mod network;
```

```
// src/network.rs
fn connect() {
}

mod server {
    fn connect() {
    }
}
```

- `src/network.rs`ファイルを`server`モジュールを抽出する

```
// src/network.rs
fn connect() {
}

mod server;
```

```
// src/server.rs
fn connect() {
}
```

- ビルドをここで実行してみる
  - エラーが起こる
    - コンパイラが、`server`は`network`のサブモジュールと考えられることを検知できないから
```
$ cargo build
   Compiling communicator v0.1.0 (................./communicator)
error[E0583]: file not found for module `server`
 --> src/network.rs:9:5
  |
9 | mod server;
  |     ^^^^^^
  |
  = help: name the file either network/server.rs or network/server/mod.rs inside the directory "src"

error: aborting due to previous error

For more information about this error, try `rustc --explain E0583`.
error: could not compile `communicator`.

To learn more, run the command again with --verbose.
```

- エラー解消する方法
  1. 親モジュール名である`network`という名前の新規ディレクトリを作成する
  2. `src/network.rs`ファイルを`network`ディレクトリに移し、 `src/network/mod.rs`と名前を変える
  3. サブモジュールファイルの`src/server.rs`を`network`ディレクトリに移す

```
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

##### モジュールファイルシステムの規則
- ファイルに関するモジュール規則
  - `foo`モジュールにサブモジュールがなければ、`foo`の定義は、 `foo.rs`というファイルに書くべき
  - `foo`モジュールにサブモジュールがあったら、`foo`の定義は、 `foo/mod.rs`というファイルに書くべき
  - ルールは再帰的に適用される

```
├── foo
│   ├── bar.rs (`foo::bar`内の定義を含む)
│   └── mod.rs (`mod bar`を含む、`foo`内の定義を含む)
```

### `pub`で公開するか制御する
- これまで関数が使用されていないと警告が出力されていた
  - 作成していた関数は他のプロジェクトで利用することにある
- 外部から`communicator`ライブラリをを利用してみて確認してみる
  - `src/main.rs`を同じディレクトリに作成する
  - `extern crate`で`communicator`ライブラリクレートをスコープに導入
- Cargoは`src/main.rs`をバイナリクレートのルートファイルとして扱い、`src/lib.rs`は既存のライブラリクレートとは区別される
  - `extern crate`はルートモジュールに記述する
```
// src/main.rs
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```
- `client`モジュールが非公開であることから、実行するとエラーになる
```
error[E0603]: module `client` is private
  --> src/main.rs:4:19
   |
4  |     communicator::client::connect();
   |                   ^^^^^^ this module is private
   |
```
- 警告は、関数を公開にすると、コンパイラが、他のコードから扱われることを理解してくれるので、警告が消える

##### 関数を公開にする
- `警告が出ていたところを解消するため、`client`モジュールを公開にする
  - `mod`の直前に`pub`キーワードを追加する
```
// src/lib.rs
pub mod client;

mod network;
```
- 関数`connect`も非公開になっているので、公開に変更する
  - その他の警告も解消する
```
// src/client.rs
pub fn connect() {
}
```
- `src/lib.rs`
```
// src/lib.rs
pub mod client;

pub mod network;
```
- src/network/mod.rs
```
// src/network/mod.rs
pub fn connect() {
}

pub mod server;
```

- `src/network/server.rs`
```
// src/network/server.rs
pub fn connect() {
}
```

##### プライバシー規則
- 要素の公開性ルール
  - 要素が公開なら、どの親モジュールを通してもアクセス可能
  - 要素が非公開なら、親モジュールと子モジュールのみアクセス可能

##### プライバシー例
- 新しいライブラリプロジェクトを作成
```
$ cargo new privacy_example --lib
$ cd privacy_example/
```

- `src/lib.rs`を下記に編集
```
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

##### エラーを確かめる
- `try_me`関数は、プロジェクトのルートモジュールに存在
  - `outermost`モジュールは非公開だが、`try_me`と同じく`outermost`は現在(ルート)のモジュールなので、`try_me`関数は`outermost`モジュールにアクセス許可される
- `middle_function`は公開なので、`outermost::middle_function`の呼び出しも動作する
  - `try_me`は`middle_function`に親モジュールの`outermost`を通してアクセスできる
- `outermost::middle_secret_function`の呼び出しは、コンパイルエラーになる
  - `middle_secret_function`は非公開で、`middle_secret_function`の現在のモジュール`outermost`でも、`middle_secret_function`の現在の子モジュールでもない
- `inside`モジュールは非公開で`outermost`からのみアクセスできる
  - `try_me`関数は、`outermost::inside`内の関数を呼び出すことを許されない

##### エラーを修正する
```
// src/lib.rs
mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

- 警告も修正
```
pub mod outermost {
    pub fn middle_function() {}

    pub fn middle_secret_function() {}

    pub mod inside {
        pub fn inner_function() {}

        pub fn secret_function() {}
    }
}

pub fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```

### 異なるモジュールの名前を参照する
- `nested_modules`関数を呼び出すためのサンプルを前回のプロジェクト`privacy_example`を利用していきます
- フルパスを指定して関数を呼び出す
```
// src/main.rs
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

##### `use`キーワードで名前をスコープに導入する
- フルパス指定した参照するより、簡潔にするキーワードが用意されている
  - 指定したものだけスコープになる
```
// src/main.rs
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```
- `use`キーワードで関数を指定して、関数をスコープに入れることもできる
  - 直接関数を参照できるようになる
```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```
- `enum`もモジュールのように名前空間なので、`enum`の列挙子を`use`キーワードでスコープに導入できる
- 1つの名前空間から複数の要素をスコープに導入する場合、`{}`を使用する
  - `Green`は`use`文に含んでいないので、`TrafficLight::Green`を指定する
```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```

##### Globで全ての名前をスコープに導入する
- 名前空間の要素を全てスコープに導入するには、`*`表記が使用できる
- `glob(*)`演算子
  - 名前の衝突を起こす可能性があるので、あまり使うべきでない
```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

##### superを使用して親モジュールにアクセスする
- ライブラリクレートを作成した時、Cargoは`tests`モジュールを用意してくれている
  - `communicator`プロジェクトを利用する
    - `tests`もただのモジュール
```
// src/lib.rs


pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

- `it_works`関数から`client::connect`関数を呼び出してみる
```
// src/lib.rs

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

- `cargo test`コマンドを呼び出してテストを実行
  - コンパイルに失敗する
    - 原因はパスが常に現在のモジュールに対して相対的になり、ここでは`tests`になっているから
    - 例外はuse文内、パスは標準でクレートのルートに相対的になる
    - `tests`モジュールは`client`モジュールをスコープにいれる必要がある
```
$ cargo test
   Compiling communicator v0.1.0 (............/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:30:9
   |
30 |         client::connect();
   |         ^^^^^^ use of undeclared type or module `client`
```

- 現在のモジュールからモジュール階層を一つあげる方法
  - `::client::connect();`
  - `super::client::connect();`
  - `use`を利用することで、親モジュールに対して相対にすることができる
```
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```
- 再度`cargo test`を実行する

```
$ cargo test
   Compiling communicator v0.1.0 (................./communicator)
    Finished test [unoptimized + debuginfo] target(s) in 1.01s
     Running target/debug/deps/communicator-064e710be12f4b74

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/communicator-f351c37161dfc4ce

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests communicator

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

