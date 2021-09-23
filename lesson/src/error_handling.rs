pub fn run() {
  let res1 = division_option(5.0, 2.0);
  match_options(res1);
}

// Option: https://doc.rust-lang.org/std/option/enum.Option.html
// 値が存在しない場合に例外処理を行う
fn division_option(x: f64, y: f64) -> Option<f64> {
  if y == 0.0 {
    None
  } else {
    Some(x / y)
  }
}

fn match_options(result: Option<f64>) {
  match result {
    Some(x) => println!("Result: {:.3}", x),
    None => println!("Not allowed!!"),
  }
}
