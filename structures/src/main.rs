struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color (i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);


    let mut user1 = User {
        email: String::from("someone@me.com"),
        username: String::from("someuser123"),
        active: true,
        sign_in_count: 1
    };

    let e = String::from("email@me.com");
    let un = String::from("user2");
    let mut user2 = build_user(e, un);

    let mut user3 = User {
        email: String::from("lc@me.com"),
        ..user2
    };

    println!("{}", user3.username);

    user1.email = String::from("anotheremail@me.com");

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}