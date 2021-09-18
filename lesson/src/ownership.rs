pub fn run() {
  // String型, Vector型, Box Pointerは所有の概念がある
  let s = String::from("hello");
  let ss = s; // sからssに所有権が移動される
  println!("{}", ss);
}
