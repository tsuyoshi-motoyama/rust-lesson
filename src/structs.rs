#[derive(Debug)]
struct User {
    username: String,
    email: String,
    singn_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}
pub fn run() {
    let user1 = User {
        email: String::from("fakesjoker@gmail.com"),
        username: String::from("tsuyoshi motoyama"),
        active: true,
        singn_in_count: 1,
    };
    let mut user1 = User {
        email: String::from("fakesjoker@gmail.com"),
        username: String::from("tsuyoshi motoyama"),
        active: true,
        singn_in_count: 1,
    };
    user1.email = String::from("selecthanawa@gmail.com");
    println!("{:#?}", user1);
    let user2 = build_user(String::from("user2@gmail.com"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        singn_in_count: 1,
        active: true,
    }
}
