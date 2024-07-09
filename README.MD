# HELLO RUSTY

> all tutorials are from the `EDX course` linked here~
>`RUST PROGRAMMING > RUST FUNDAMENTALS`

_*Revision\**_
> for reference this microsoft course on basics is a good start --> 
[Rust First Steps](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/)


## lec 1: familiarizing with the env
1. creating cargo projects as library and executable
2. cargo.toml file
3. makefiles and other settings for source code
4. tests for rust libraries 

## lec 2: basics
1. variables!
2. SCOPES in rust
3. help/explain commands in rust compiler
4. basic control Statements and Expressions
5. shadowing variables
6. crates are compilable units of rust code. cargo is the tool to handle rust crates

## lec 3: loops and control flow
1. loop keyword (same as while true in python)
2. Some() and Option in rust are used for handeling none values
3. while loop, for loop and it's ways
4. macros are magic!
5. vectors? damn: a growable array
6. _continue_ statement skips the itteration and moves to next itteration. _break_ statement gets out of loop
7. match (similar to case in python)

> In Rust, if no return type is specified for the main function, it implicitly returns the unit type (). The unit type is represented by an empty tuple () and signifies the absence of a meaningful value. It's used in situations where a function needs to return, but there's no significant value to return.

### STRING VS &STR

No, `&str` and `String` are not the same in Rust. They are both used to work with strings, but they have different characteristics and use cases:

1. **`&str` (String Slice):**
   - It is a reference to a string data.
   - Immutable by default.
   - Typically points to a string literal stored in the program's binary, or a slice of a `String` or another `&str`.
   - Does not own its content; it borrows it.
   - Has a fixed size, known at compile time, which includes the pointer to the string data and its length.

2. **`String`:**
   - A growable, heap-allocated data structure.
   - Can be mutated (i.e., you can add, modify, or remove characters).
   - Owns its content.
   - Its size can change at runtime.
   - When a `String` is dropped, its memory is deallocated.

**Conversion:**
- You can easily convert between `&str` and `String`:
  - To create a `String` from a `&str`, you can use `to_string()` method or the `String::from()` function.
  - To get a `&str` from a `String`, you can simply take a reference to it (e.g., `&my_string`).

**Usage:**
- Use `&str` when you need a read-only view of string data and when the string's size is known at compile time.
- Use `String` when you need to store or manipulate string data at runtime.

Here's a quick example to illustrate the conversion:

```rust
let s_literal = "hello"; // &str
let s_string = String::from(s_literal); // Convert &str to String
let s_str_from_string = &s_string; // Borrow String as &str
```

Understanding when to use each and how to convert between them is a key part of effectively working with strings in Rust.


### OUTPUT OPTIONS FOR RUST PROGRAM
In Rust, there are several ways to produce output from a program, each serving different purposes:

1. **Standard Output (stdout):**
   - Primarily used for normal program output.
   - Utilizes macros like `println!` for printing text with a newline at the end, or `print!` for printing text without a newline.
   - Example: `println!("Hello, world!");`

2. **Standard Error (stderr):**
   - Used for error messages and diagnostics.
   - Utilizes macros like `eprintln!` for printing error messages with a newline, or `eprint!` for printing error messages without a newline.
   - Example: `eprintln!("Error: Could not complete the operation.");`

3. **File Output:**
   - Writing directly to files using Rust's I/O library (`std::fs` and `std::io` modules).
   - Involves opening a file and writing to it.
   - Example:
     ```rust
     use std::fs::File;
     use std::io::Write;
     
     let mut file = File::create("output.txt").unwrap();
     writeln!(file, "Hello, file!").unwrap();
     ```

4. **Logging:**
   - Using logging libraries like `log` and `env_logger` for structured logging.
   - Logs can be configured to output to various destinations, not just the console.
   - Example:
     ```rust
     use log::{info, warn};
     
     fn main() {
         env_logger::init();
         info!("This is an info message.");
         warn!("This is a warning message.");
     }
     ```

5. **Custom Output Streams:**
   - Implementing custom traits or using existing traits like `Write` to define custom output behaviors.
   - Allows for flexible output destinations, such as network sockets, buffers, or even mock objects for testing.

6. **Panic Messages:**
   - Used for unrecoverable errors where the program cannot continue.
   - Automatically printed to stderr.
   - Example: `panic!("This is a panic message.");`

Each method serves different use cases, from simple console output to complex logging and error handling mechanisms. The choice of output method depends on the specific needs of the program, such as whether the output is meant for end-users, system administrators, or developers debugging the program.