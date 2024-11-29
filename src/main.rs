// A package can contain multiple binary crates and optionally one library crate.
// Scope: The nested context in which code is written has a set of names that are defined as "in scope".

// Packages: A cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module
// The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.


// λ cargo new restaurant --lib
// Creating library `restaurant` package
// Adding `restaurant` as member of workspace at `G:\rust\ch7`
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

fn main() {
    println!("Hello, world!");
}