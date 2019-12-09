
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("someuser@email.com"), String::from("someuser123"));
    let user2 = User {
        email: String::from("Teste"),
        username: String::from("teste"),
        ..user1
    };

    let c1 = Color(0,0,0);
    let p1 = Point(0,0,0);
    println!("Hello, world! {}, {}, {}, {}", user1.email, user2.email, c1.1, p1.0);
}

