pub fn run() {
  let st1 = String::from("x");
  let st2 = String::from("y");
  let res1 = get_longest(&st1, &st2);
  println!("{}", res1);

  // lifetimeが違う場合
  let st3 = String::from("x");
  let res2; // &strとなっており、型が逆算されて推論されている
  {
    let st4 = String::from("y");
    res2 = get_longest(&st3, &st4);
    println!("{}", res2);
  }
  // println!("{}", res2); // dangling pointerが発生する
}

// 大きい方の参照を返す関数
// どのLifetimeを扱えば良いのかわからないため、Generics利用して指示することができる（例: <'a>）
// xとyの短い方のLifetimeを適応する
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
