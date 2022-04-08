struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("johndoe@example.com"), String::from("johndoe"));

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    print_user(user2);
    print_user(user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: User) {
    println!("User :\n  - email: {}\n  - username: {}\n  - active: {}\n  - sign_in_count:{}",
    user.email,
    user.username,
    user.active,
    user.sign_in_count);
}
