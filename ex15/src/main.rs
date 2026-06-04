use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut _v: Vec<i32> = Vec::new();
    let _the_vec: Vec<i32> = vec![1, 2, 3];

    _v.push(5);
    println!("The vector is: {:#?}", _v);

    _v.push(15);
    println!("The vector is: {:#?}", _v);

    let _w: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _third: &i32 = &_w[2]; // Direct indexing, will panic if index is out of bounds
    println!("The vector contains: {:?}", _w);
    println!("The third element is: {}", _third);

    println!("Element 0: {:?}", retrieve_element(&_w, 0));
    println!("Element 4: {:?}", retrieve_element(&_w, 4));
    println!("Element 5: {:?}", retrieve_element(&_w, 5));
    println!("Element 6: {:?}", retrieve_element(&_w, 6));

    let _s: String = "What is the meaning of life?".to_string();
    let _s: String = String::from("What is the meaning of life?");
    let mut _s: String = String::from("What is the meaning of life?");
    _s.push_str(" I don't know.");
    _s.push('!'); // only pushes a single character

    println!("The string is: {}", _s);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; // s1 is moved and can no longer be used
                               // println!("s1: {}", s1); // not allowed, s1 has been moved
    println!("s2: {}", s2);
    println!("Concatenated string: {}", s3);

    let s3: String = s3 + " " + &s2[2..5];
    println!("Concatenated string: {}", s3);

    // formatting strings
    let s4: String = format!("{} {}!", "Hello", "world");
    println!("Formatted string: {}", s4);
    let full_message: String = format!("{s2} {s4}");
    println!("Formatted string: {full_message}");

    // hashmaps

    let mut _scores: HashMap<String, i32> = HashMap::new();
    _scores.insert(String::from("Blue"), 10);
    _scores.insert(String::from("Yellow"), 20);
    _scores.insert(String::from("Red"), 25);
    _scores.insert(String::from("Yellow"), 26); // updates the value for "Yellow"
    _scores.insert(String::from("Orange"), 5);
    println!("Scores: {:#?}", _scores);

    let team_name: String = String::from("Blue");
    let score: Option<&i32> = _scores.get(&team_name);
    match score {
        Some(score) => println!("Score for {}: {}", team_name, score),
        None => println!("No score found for {}", team_name),
    }
    let score_value: i32 = _scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for {}: {}", team_name, score_value);

    for (key, value) in &_scores {
        println!("{key}: {value}");
    }
}

fn retrieve_element(vec: &Vec<i32>, index: usize) -> Option<&i32> {
    vec.get(index)
}
