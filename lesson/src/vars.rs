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

  let i = 3; // 型の推論として、デフォルトでi32が指定される
  let f = 3.0; // 型の推論として、デフォルトでf32が指定される
}
