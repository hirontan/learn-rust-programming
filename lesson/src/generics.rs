pub fn run() {
  let number_list = vec![34, 50, 25, 100, 101];

  println!("The largest: {}", largest_i32(number_list));
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
