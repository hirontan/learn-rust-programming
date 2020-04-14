## 7. Managing Growing Projects with Packages, Crates, and Modules（パッケージ、クレート、モジュールを使用した成長中プロジェクト管理）
- 大規模なプログラムでは、コードの整理が重要になる
  - 方法：関連する機能をグループ化し、機能ごとにコードを分ける
- 実装の詳細をカプセル化する
  - より高いレベルでコードを再利用
  - パブリックかプライベートな変更する権利
- スコープの概念もある
- コードの構成を管理するための機能
  - パッケージ：クレートを構築、テスト、共有できるCargo機能
  - クレート：ライブラリまたは実行可能ファイルを生成するモジュールツリー
  - モジュールと`use`：パスの編成、スコープ、プライバシーを制御できる
  - パス：構造体、関数、モジュールなどのアイテムに名前を付ける

### パッケージとクレート
- クレート
  - バイナリやライブラリ
  - ルートは、コンパイラが起動するソースファイルで、クレートのルートモジュールで構成
- パッケージ
  - 一連の機能を提供する 1 つ以上のクレート
  - どのように構築するかを記述した Cargo.toml ファイルが含まれてる
  - 規則
    - 0 個または 1 個のライブラリクレートが含まれていなければならない
    - バイナリクレートは好きなだけ含めることができるが、少なくとも 1 つのクレート (ライブラリまたはバイナリのいず>れか) を含める必要がある

- パッケージを作成。`cargo new` コマンドを入力
  - 作成したときに何が起こるか確認していく
```
$ cargo new my-project
$ cd my-project/
$ tree
```

- `cargo`が`Cargo.toml`ファイルを作成し、パッケージを提供
- `Cargo.toml`の内容を見ると、`src/main.rs`の記述ない
  - `src/main.rs`はパッケージと同じ名前のバイナリクレートのクレートルートだから
  - パッケージディレクトリに`src/lib.rs`がある場合、パッケージはパッケージと同じ名前のライブラリクレートを含んでおり、`src/lib.rs`がクレートルートであると認識している
  - ライブラリやバイナリをビルドするために、クレートルートファイルを rustc に渡す

- `my-project`は`src/main.rs`だけを含むパッケージがあり、`my-project`のバイナリクレートだけが含まれている
  - パッケージが`src/main.rs`と`src/lib.rs`を含む場合、ライブラリとバイナリの二つのクレートを持ち、両方ともパッケージと同じ名前になる
    - パッケージは、ファイルを`src/bin`ディレクトリに配置することで、複数のバイナリのクレートを持つことができる
      - 各ファイルは別々のバイナリクレートになる

- クレートは、関連する機能をスコープ内にまとめ、複数のプロジェクト間で機能を共有しやすくする
  - ex) `rand`クレートは、乱数を生成する機能を提供
    - `rand`クレートをプロジェクトのスコープに組み込むことで、その機能を使用できる

- クレートの機能を独自のスコープに入れておくことで、特定の機能がクレートで定義されているのか、`rand`クレートで定義されているのかが明確になり、競合の可能性を防ぐことができる
  - ex) `rand クレート`は`Rng`トレイトを提供
    - 独自のクレートで`Rng`という名前の構造体を定義できる
    - クレートの機能は独自のスコープ内で名前空間が設定されているため、依存関係として`rand`を追加しても、コンパイラは`Rng`が何を参照しているのかを混乱させない
    - このクレートでは、定義した`Rng`構造体を参照している
      - `rand`クレートから`Rng`トレイトにアクセスするには`rand::Rng`を使用

### スコープとプライバシーを制御するためのモジュール定義
- モジュールはクレート内のコードをグループにして、読みやすく再利用しやすいようにしてくれる。また、プライバシーも制御してくれる
  - ex) レストランの機能を提供するライブラリクレートを作成
    - 関数のシグネチャを定義。コードの構成のみ
- クレートを構造化するには、機能を入れ子になったモジュールに整理
  - `cargo new --lib restaurant`を実行し、`restaurant`ライブラリを作る
  - `src/lib.rs`にて作成

```
$ cargo new --lib restaurant
$ cd restaurant/
$ tree
.
├── Cargo.toml
└── src
    └── lib.rs
```
- `src/lib.rs`
```
// src/lib.rs

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
- モジュール
  - `mod`キーワードで始める
  - モジュールの本文は`{}`で囲む
  - モジュールの中にモジュールも入れられる
  - 造体、列挙型、定数、特性、関数を定義できる
  - 全ての定義を読むよりもグループに基づいてコードをナビゲートできるので、定義を見つけることが簡単になる
  - コードをどこに配置すれば良いのかもしれる
- クレートルート
  - `src/main.rs`と`src/lib.rs`
- モジュールツリー構造
  - ファイルシステムのディレクトリと同じように、モジュールを使ってコードを整理
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### モジュールツリー内のアイテムを参照するためのパス
- パスには2つの形式がある
  - 絶対パス：クレート名またはリテラルクレートを使用して、クレートのルートから開始
  - 相対パス：現在のモジュールから開始し、`self`・`super`または現在のモジュール内の識別子を使用
- 絶対パスと相対パスの後には、`::`で区切られた1つ以上の識別子が続く
- `restrant`ライブラリにある関数をを絶対パスと相対パスで呼び出せるように変更

```
// src/lib.rs

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- 絶対パス
  - `crate`の後に`::`でパスを指定する
- 相対パス
  - 名前で始まり、`::`でパスを指定する
- 相対パスを使うか絶対パスを使うかは、プロジェクトに応じて決める
  - その決定は、アイテム定義コードをアイテムを使用するコードとは別に移動させるか、一緒に移動させるかに依存する
  - ex)
    - 絶対パス×、相対パス○
      - `front_of_house`モジュールと`eat_at_restaurant`関数を`customer_experience`モジュールに移動した場合
    - 絶対パス○、相対パス×
      - `eat_at_restaurant`関数を別の`dining`モジュールに移動した場合
- `Rust`におけるプライバシーの仕組み
  - すべての項目（関数、メソッド、構造体、列挙型、モジュール、定数）がデフォルトでプライベートになっている
    - 内部の実装の詳細を隠すことがデフォルト
  - 親モジュール内のアイテムは子モジュール内のプライベートなアイテムを使用することはできない
  - 子モジュール内のアイテムは正規祖先モジュール内のアイテムを使用することができる
  - 理由
    - 子モジュールは実装の詳細をラップして隠している
    - 子モジュールはそれらが定義されているコンテキストを見ることができる
  - アイテムを`pub`キーワードを使用することで子モジュールのコードの内部部分を外部の祖先モジュールに公開することができる

##### `pub`キーワードでパスを公開
- `eat_at_restaurant`関数が、子モジュールの`add_to_waitlist`関数にアクセスできるように、`hosting`モジュールに`pub`キーワードをつける
  - 一度、ビルドしてみるが、エラーになる
```
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

- モジュールの`pub`キーワードはその祖先のモジュールのコードだけを参照できる
  - `add_to_waitlist`関数がプライベートであることを示す
  - プライバシーのルールはモジュールだけでなく構造体、列挙型、関数、メソッドにも適用される
- `add_to_waitlist`にも`pub`キーワードを付与

```
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

### `super`で相対パスの開始
- パスの先頭に`super`を使用することで、親モジュールから始まる相対パスを構築することもできる
- `fix_incorrect_order`関数は`super`で始まる`serve_order`へのパスを指定して`serve_order`関数を呼び出す
  - `super`を使って、親モジュール（下記の場合はルートである`crate`）に行くことができる
  - お互いに同じ関係にあり、クレートのモジュールツリーを再編成する場合には、一緒に移動する可能性が高い
    - このコードが別のモジュールに移動した場合に、コードを更新する場所が少なくなるように`super`を使用
```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

##### 構造体と列挙型を公開
- `pub`を使用して構造体や`enums`を`public`に指定できる
- 構造体の定義の前に`pub`を使用
  - 構造体はパブリックになるが、構造体のフィールドはプライベートになる
- ケースバイケースでそれぞれのフィールドを公開するかどうかを決めることができる
- 例を下記に示す

```
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```


- `enum`を`public`
  - その変種はすべて`publicになる

```
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### `use`キーワードでパスをスコープにいれる
- `use`キーワード
  - 絶対パスか相対パスどちらを選択しても、パスを毎回書かなけれならなかった
  - パス内のアイテムをローカルアイテムにあるかのようにできる
  - シンボリックリンクを作成するのと似ている
  - 使用時にスコープに持ち込まれるパスは、他のパスと同様にプライバシーもチェックする

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- use と相対パスを使ってアイテムをスコープにもできる
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

##### Creating Idiomatic `use` Paths
- 関数までスコープに入れてみる
  - 相対パスや絶対パスで実行した例と達成するものは同じになるが、どこで関数が定義されているか不明瞭になる
  - フルパスの繰り返しを最小限に抑えながら、関数がローカルに定義されていないことを明確にする必要がある
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```
- 構造体や列挙型などを使いながら持ち込む場合、フルパスを指定するのがイディオム
  - 標準ライブラリの`HashMap`構造体をバイナリクレートのスコープに持ち込むイディオムを例にしてみる

```
// src/main.rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

- 同じ名前を持ちながらも親モジュールが異なる 2 つの結果型をスコープに持ち込む方法
  - 親モジュールを使用することで2つの`Result`が区別される
  - 代わりに`use std::fmt::Result`と`use std::io::Result`を指定した場合、同じスコープ内に2つの`Result`が存在し、どちらを意味しているのか分からなくなる
```
// src/lib.rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

##### `as`キーワードで新しい名前を提供
- 同じ名前の2つの型を同じスコープに持ち込む問題の解決策がある
  - `as`で新しいローカル名もしくはエイリアスを指定する
- 2つの`Result`型のうちの1つを`as`を使ってリネームする例

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

##### `pub use`で名前を再エクスポート
- `use`キーワードで名前をスコープにすると、新しいスコープで使用可能な名前は`private`になる
- `pub use`
  - 他がそのアイテムをスコープにするため、再エクスポートと呼ばれる
    - プログラマがドメインについてどのように考えるか異なる時に便利になる
  - スコープ外から`hosting::add_to_waitlist`で関数を呼び出せるようになる。新しいパスを呼び出せるようになる
```
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

##### 外部パッケージの利用
- `Rust`コミュニティのメンバーが`crates.io`でパッケージを公開してくれている
  - https://crates.io/
- パッケージに取り込むには、`Cargo.toml`ファイルにパッケージをリストアップし、`use`を使ってアイテムをスコープに入れる

- 標準ライブラリ (std) もパッケージの外部にあるクレートであることに注意
  - 標準ライブラリは Rust 言語に同梱されているので、`Cargo.toml`を変更する必要はないが、アイテムをパッケージのスコープに入れるために`use`を使う必要がある

```
use std::collections::HashMap;
```

##### ネストされたパスを使っていて、大規模リストをクリーンアップする
- 同じモジュールで定義された複数のアイテムを使っている場合、それぞれのアイテムをそれぞれの行にリストアップすると、ファイルの縦方向のスペースを多く取る

```
use std::io;
use std::cmp::Ordering;
```

- 同じアイテムを1行でスコープに入れるために入れ子になったパスを使うことができる
- パスの共通部分を指定し`{}`で囲む

```
use std::{cmp::Ordering, io};
```

- 大規模なプログラムでは、パスを使って同じパッケージやモジュールから多くのアイテムをスコープにして持ってくることで、必要な個別の`use`文の数を大幅に減らすことができる
-  サブパスを共有する2つの`use`ステートメントを組み合わせるときに便利

```
use std::io;
use std::io::Write;
```

- 2つのパスの共通部分は`std::io`、一つの`use`ステートメントに合わせる場合、`self`を利用すると良い

```
use std::io::{self, Write};
```

##### `Glob`演算子
- すべてのパブリックアイテムをスコープに入れたい場合は、パスの後にglob演算子`*`を利用
  - 注意：どの名前がスコープ内にあるのか、プログラムで使用されている名前がどこで定義されているのかがわかりにくくなる
- `glob`演算子は、テスト時によく使用され、テスト対象のすべてを`tests`モジュールに取り込むことができる
```
use std::collections::*;
```
