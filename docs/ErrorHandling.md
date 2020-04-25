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

