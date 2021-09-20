pub fn run() {
  let number_list = vec![34, 50, 25, 100, 101];
  let mut largest = number_list[0];

  // Vector型で格納された数値の最大値を取得する
  for number in number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest: {}", largest);
}
