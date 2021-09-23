struct Rectangle {
  width: u32,
  height: u32,
}
impl Rectangle {
  fn compare_area(&self, other: &Rectangle) -> bool {
    (self.width * self.height) > (other.width * other.height)
  }
}

fn double_value(a: i32) -> i32 {
  a * 2
}

fn greeting(name: &str) -> String {
  format!("Hello {} san", name)
}
