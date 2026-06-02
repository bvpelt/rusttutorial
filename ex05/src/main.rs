fn main() {
    println!("Ownership");

    let s1 = String::from("Rust"); // s1 is owner of the string "Rust"
    let len = calculate_length(&s1); // pass reference to s1, not ownership
    println!("s1: {}, length: {}", s1, len);

    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    let len = calculate_length(&s2);
    // println!("s1: {}", s1); // this would cause a compile error
    println!("s2: {}, length: {}", s2, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s goes out of scope and is dropped here
}
