pub fn run() {
  let st1 = String::from("x");
  let st2 = String::from("y");
}

// 大きい方の参照を返す関数
fn get_longest(x: &str, y: &str) -> &str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
