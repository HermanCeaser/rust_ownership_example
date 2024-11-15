# Rust Ownership, Borrowing, and References Demonstration

This project is a simple Rust program designed to demonstrate the core concepts of **ownership**, **borrowing**, and **references**. The program takes two strings as input, concatenates them, and prints the result to the console while adhering to Rust's ownership rules.

## Features
- Demonstrates Rust's memory safety principles.
- Uses string slices (`&str`) to borrow data without taking ownership.
- Illustrates the use of the `String` type and its associated methods.

## Prerequisites
To run this project, ensure you have the following installed:
- [Rust and Cargo](https://www.rust-lang.org/tools/install)

## Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/HermanCeaser/rust_ownership_example.git
cd rust_ownership_example
```

### 2. Build the Project
Compile the project using Cargo:
```bash
cargo build
```

### 3. Run the Project
Run the compiled program:
```bash
cargo run
```

## Program Overview

### Main Function
The main function initializes two String variables and passes their slices as references to the `concatenate_strings` function. The resulting concatenated string is then printed to the console.

### `concatenate_strings` Function

This function takes two string slices as input, concatenates them into a new String, and returns the result. It uses the `push_str() ` method to append each string slice to the result.

### Example Output
```bash
Concatenated String: Hello, World!
```

