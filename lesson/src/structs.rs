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
}
