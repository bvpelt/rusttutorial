fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
     println!("Numbers: {:#?}", numbers); // Pretty-print the array

     let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
     println!("Fruits: {:?}", fruits);
     println!("Fruits: {:#?}", fruits); // Pretty-print the array
     println!("Fruits 1st element: {}", fruits[0]);
     println!("Fruits 2nd element: {}", fruits[1]);
     println!("Fruits 3rd element: {}", fruits[2]);

     let human = ("Alice", 30, true);
     println!("Human: {:?}", human);
     println!("Human: {:#?}", human); // Pretty-print the tuple
     println!("Name: {}", human.0);
     println!("Age: {}", human.1);
     println!("Is Student: {}", human.2);

     let humanx: (&str, i32, bool) = ("Bob", 50, false);
     println!("Human: {:?}", humanx);
     println!("Human: {:#?}", humanx); // Pretty-print the tuple
     println!("Name: {}", humanx.0);
     println!("Age: {}", humanx.1);
     println!("Is Student: {}", humanx.2);

     let humany: (String, i32, bool) = ("Jack".to_string(), 45, false);
     println!("Human: {:?}", humany);
     println!("Human: {:#?}", humany); // Pretty-print the tuple
     println!("Name: {}", humany.0);
     println!("Age: {}", humany.1);
     println!("Is Student: {}", humany.2);

     let humanz = ("Jill", 28, true, [4,5,6,7,8]);
     println!("Human: {:?}", humanz);
     println!("Human: {:#?}", humanz); // Pretty-print the tuple
     println!("Name: {}", humanz.0);
     println!("Age: {}", humanz.1);
     println!("Is Student: {}", humanz.2);
     println!("Array: {:?}", humanz.3);

     let number_slice: &[i32] = &[1,2,3,4,5];
     println!("Number slice: {:?}", number_slice);

     let animals_slice: &[&str] = &["Dog", "Cat", "Rabbit"];
     println!("Animals slice: {:?}", animals_slice);

     let book_slice: &[&String] = &[&"IT".to_string(), &"Cat".to_string(), &"Rabbit".to_string()];
     println!("Book slice: {:?}", book_slice);

     // Strings stored on the heap.
     let stone_cold: String = String::from("Amsterdam, Netherlands");
     println!("Stone Cold: {}", stone_cold);
     // stone_cold.push_str(" is a great city!"); not mutable, cannot modify the string after it's created
     // println!("Stone Cold: {}", stone_cold); 
 
     let mut stone_hot: String = String::from("Amsterdam, Netherlands");
     println!("Stone Hot: {}", stone_hot);
     stone_hot.push_str(" is a great city!");
     println!("Stone Hot: {}", stone_hot);

     // Strings stored on the stack.
     // &str is a reference to a string slice, which is stored on the stack. 
     // The string cannot be modified after it's created, and it has a fixed size determined at compile time.
     let string: String = String::from("Hello, world!");
     let string_slice: &str = &string; // Create a string slice that references the string
     let string_slice1: &str = &string[0..5]; // Create a string slice that references the string
     println!("String: {}", string);
     println!("String slice: {}", string_slice); 
     println!("String slice 1: {}", string_slice1); 

     print_slice(string_slice);
 }  

 fn print_slice(slice: &str) {
    println!("print_slice Slice: {:}", slice);
 }