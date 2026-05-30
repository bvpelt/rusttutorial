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
    }