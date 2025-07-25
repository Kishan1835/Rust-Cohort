// fn main() {
//     let s1 = String::from("Hi there ");
//     println!("The value of S1: {}", s1);
//     let s2: String = s1;
//     print!("{}", s2)
// }
// So here the value of s1 has been moved to s2 after s2 wsa asssigned to s1 so the value inside the s1 will be no longer be the owner s2 has the rights to own it so we cant use the variable s1 as it will be deallocated from the memory no variale named s1.
// here we no need any gatbage collector explicitly deallocate teh memory its done automatically.
//(A VARIABLE GOES OOUT OF SCOPE THEN THE DATA IT OWNS IT SLO GOES IUT OF SCOPE )
// By default we cant't clone a variable in rust as String is present in heap memory, if its an interger then we have to manually manage the data cloning and point it to new variable.

use std::string;

fn main() {
    let mut str = String::from("hello ");
    let s2 = &mut str;

    let s3 = &str;
    println!("{}", &s3);
    s2.push_str("world");
}

fn update_str(s: &mut String) {
    s.push_str("world");
}
