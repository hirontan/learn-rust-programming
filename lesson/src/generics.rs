struct Point<T> {
  x: T,
  y: T,
}

struct PointAnother<T, U> {
  x: T,
  y: U,
}

// インプリメンテーション
impl<T, U> PointAnother<T, U> {
  // インスタンスの情報が必要なため, selfが必要
  fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
    PointAnother {
      x: self.x,
      y: other.y,
    }
  }
}

pub fn run() {
  let number_list = vec![34, 50, 25, 100, 101];

  // 所有権が移動しているので、コメントアウト
  // println!("The largest: {}", largest_i32(number_list));

  let char_list = vec!['a', 'b', 'c', 'd'];
  println!("The largest of char_list: {}", largest(char_list));
  println!("The largest of number_list: {}", largest(number_list));

  // 構造体
  let p1 = Point { x: 1, y: 2 };
  let p2 = Point { x: 1.0, y: 2.0 }; // xとyの型は同じであると制約がかかっているので、片方がintなど違う型はできない
  let p3 = PointAnother { x: 1, y: 2.0 }; // こちらは、両方違ったデータ型を格納可能
  let p4 = PointAnother { x: "Rust", y: 'a' };
  let p5 = p3.mixup(p4);
  println!("{} {}", p5.x, p5.y);
}

// Vector型で格納された数値の最大値を取得する
fn largest_i32(list: Vec<i32>) -> i32 {
  let mut largest = list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}

// Genericsを活用して、関数定義
// <T>を指定するとあらゆるデータ型を取得できる
// Trait境界: 比較が可能なデータ型をGenericsとして許容する（今回は、PartialOrdとCopyを利用）
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
