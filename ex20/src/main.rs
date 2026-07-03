use ex20::add_one;
use ex20::{mix, PrimaryColor, SecondaryColor};

fn main() {
    println!("Hello, world!");

    let y = add_one(5);
    println!("The result is {}", y);

    let primary_color1 = PrimaryColor::Red;
    let primary_color2 = PrimaryColor::Blue;
    let answer = mix(primary_color1, primary_color2);

    if answer == SecondaryColor::Purple {
        println!("The result is Purple as expected");
    } else {
        println!("The result is not Purple");
    }
}
