## 4. Understanding Ownership（所有権）
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

