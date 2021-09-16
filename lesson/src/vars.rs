mod sub_a;
mod sub_b;

// デフォルトではプライベート
pub fn run() {
  println!("Vars Module!");
  sub_a::func_a();
  sub_b::func_b();
}
