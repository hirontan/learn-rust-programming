## 5. Using Structs to Structure Related Data

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
