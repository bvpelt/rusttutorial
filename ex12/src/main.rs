#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");

    // tuples are a general way of grouping together a number of values with a variety of types into one compound type.
    let rect: (u32, u32) = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect));

    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    let mut user1 = User {
        // must be mutable to change the value of the fields
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@gmail.com"),
        sign_in_count: 1,
    };
    println!("User {:#?}", user1);

    user1.email = String::from("someusername@live.com");
    println!("User1 {:#?}", user1);

    let user2 = build_user(
        String::from("anotheruser@gmail.com"),
        String::from("anotheruser"),
    );
    println!("User2 {:#?}", user2);

    let user3 = User {
        email: String::from("newuser@m.com"),
        ..user1 // struct update syntax, creates a new instance of the struct with the same values as user2 for the fields that are not explicitly set
    };
    println!("User3 {:#?}", user3);

    // Tuple structs are similar to tuples, but they have named fields. They are useful when you want to give a name to a tuple, but you don't need to name the fields.
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    println!("black {:#?}", black);
    println!("white {:#?}", white);

    // Unit-like structs are structs that don't have any fields. They are useful when you want to implement a trait on a type that doesn't have any data associated with it.
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
    println!("subject {:#?}", subject);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
