pub fn run() {
  let st1 = String::from("x");
  let st2 = String::from("y");
}

// 大きい方の参照を返す関数
// どのLifetimeを扱えば良いのかわからないため、Generics利用して指示することができる（例: <'a>）
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
