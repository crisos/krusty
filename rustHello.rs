// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}

// In this file we will try to practice
// TODO

// Based on the practice Book
// Chapter 15 . Croping rules - https://doc.rust-lang.org/rust-by-example/scope.html
// RAII - Ownership and moves - Borrowing - Lifetimes


// Based on the Courses Book
// 4 - Understanding Ownership - 4.2 Borrowing
// 10.3 Lifetimes


// From https://doc.rust-lang.org/rust-by-example/scope.htm
// Multible solutions have been implementedd in RUST to take care of Memory Safety.4
// "Rust enforces RAII (Resource Acquisition Is Initialization), so whenever an object goes out of scope, its destructor is called and its owned resources are freed."
// "This behavior shields against resource leak bugs, so you'll never have to manually free memory or worry about memory leaks again!"
// RAII is a pattern that can also be used in C++

// Ressources in the heap are owned.

// There is different notions :
// Owners / Scope / Drop / Move
// Clone / Copy
// Ownership / Functions
// Return Values and Scope
// Borrowing

// Mutable references
// Dangling reference exemple at 4.2 

// Life time

