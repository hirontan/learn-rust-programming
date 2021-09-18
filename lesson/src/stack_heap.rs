// NodeのListのサイズが決まらない。そういった場合にBoxPointerが役立つ
// BoxPointerは8bytesのサイズが固定されている
enum List {
  // Node(i32, List),
  Node(i32, Box<List>), // Generics
  Nil,
}

pub fn run() {
  // Stack overflow（8MのStackを超える）
  let a1: [u8; 7000000] = [1; 7000000];

  // Vector型
  let mut v = vec![1, 2, 3, 4];
  let vv = vec![5, 6, 7, 8];
  let mut vvv = vec![9, 10];
  println!("Stack address of v: {:p}", &v);
  println!("Stack address of vv: {:p}", &vv);
  println!("Heap memory address of v: {:?}", v.as_ptr());
  println!("Len of v: {}", v.len());
  println!("Capacity of v: {}", v.capacity());
  v.insert(1, 10);
  println!("{:?}", v);
  v.remove(0);
  println!("{:?}", v);
  v.append(&mut vvv);
  println!("{:?}", v);
  println!("{:?}", vvv);

  // box pointer
  // https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
  let t: (i64, String) = (10, String::from("hello"));
  println!("Stack address of t: {:p}", &t);
  println!("Heap memory address of t.1: {:?}", t.1.as_ptr());
  println!("Length of t.1: {}", t.1.len());
  println!("Capacity of t.1: {}", t.1.capacity());
  let mut b = Box::new(t);
  (*b).1 += "world";
  println!("{} {}", b.0, b.1);
  println!("Stack address of b: {:p}", b);
}
