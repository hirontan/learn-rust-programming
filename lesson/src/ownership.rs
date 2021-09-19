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
}
