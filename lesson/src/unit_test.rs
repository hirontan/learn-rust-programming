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

#[cfg(test)] // テストをコンパイルしない
mod tests {
  use super::*; // 親にアクセスする

  #[test]
  fn test_a_is_larger() {
    let a = Rectangle {
      width: 5,
      height: 5,
    };
    let b = Rectangle {
      width: 3,
      height: 3,
    };
    assert!(a.compare_area(&b));
  }

  #[test]
  fn test_a_is_smaller() {
    let a = Rectangle {
      width: 3,
      height: 3,
    };
    let b = Rectangle {
      width: 5,
      height: 5,
    };
    assert!(!(a.compare_area(&b)));
  }
}
