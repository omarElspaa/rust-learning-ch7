// A package can contain multiple binary crates and optionally one library crate.
// Scope: The nested context in which code is written has a set of names that are defined as "in scope".

// Packages: A cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module
// The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

// Most of the time when Rustaceans say "crate, " they mean library crate, and they use "crate" interchangeably with the general programming concept of a "library."


// By just adding lib.rs to your src file Rust adds new library crate with the same name of your project to the package.

// To have multiple binary crates you can either place multiple .rs files in the src/bin directory or by defining additional binaries in the Cargo.toml file. Each of these can be built as a separate executable, but they all belong to the same crate. ( You can also get rid of the whole main.rs )

// [[bin]]
// name = "first"
// path = "src/first.rs"  # This can point to any valid Rust file
// To run it cargo run --bin first


// λ cargo new restaurant --lib
// Creating library `restaurant` package
// Adding `restaurant` as member of workspace at `G:\rust\ch7`
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// To use the code in the library crate of the same project: my_project::greet;

// To use the code in the library crate of another project you should add it to your dependencies: "restaurant = { path = "../restaurant" }"

fn main() {
    println!("Hello, world!");
}