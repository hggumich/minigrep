use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let zero = &args[0];
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
    println!("What is stored at zero {}", zero);
}
