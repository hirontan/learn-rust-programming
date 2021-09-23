pub fn run() {}

// Option: https://doc.rust-lang.org/std/option/enum.Option.html
// 値が存在しない場合に例外処理を行う
fn division_option(x: f64, y: f64) -> Option<f64> {
  if y == 0.0 {
    None
  } else {
    Some(x / y)
  }
}
