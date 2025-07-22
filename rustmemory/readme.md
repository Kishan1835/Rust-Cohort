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

```rust
fn main() {
    let mut x: i32 = 1;
    x = 2; // No error
    println!("{}", x);
}
```

### Heap and Stack

So the concept of Heap&Stack comes under Memory management where all the allocated variables are stored on the RAM(Random access Memory) but sometimes the accessability of the variables may takes times, So these times may be very much valuable and the time fro example like transaction that may take some time as based on teh ACID property.
Variables that may change in future like value of (transactions-Amount) so these variable has to be allocated in memory but these has to be fixed size but whene we store these in RAM(stack) but as thus memory has already fixed we have to store data in the Heap memory that is disoriented way where we cannnot find the data so we intend to store the poiter to the variable where it is stored on the heap memory, their it will be stored in a contiguous memory location.

```rust
fn main() {
    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
}
```

