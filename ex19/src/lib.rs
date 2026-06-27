use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::thread;

// Closures

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        println!("user_preference: {:?}", user_preference);
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn closures() {
    println!("CLOSURES");
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let user_pref3 = Some(ShirtColor::Blue);
    let giveaway3 = store.giveaway(user_pref3);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref3, giveaway3
    );

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Compiler error since example_closure is bound to type String
    println!("s: {}", s);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}"); // create immutable reference to the vector list!

    println!("Before calling closure: {list:?}");
    only_borrows(); // <-- variable called as a function
    println!("After calling closure: {list:?}");

    let mut borrows_mutably = || {
        println!("From borrow: {:?}", list.push(7));
    }; // create a mutable reference to the vector list!

    borrows_mutably();
    println!("After calling closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut rlist = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 1,
            height: 5,
        },
        Rectangle {
            width: 6,
            height: 5,
        },
        Rectangle {
            width: 8,
            height: 4,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    rlist.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{rlist:#?}, sorted in {num_sort_operations} operations");

    num_sort_operations = 0;
    rlist.sort_by_key(|r| {
        num_sort_operations += 1;
        r.height
    });
    println!("{rlist:#?}, sorted in {num_sort_operations} operations");
}
// Closures

// Iterators

#[derive(PartialEq, Debug, Clone)]
struct Shoe {
    size: u32,
    style: String,
}

pub fn iterators() {
    println!("ITERATORS");
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

fn shoes_in_size<'a>(shoes: &'a [Shoe], shoe_size: u32) -> Vec<&'a Shoe> {
    shoes.iter().filter(|s| s.size == shoe_size).collect() // Retourneert Vec<&Shoe>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        // assert_eq!(v1_iter.next(), None); iter.next() can't be used after iter.sum()!!!!

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_add_one() {
        let v1: Vec<i32> = vec![1, 2, 3];

        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(&shoes, 10);

        assert_eq!(in_my_size, vec![&shoes[0], &shoes[2],]);
    }
}
// Iterators

// I/O

#[derive(PartialEq, Debug)]
struct Config {
    query: String,
    file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut args = args.iter(); // Maak een iterator van de slice

        args.next(); // returns program name, is not used

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query.clone(), // Clone omdat we &String hebben
            file_path: file_path.clone(),
            ignore_case,
        })
    }
}

pub fn io() {
    println!("IO");

    // Simuleer: cargo run -- --recurse query *.txt
    let test_args = vec![
        "my_program".to_string(), // args[0] = programma naam
        "a".to_string(),          // args[1] = zoekterm
        "b".to_string(),          // args[2] = patroon
        "--recurse".to_string(),  // args[3]
        ".".to_string(),          // args[4] = directory
    ];

    let config = Config::build(&test_args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("config: {:?}", config);
}

#[cfg(test)]
mod mytests {
    use super::*;

    #[test]
    fn config() {
        // Simuleer: cargo run -- --recurse query *.txt
        let test_args = vec![
            "my_program".to_string(), // args[0] = programma naam
            "a".to_string(),          // args[1] = zoekterm
            "b".to_string(),          // args[2] = patroon
            "--recurse".to_string(),  // args[3]
            ".".to_string(),          // args[4] = directory
        ];

        let config = Config::build(&test_args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        assert_eq!(
            config,
            Config {
                query: String::from("a"),
                file_path: String::from("b"),
                ignore_case: false,
            }
        )
    }
}
// I/O
