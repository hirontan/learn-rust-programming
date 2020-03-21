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
