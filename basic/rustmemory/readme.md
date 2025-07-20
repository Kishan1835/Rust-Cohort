# Memory Optimirating in Rust

## When ever we run a program either in C/C++/JS,Rust it allocates and deallocates momory in RAM

## So First thing is that the there are 3 type of languages

- Garbage Collecting (JAVA, JS)
- Mannual (C)
- Rust Way (Ownerships)

### 1. Garbage Collecting Laungages are written by some genius peoples who have already cracked the code to collect the the variabled that are not in use and deacclocate the memory for better optimixcation.

### 2. Manual Languages that we allocate and deallocate memory by ourself, here we use concepts of pointer and dynamic memory allocation such as (MALLOC).

### 3. THE RUST WAY in this we use the concept of

1. Mulability
2. Heap and Stack
3. Ownerships
4. Borrowing and refrences(Pointer in Rust )
5. Lifetime

### Mutability

Here the variables cannont be changed once decleard we must use the keyword "mut" befor declearation thus it make the variable value to be changed.

BY DEFALUT IN RUST ALL VARIABLES ARE IMMULABLE, KNOWING THAT CERTAING DATA MIGHTNOT CHANGE IN FUTERR MAKE THE COMPILER OPTIMIZE THE CODE BETTER 

``` rust
fn main() {
    let mut x: i32 = 1;
    x = 2; // No error
    println!("{}", x);
}
```