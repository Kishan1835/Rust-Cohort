use std::io;

fn main() {
    //exercise
    // ask the user for its name and print it
    let mut name = String::new();
    println!("\nEnter Your Name Human!!\n");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name {:?}", name.trim())
}
