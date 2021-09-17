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
}
