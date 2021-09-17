// varsの親であるmainからも呼び出す場合、パブリックを設定
pub mod sub_a;
pub mod sub_b;

// 定数
const MAX_POINTS: u32 = 100_000;

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

  println!("system size: {}", usize::BITS); // システムのサイズを確認
  println!("Memory address: {:p}", &MAX_POINTS); // 「&」をつけることでメモリの番地を確認できる（ポインタの表記は、「:p」）

  // スタックに積まれる
  let ii: i64 = 1;
  let iii: i64 = 2;
  println!("Stack address of ii: {:p}", &ii);
  println!("Stack address of iii: {:p}", &iii);

  // シャドーイング
  let y = 5;
  println!("Stack address of y: {:p}", &y);

  let y = y + 1; // bind
  println!("Stack address of y: {:p}", &y);

  let y = y * 2;
  println!("Stack address of y: {:p}", &y);
  println!("The Value of y: {}", y);

  // 別のスコープを定義
  {
    let y = 0;
    println!("The Value of y: {}", y);
  }
  println!("The Value of y: {}", y);

  // タプル型
  let t = (100, 6.6, "hoge"); // 違う型を扱える
  let (x, y, z) = t;
  println!("The Value of y: {} {} {}", t.0, t.1, t.2);

  let mut tt = ((0, 2), (2, 3));
  let ((ref mut x1_p, ref mut y1_p), _) = tt; // ポイントを取得（何も受け取らない場合「_」）
  *x1_p = 5; // ポインタにアクセスする「*」
  *y1_p = -5;
  println!("Point: {:?}", tt); // プリミティブ型は、ブラケットの中は何も指定しなくても良い。構造体などを表示する場合は「:?」も指定する

  // 配列（スタックに積まれていく）
  let a = [1, 2, 3, 4, 5];
  let aa = [0; 10];
  println!("Array: {:?} {:?} {} {}", a, aa, a[2], a[3])
}
