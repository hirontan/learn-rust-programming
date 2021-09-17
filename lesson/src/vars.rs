// varsの親であるmainからも呼び出す場合、パブリックを設定
pub mod sub_a;
pub mod sub_b;

// デフォルトではプライベート
pub fn run() {
  println!("Vars Module!");
  // sub_a::func_a();
  // sub_b::func_b();

  // デフォルトではimmutable
  let mut x = 5;
  println!("x is {}", x);

  x = 6;
  println!("x is {}", x);
}
