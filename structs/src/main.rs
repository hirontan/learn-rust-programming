struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let email = String::from("test@localhost.com");
    let username = String::from("test");
    let user1 = build_user(email, username);
    println!("{} / {} / {} / {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{} / {} / {} / {}", user2.email, user2.username, user2.active, user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
