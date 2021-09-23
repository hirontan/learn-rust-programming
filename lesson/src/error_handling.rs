pub fn run() {
  let res1 = division_option(5.0, 2.0);
  match_options(res1);
  let res2 = division_option(5.0, 0.0);
  match_options(res2);

  let res3 = division_result(5.0, 2.0);
  match_results(res3);
  let res4 = division_result(5.0, 0.0);
  match_results(res4);

  let a = [0, 1, 2];
  let res5 = sum(&a);
  match_sum(res5);
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

// Result: https://doc.rust-lang.org/std/result/enum.Result.html
fn division_result(x: f64, y: f64) -> Result<f64, String> {
  if y == 0.0 {
    Err(String::from("Not allowed!!"))
  } else {
    Ok(x / y)
  }
}

fn match_results(result: Result<f64, String>) {
  match result {
    Ok(x) => println!("Result: {:.3}", x),
    Err(e) => println!("{}", e),
  }
}

// 存在しないインデックスにアクセスしたとき、Out of Indexでエラーになる。その回避方法
fn sum(a: &[i32]) -> Option<i32> {
  let a0 = a.get(0)?;
  let a1 = a.get(1)?;
  let a2 = a.get(2)?; // 「?」をつけることでOut of Indexでエラーになったときに、即時にreturn(None)してくれる
  Some(a0 + a1 + a2)
}

fn match_sum(result: Option<i32>) {
  match result {
    Some(x) => println!("Total: {}", x),
    None => println!("Out of index!!"),
  }
}
