## 10. Generic Types, Traits, and Lifetimes（一般的なタイプ、トレイト、ライフタイム）

- すべてのプログラミング言語：コードの重複を効果的に処理するためのものがある
  - Rustでは、ジェネリクス
- ジェネリクス
  - 具体的な型やその他のプロパティの抽象的な代用
  - ジェネリクスの動作や他のジェネリクスとの関係を表すことができる
  - 関数は`i32`や`String`のような具体的な型の代わりに、何らかの一般的な型のパラメータを取ることができる
    - `Option<T>`や`Vec<T>`、`HashMap<K, V>`、`Result<T, E>`とジェネリクスを扱っている

##### 関数を抽出して重複を削除する
- 数字のリストで最大の数字を見つけるコード
  - 整数のリストを変数`number_list`に格納し、リストの最初の数字を変数`largest`_に格納
  - 現在の数値が最大値に格納されている数値よりも大きい場合は、その変数内の数値を置き換える
```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```


- 2つの異なる数字のリストで最大の数字を見つけるために、2つの異なる場所で同じロジックを使うように変更

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

- 最大の数を見つけるコードを`largest`関数に抽出

```
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```


- 最大の関数には`list`パラメータがあり、これは関数に渡す可能性のある`i32`値の具体的なスライスを表す

- コードを変更するために行ったステップ
  1. 重複するコードを識別する
  2. 重複するコードを関数のボディに抽出し、そのコードの入力と戻り値を関数のシグネチャで指定
  3. 代わりに関数を呼び出すために重複コードの2つのインスタンスを更新

### 一般的なデータ型

##### 関数定義

