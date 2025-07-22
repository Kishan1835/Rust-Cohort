fn main() {
    for i in (1..=4).rev() {
        for _ in 0..i {
            print!("{}", i);
        }
        println!();
    }
}
