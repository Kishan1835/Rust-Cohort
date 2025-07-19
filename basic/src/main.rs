// fn main() {
//     let is_male: bool = true;
//     let mut is_above_18: bool = true;

//     is_above_18 = false;

//     if is_male {
//         println!("you are a male");
//     } else {
//         println!("You are not male");
//     }
//     if is_male && is_above_18 {
//         print!("you are a legal male");n
//     }
// }

// Strings

// fn main() {
//     // let ax: &str = "kishan"; // as they dont have fixed types(size), then we have something to deal with at runtime we might need more.
//     // println!("{}", ax);
//     // for example like when we want to add something new to the string(concatination), the we might have to increase the memory capacity of the string so(basically size of bits increases -> i8-i32)
//     let greeting = String::from("hello world!");
//     println!("{}", greeting);
//     let cha1 = greeting.chars().nth(1000);

// }

// Conditional and Loops

// fn main() {
//     let is_even = 5;
//     if is_even % 2 == 0 {
//         print!("The number is even ");
//     } else {
//         println!("The no is odd");
//     }
// }

// Loops

// fn main() {
//     // arrays, maps , strings

//     let sentence = String::from("  kishan ");
//     let first_word = get_first_word(sentence);
//     let n = 1000;
//     for i in 0..n {
//         println!("all number form 0-n {}", i)
//     }
//     print!("Firts word is: {}", first_word)
// }

// Complex iterations on a String in rust
// basic can be use like loop(for, while)
// fn get_first_word(sentence: String) -> String {
//     let mut ans = String::from("");
//     for char in sentence.chars() {
//         ans.push_str(char.to_string().as_str());
//         if char == ' ' {
//             break;
//         }
//     }
//     return ans;
// }

//  Functions

fn main() {
    let _result = do_sum(23, 34);
    println!("{}", _result);
}

fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
