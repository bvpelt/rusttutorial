use crate::proces::count_words;

pub mod proces;

fn log(message: &str) {
    println!("Log: {}", message);
}

pub fn process_data(data: &str) -> usize {
    log("Processing data...");
    // Simulate data processing

    let word_count = count_words(&data);
    println!("Word count: {}", word_count);

    println!("Data processed: {}", data);

    return word_count;
}
