use std::fmt;
use std::fmt::Display;

pub fn test() {
    println!("Hello, world!");
    demo_struct();
    demo_tuple_struct();
    demo_rectangle();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User: {{\n\tusername: \"{}\",\n\temail: \"{}\",\n\tsign_in_count: \"{}\",\n\tactive: \"{}\",\n}}", self.username, self.email, self.sign_in_count, self.active)
    }
}

fn demo_struct() {
    let user1 = User {
        username: String::from("nyefan"),
        email: String::from("nyefan@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = build_user(String::from("nyefan2@nyefan.com"), String::from("nyefan"));


    let user3 = User {
        username: user1.username.clone(),
        email: user1.email.clone(),
        ..user1
    };

    println!("{}", user1);
    println!("{}", user2);
    println!("{}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub struct Color(u8, u8, u8);

pub struct Point(i32, i32, i32);

struct Rectuple(u32, u32);

type RectInt = u32;

#[derive(Debug)]
pub struct Rectangle {
    width: RectInt,
    height: RectInt,
}

//impl Display for Rectangle {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        self.fmt(f)
//    }
//}

impl Rectangle {
    fn area(&self) -> RectInt {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height ||
            self.width >= other.height && self.height >= other.width
    }
    pub fn square(size: RectInt) -> Self {
        Rectangle { width: size, height: size }
    }
}

pub static BLACK: Color = Color(0, 0, 0);
pub static WHITE: Color = Color(255, 255, 255);
pub static ORIGIN: Point = Point(0, 0, 0);

fn demo_tuple_struct() {
    let rectuple = Rectuple(10, 15);

    println!("{}", area_of_rect(10, 20));
    println!("{}", area_of_rectuple(&rectuple));
}

fn area_of_rect(width: u32, height: u32) -> u32 {
    width * height
}

fn area_of_rectuple(rectuple: &Rectuple) -> u32 {
    rectuple.0 * rectuple.1
}

fn demo_rectangle() {
    let rect1 = Rectangle { width: 15, height: 15 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
//    let _rect4 = Rectangle::square(30);

    println!("{}", area_of_rectangle(&rect1));
    println!("#[derive(Debug)] on Rectangle using {{:?}}: {:?}", rect1);
    println!("#[derive(Debug)] on Rectangle using {{:#?}}: {:#?}", rect1);
    println!("{}", rect1.area());

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect2: {}", rect2.can_hold(&rect2));
    println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
}

fn area_of_rectangle(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

