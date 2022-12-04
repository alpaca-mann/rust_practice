fn main() {
    println!("Hello, world!");
}

/*
Creating a Rust project folder using Cargo
1. cargo new <project_name>

Creating exe file and running the exe file
1. In terminal type "cargo build" to create exe file
2. naviage to .\target\debug\hello_cargo.exe to run the exe file

OR

3. In terminal type "cargo run" to create and run the exe file

Checking for error in code base
if "cargo check" is typed in the console, rust compiler will check for code errors.
note that "cargo check" will create exe file.

Creating exe to publish a rust program
"cargo build --release"

This will produce exe file which also was optimized.
the exe file will be located in "target\release"
 */