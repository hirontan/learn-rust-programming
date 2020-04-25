## 8. Common Collections（共通コレクション）

- 標準ライブラリにコレクションと呼ばれる便利なデータ構造がある
- 下記の3つのコレクションについて、この章では説明
  - ベクタ型：可変数の値を並べて格納できる
  - 文字列：`String`
  - ハッシュマップ：値を特定のキーに関連付ける

### ベクタで一連の値を保持する
- `Vec<T>`：ベクタ
  - メモリに値を隣り合わせに並べる単独のデータ構造に2つ以上の値を格納する
  - 同じ型の値しか格納できない
  - 要素のリストがある場合に有用
    - ex) テキストファイルの各行やショッピングカートのアイテムの価格など

##### 新しいベクタを生成する
- `Vec::new`関数：新しい空のベクタを作る
- 新しい空のベクタを精製して、`i32`型を格納
  - 型注釈をつけている
  - コンパイラは値の型を推測するので、型注釈をつける必要はない。
    - 初期値のある`Vec<T>`を精製する方が一般的
```
let v: Vec<i32> = Vec::new();
```

- `vec!`というマクロもある
- 1、2、3という値を持つ新しい`Vec<i32>`を生成
  - 初期値の`i32`値を与えたので、型を推測し、型注釈が不要になった

```
let v = vec![1, 2, 3];
```

##### ベクタを更新する
- 要素を追加
  - `push`メソッドを使用
    - 値を変化させるので、`mut`キーワードが必要
    - 追加した要素が全て`i32`型で、コンパイラはデータから型を推測し、`Vec<i32>`の注釈なし
```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

##### ベクタのドロップで、要素もドロップ
- ベクタもスコープを抜ければ、解放される

```
{
    let v = vec![1, 2, 3, 4];

    // vで作業をする

} // <- vはここでスコープを抜け、解放される
```

##### ベクタの要素を読む
- ベクタに格納された値を参照する方法は2つある
  - 索引構文
  - `get`メソッド
- インデックス：0から始まる

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

- ベクタに要素が含まれない番号の値を使用しようとした時に、 プログラムの振る舞いを選択できる
  - 索引構文：プログラムをパニックさせる
    - プログラムをクラッシュさせたい場合に最適
  - getメソッド：パニックすることなく`None`を返す
    - 大きすぎる値を誤って入力し、プログラムがNone値を得ると、現在ベクタに幾つ要素があるかをユーザに教え、 再度正しい値を入力できる（タイプミスでプログラムをクラッシュをなくせる）


```
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

- 所有権と借用規則を強制し、ベクタの中身へのこの参照や他のいかなる参照も有効である
  - 同一スコープ上では、可変と不変な参照を同時には存在させられない（ルール）
  - 下記では`error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable`
```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```

##### ベクトル内の値の反復処理
- 全要素の反復処理
  - `for`ループ
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

- 各要素への可変な参照を反復可能
  - 参照解除演算子(*)を使用してiの値を参照する必要がある

```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

##### Enumを使って複数の型を保持する
- ベクタは同じ型の値しか保持できない
- 異なる型の要素を保持する必要性が出てくるユースケースもあり、`enum`を定義して利用する
  - 副次的な利点は、 このベクタではどんな型が許容されるのか明示できること
```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

### 文字列でUTF-8でエンコードされたテキストを保持する
- 「文字列」3つの概念の組み合わせで困難に見える
  - Rustのありうるエラーを晒す性質
  - 思っている以上に文字列が複雑なデータ構造
  - UTF-8

- 文字列の実装
  - テキストとして解釈された時に有用になる機能を提供するメソッド
  - バイトの塊

##### 文字列とは？
- 1種類しか文字列型が存在しない
- 文字列スライスの`str`で、通常借用された`&str`で見かける
- 文字列スライス
  - 別の場所に格納された`UTF-8`エンコードされた文字列データへの参照
- String型は、Rustの標準ライブラリで提供。伸長可能、可変、所有権のあるUTF-8エンコードされた文字列型
- 「文字列」を指したら、`String`と文字列スライスの`&str`のことを意味

##### 新規文字列を生成する
- 空の`String`を生成する
  - 新しい空の`s`という文字列を生成している
```
let mut s = String::new();
```
リスト8-11: 新しい空のStringを生成する


- 文字列の初期値を決めるデータがあり、`to_string`メソッドを使用する
  - 文字列リテラルのように、`Display`トレイトを実装する型ならなんでも使用できる
- `to_string`メソッドを使用して文字列リテラルから`String`を生成
  - 初期値を含む文字列を生成
```
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

- `String::from`関数を使って、文字列リテラルから`String`
  - `to_string`を使うことと同義
```
let s = String::from("initial contents");
```

- `String::from`と`to_string`は全く同じで、どちらを選ぶかはスタイル次第

- 文字列はUTF-8エンコード
  - 適切にエンコードされていればどんなものでも含められる
- 様々な言語の挨拶を文字列に保持する
```
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```

##### 文字列を更新する
- `String`は、サイズを伸ばせられる
  - 追加のデータをプッシュ
  - 文字列を連結する`+`演算子
  - `format!`マクロ

###### `push_str`と`push`で文字列に追加
- `push_str`メソッドで文字列スライスを追記
  - `push_str`メソッドは、必ずしも引数の所有権を得なくていいので。文字列スライスを取る
```
let mut s = String::from("foo");
s.push_str("bar");
```

- `String`に追加した後に、文字列スライスを使用する
  - `push_str`メソッドが所有権を奪わないので、渡したあとでも出力できる
```
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```

- `push`メソッドは、1文字を引数として取り、`String`に追加
```
let mut s = String::from("lo");
s.push('l');
```

###### `+`演算子、または`format!`マクロで連結
- `+`演算子を使用して二つの`String`値を新しい`String`値にする
  - 
```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
```

- `s1`が有効でなくなることと、`s2`への参照を使用したことの理由は、 `+`演算子で呼ばれるメソッドのシグニチャ「メソッドや方など」と関係がある
  - `+`演算子は、`add`メソッドを使用している
    - s2に`&`がついているのは、`&str`を追加することしかできないから
      - `&s2`の型は、`&str`ではなく、`&String`のはずだが、コンパイラが`&String`引数を`&str`に型強制してくれる（参照外し型強制）
    - シグニチャから`add`は`self`の所有権をもらう(`&`がついていないから)
```
fn add(self, s: &str) -> String {
```

- 複数の文字列を連結する必要が出ると、`+`演算子の振る舞いは扱いにくくなる
  - `+`と`"`文字で何が起きているのかわかりにくい
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
- 複雑な文字列の連結には、format!マクロを使用する
  - `String`で返す
  - はるかに読みやすく、 引数の所有権を奪わない
```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

##### 文字列のインデックス化
- 他の多くのプログラミング言語では、文字列内の個々の文字をインデックスで参照してアクセスすることは有効できるが、Rustでインデックス構文を使って`String`の一部にアクセスしようとするとエラーが発生する

```
let s1 = String::from("hello");
let h = s1[0];
```

###### 内部表現
- `String`は`Vec<u8>`のラッパー
 - UTF-8でエンコードされている場合、それぞれ1バイトかかる
```
let hello = String::from("Hola");
```

- UTF-8で "Здравствуйте"をエンコードするのに必要なバイト数は12。文字列の各Unicodeスカラ値は2バイトのストレージを必要とする
  - 文字列のバイトへのインデックスは常に有効な`Unicode`スカラ値と相関するとは限らない
```
let hello = String::from("Здравствуйте");
```

###### バイトとスカラー値とグラフェンクラスター
- grapheme cluster (書記素クラスタ)
  - Unicodeテキストを1文字ずつ分割するアルゴリズムをUnicodeの仕様として定められている
- コンピュータが保存する生の文字列データを解釈するための様々な方法を提供しており、データがどのような人間の言語であっても、各プログラムが必要な解釈を選択できるようになっている
- Rustが文字を取得するために`String`にインデックスを作成できない理由は、インデックス作成操作には常に一定の時間がかかることが予想されるから

##### スライス文字列
- 文字列へのインデックス付けは、文字列インデックス付け操作の戻り値の型がバイト値、文字、書記素クラスタ、文字列スライスのどれであるかが明確である必要がある。
  - 文字列スライスを作成するためにインデックスを使用する必要がある場合は、より具体的にするように`Rust`は求める
  - インデックスをより具体的にして文字列スライスが必要であることを示す
   - `[]`を使って単一の数値でインデックスを作成するのではなく、`[]`に範囲を指定して特定のバイトを含む文字列スライスを作成する
```
let hello = "Здравствуйте";

let s = &hello[0..4];
```
- `s`は文字列の最初の4バイトを含む`&str`になる
  - 文字はそれぞれ2バイト
- `&hello[0...1]`を使うと、無効なインデックスがベクタでアクセスされた場合と同じように、実行時にパニックを起こす

##### 文字列を反復処理するための方法
- 個々のUnicodeスカラ値に対して操作を行う必要がある場合、最適な方法は`chars`メソッドを使う
  - 各要素にアクセスできる
```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
- bytesメソッドは各生のバイトを返す
```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```
- 有効な`UnicodePスカラ値は1バイト以上で構成されている可能性がある
- 文字列から書記素クラスタを取得するのは複雑で、標準ライブラリでは提供されていない
  - crate：https://crates.io/crates/unicode-segmentation
  - [crate.io](https://crates.io/)

##### 文字列は簡単ではない
- 文字列は複雑
- プログラミング言語は、プログラマにこの複雑さをどのように提示するかについて、異なる選択をしている
  - Rustは文字列データの正しい処理をすべてのRustプログラムのデフォルト動作にすることを選択した（トレードオフ）
  - 開発ライフサイクルの後半で非ASCII文字を含むエラーを処理する必要がなくなるようにしている

### 関連づけられた値を持つキーをハッシュマップに格納する
- ハッシュマップ
  - HashMap<K, V>型は、K型のキーとV型の値のマッピングを格納
  - ハッシュ、マップ、オブジェクト、ハッシュテーブル、辞書、連想配列などの別の名前を使用している
  - ベクトルのようにインデックスを使わずにデータを調べたいときに便利
  - https://doc.rust-lang.org/std/collections/struct.HashMap.html

##### 新しいハッシュマップの作成
- `new`で空のハッシュマップを作り、`insert`で要素を追加できる

```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

- ハッシュマップ
  - データをヒープ上に保存（ベクターと同様）
  - `String`型のキーと`i32`型の値を持つ
  - 同じ型を持たななければならない

- ハッシュマップを構築するもう一つの方法
  - タプルのベクターに対してイテレータと`collect`メソッドを使用する
- `HashMap<_, _>`
  - キーと値の型のパラメーターにはアンダースコアを使い、ベクターのデータの型に基づいてハッシュマップに含まれる型を推測することができる
```
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
```

##### ハッシュマップと所有権
- `i32`のように`Copy`トレイトを実装する型の場合、値はハッシュマップにコピーされる
- `String`のような所有する値のため、値は移動され、ハッシュマップは値の所有者になる
  - ハッシュマップに移動された後、挿入の呼び出しで変数は使用できなくなる
  - 値への参照をハッシュマップに挿入すると、値はハッシュマップに移動しません
```
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
```

##### ハッシュマップ内の値へのアクセス
- キーを`get`メソッドを提供することでハッシュマップから値を取得できる
  - `get`は`Option<&V>`を返すので、結果はSomeにある
  - ハッシュマップにそのキーの値がなければ、`get`は`None`を返す
```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
```

- ハッシュマップ内の各キーと値のペアを、ベクトルで行うのと同様の方法で、forループを使って反復処理することができる
```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

##### ハッシュマップの更新
- キーと値の数は増やすことが可能
- 各キーには一度に1つの値しか関連付けられない

###### 値の上書き
- キーと値をハッシュマップに挿入してから同じキーに異なる値を挿入する場合、そのキーに関連付けられた値は置き換えられる
```
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
```

###### キーに値がない場合にのみ値を挿入する
- 特定のキーが値を持っているかどうかをチェックし、持っていない場合はそのキーの値を挿入する
  - ハッシュマップには、チェックしたいキーをパラメータとして受け取る`entry`と呼ばれる`API`がある
  - `entry`メソッドの戻り値は、`Entry`と呼ばれる`enum`で、 「存在する」「存在しない」値を表す
  - `Entry`の`or_insert`メソッドは、対応する`Entry`キーが存在する場合は、そのキーの値への変更可能な参照を返し、存在しない場合は、このキーの新しい値としてパラメータを挿入し、新しい値への変更可能な参照を返すように定義されている
```
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
```

###### 古い値に基づく値の更新
- キーの値を検索して古い値に基づいて更新する
  - ミュータブル参照を`count`変数に格納しているので、その値に代入するためには、まずアスタリスク (*) を使用して count を参照解除する必要がある
```
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
```

##### ハッシュ関数
- デフォルトで、`HashMap`は「暗号的に強力な」ハッシュ関数を使用
  - 利用可能な最速のハッシュアルゴリズムではないが、パフォーマンスの低下に伴うセキュリティの向上と引き換えにしている
  - デフォルトのハッシュ関数があまりにも遅いと感じたら、別のハッシャーを指定することで別の関数に切り替えられる
    - default: [RandomState](https://doc.rust-lang.org/std/collections/hash_map/struct.RandomState.html)
