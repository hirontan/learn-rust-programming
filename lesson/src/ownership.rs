pub fn run() {
  // String型, Vector型, Box Pointerは所有の概念がある
  let s = String::from("hello");
  let ss = s; // sからssに所有権が移動される
  println!("{}", ss);

  let i = 1;
  let ii = i; // Copy Traitが行われている
  println!("{} {}", i, ii);
  println!("Stack address of i: {:p}", &i);
  println!("Stack address of ii: {:p}", &ii);

  // 参照
  let sl1 = "literal";
  let sl2 = sl1;
  println!("{} {}", sl1, sl2);
  println!("Stack address of sl1: {:p}", &sl1);
  println!("Stack address of sl2: {:p}", &sl2);

  // Deep Copy
  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("Stack address of s3: {:p}", &s3);
  println!("Stack address of s4: {:p}", &s4);
  println!("Heap memory address of s3: {:?}", s3.as_ptr());
  println!("Heap memory address of s4: {:?}", s4.as_ptr());
  println!("{} {}", s3, s4);

  // 関数の戻り値として利用した場合、所有権が移動する
  let s5 = String::from("hello");
  println!("Stack address of s5: {:p}", &s5);
  println!("Heap memory address of s5: {:?}", s5.as_ptr());
  println!("Len of s5: {}", s5.len());
  println!("Capacity of s5: {}", s5.capacity());
  take_ownership(s5);
  // 下記は所有権が移動している
  // println!("{}", s5);

  let s6 = String::from("hello");
  println!("Stack address of s6: {:p}", &s6);
  println!("Heap memory address of s6: {:?}", s6.as_ptr());
  println!("Len of s6: {}", s6.len());
  println!("Capacity of s6: {}", s6.capacity());
  let s7 = take_giveback_ownership(s6);
  println!("Stack address of s7: {:p}", &s7);
  println!("Heap memory address of s7: {:?}", s7.as_ptr());
  println!("Len of s7: {}", s7.len());
  println!("Capacity of s7: {}", s7.capacity());

  // 参照
  let s8 = String::from("hello");
  let len = calculate_length(&s8);
  println!("The length of len: {}", len);

  // Mutableな参照
  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("{}", s9);

  // immutableな場合、参照は複数作成できる
  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;
  println!("{} {} {}", s10, r1, r2);

  // immutableな参照とmutableな参照は共存できない
  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // let r2 = &mut s10;
  // println!("{} {}", r1, r2);
}

fn take_ownership(s: String) {
  println!("Stack address of s: {:p}", &s);
  println!("Heap memory address of s: {:?}", s.as_ptr());
  println!("Len of s: {}", s.len());
  println!("Capacity of s: {}", s.capacity());
  println!("{}", s);
}

// 所有権で持っている値をそのまま返す
fn take_giveback_ownership(s: String) -> String {
  s
}

// 返り値の長さ
fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str("_world");
}
