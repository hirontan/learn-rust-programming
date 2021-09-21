struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
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
}
