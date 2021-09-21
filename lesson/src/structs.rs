#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn create(width: u32, height: u32) -> Self {
    // SelfがRectangleのデータ型を意味
    Self { width, height }
  }
}

pub fn run() {
  let user1 = User {
    username: String::from("test"),
    email: String::from("test@example.com"),
    sign_in_count: 1,
    active: true,
  };
  // let user2 = user1; // 所有権が移動する
  let mut user1 = User {
    username: String::from("test"),
    email: String::from("test@example.com"),
    sign_in_count: 1,
    active: true,
  };
  user1.email = String::from("test_update@example.com");
  println!("{:#?}", user1); // #をつけると見やすくなる

  let user2 = build_user(String::from("user2@example.com"), String::from("user2"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(20, 20);
  println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
  User {
    username,
    email,
    sign_in_count: 1,
    active: true,
  }
}
