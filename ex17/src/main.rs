use ex17::NewsArticle;
use ex17::Pair;
use ex17::SocialPost;
use ex17::Summary;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("Finding the largest number in number_list using a code block");

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    println!("Finding the largest number in number_list using a function");

    largest = biggest_int(&number_list);
    println!("The largest number is {largest}");

    println!("Finding the largest char in char_list using a function");
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = biggest_char(&char_list);
    println!("The largest char is {result}");

    println!(
        "Finding the largest number or char in number_list or char_list using a generic function"
    );
    let largest_from_generic = largest_generic(&number_list);
    println!("The largest number is {largest_from_generic}");
    let result_from_generic = largest_generic(&char_list);
    println!("The largest char is {result_from_generic}");

    let string_list = vec!["Yes", "No", "Maybe", "Never", "I don't know"];
    let result_from_generic_string = largest_generic(&string_list);
    println!("The largest string is {result_from_generic_string}");

    let float_list = vec![3.14, 2.71, 1.41, 0.577];
    let result_from_generic_float = largest_generic(&float_list);
    println!("The largest float is {result_from_generic_float}");

    /*
    Direct the compiler how to compare two Points.
    Rust will then automatically compare the structs from top to bottom (first x, and if they are equal, y).
     */
    #[derive(PartialEq, PartialOrd)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = vec![
        Point { x: 5, y: 10 },
        Point { x: 5, y: 1 },
        Point { x: 15, y: 9 },
        Point { x: 25, y: 8 },
        Point { x: 35, y: 9 },
        Point { x: 35, y: 7 },
    ];

    let largest_point = largest_generic(&int_point);
    println!(
        "The largest point is ({}, {})",
        largest_point.x, largest_point.y
    );

    let smallest_point = smallest_generic(&int_point);
    println!(
        "The smallest point is ({}, {})",
        smallest_point.x, smallest_point.y
    );

    // Using traits
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("Post summary: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("Article summary: {}", article.summarize());

    // 1. Instantieer de eerste Pair met i32 waarden (x is groter)
    let pair1 = Pair::new(42, 10);

    // 2. Instantieer de tweede Pair met i32 waarden (y is groter)
    let pair2 = Pair::new(5, 99);

    println!("Vergelijking voor pair1:");
    pair1.cmp_display(); // Output: The largest member is x = 42

    println!("Vergelijking voor pair2:");
    pair2.cmp_display(); // Output: The largest member is y = 99

    //
    let pair3 = Pair::new("aap", "noot");
    let pair4 = Pair::new("karel", "appel");

    println!("Vergelijking voor pair1:");
    pair3.cmp_display(); // Output: The largest member is x = 42

    println!("Vergelijking voor pair2:");
    pair4.cmp_display(); // Output: The largest member is y = 99

    struct Book {
        pub title: String,
        pub isbn: String,
    };

    impl Clone for Book {
        fn clone(&self) -> Self {
            Self {
                title: self.title.clone(), // Kloon de String expliciet
                isbn: self.isbn.clone(),   // Kloon de String expliciet
            }
        }
    }

    let book1 = Book {
        title: String::from("Dit is het"),
        isbn: String::from("12293-33345-344"),
    };
    let book2 = Book {
        title: String::from("Dit was het"),
        isbn: String::from("99998-34537-344"),
    };
    let book3 = Book {
        title: String::from("Poezie"),
        isbn: String::from("1111-34537-344"),
    };

    //
    // book2 is copied for _pair5 so ownership is no problem when pair6 is created
    // to be able to clone, the Clone implementations must be made
    //
    // Alternative
    // add
    //   #[derive(Clone)] // Genereert automatisch de .clone() logica
    // above the struct Book declaration
    //

    let _pair5 = Pair::new(book1, book2.clone());
    let _pair6 = Pair::new(book2, book3);

    println!("Vergelijking voor pair1:");
    // pair5.cmp_display(); // compiles with error the trait `std::fmt::Display` must be implemented

    println!("Vergelijking voor pair2:");
    // pair6.cmp_display(); // compiles with error the trait `std::fmt::Display` must be implemented
}

fn biggest_int(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn biggest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
  We can use a generic function to find the largest item in a list of any type that can be compared using the `>` operator. To do this, we need to specify that the type `T` must implement the `PartialOrd` trait,
  which allows us to compare values of type `T` using the `>` operator.

  The PartialOrd trait is a trait that allows us to compare values of a type using the `>` operator.
  By specifying that `T` must implement `PartialOrd`, we can use the `>` operator to compare values of type `T` in
  our generic function.
*/
fn largest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn smallest_generic<T: PartialOrd>(list: &[T]) -> &T {
    let mut smallest = &list[0];

    for item in list {
        if item < smallest {
            smallest = item;
        }
    }

    smallest
}
