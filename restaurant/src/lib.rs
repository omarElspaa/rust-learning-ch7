// use std::io::Error;
// use std::fmt::Error;
// error[E0252]: the name `Error` is defined multiple time

// use std::io::*;
// use std::fmt::*;
// It doesn't result an immediate error, but when you try to call Error it will result an error. error[E0659]: `Error` is ambiguous
// use std::io::Error; You can add this line to remove the ambiguity.
// This import shadows any previous imports of Error from wildcard imports.
// Key Rule: Explicit Imports Win: Rust gives priority to explicit imports over wildcard imports. This avoids ambiguity and allows you to override specific names when using wildcards.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
} // Defining Modules to Control Scope and Privacy

// without using pub keyword this compile-time error will arise "module `hosting` is private"
// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they're defined.
// If you plan on sharing your library crate so other projects can use your code, your public API is your contract with users of your crate that determines how they can interact with your code, There are many considerations around managing changes to your public API to make it easier for people to depend on your crate. These considerations are beyond the scope of this book; if you're interested in this topic, see the Rust API Guidelines at https://rust-lang.github.io/api-guidelines.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}



// We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path. This is like starting a filesystem path with the .. syntax.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}



    struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    } // In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// If we use pub before a struct definition, we make the struct public, but the struct's fields will still be private.
// All items inside the module are considered part of the same "team."
// The module is treated as an abstraction boundary. Anything outside the module needs explicit permission (pub) to access its contents, creating a clear separation between "internal details" and "external API."
// Also, note that because of back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast.


// We can create a shortcut to a path with the use keyword once, and then use the shorter name everywhere else in the scope.
// A use statement only applies in the scope it's in.
// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator.

// Bringing the function's parent module into scope ( The idiomatic way ) with use means we have to specify the function parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path.
// On the other hand, when bringing in structs, enums, and other items with use, it's idiomatic to specify the full path.

// There's no strong reason behind this idiom: it's just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
// After the path, we can specify as and a new local name, or alias, for the type.

// When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine pub and use. This technique is called re-exporting.
// If pub use is declared in a private module, the item is only accessible within the module hierarchy that has access to the private module.
// You can't re-export a private item.


// In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers. To use rand in our project, we added this line to Cargo.toml:

// rand = "0.8.5"

// Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from https://crates.io, and make rand available to our project.

// Then, to bring rand definitions into the scope of our package, we added a use line starting with the name of the crate, rand, and listed the items we wanted to bring into scope. ( You can try not using use, but you'll have to write the full path )

// Note that the standard std library is also a crate that's external to our package. Because the standard library is shipped with the Rust language, we don't need to change Cargo.toml to include std. But we do need to refer to it with use to bring items from there into our package's scope. ( but it's not on crates.io, it's downloaded on the device )

// We can use "use std::{cmp::Ordering, io};" instead of "use std::cmp::ordering;" and "use std::io;".

// We can use "use std::{self, Write};" instead of "use std::io;" and "use std::io::Write;".



// The prelude pattern refers broadly to the concept of automatically importing a set of commonly used items (like traits, types, and functions) into every Rust module. This concept includes both the default prelude provided by Rust and any custom prelude that developers might create for their projects or libraries.
// This is often called the "standard prelude" or simply the "Rust prelude."
// The prelude pattern in Rust refers to a set of commonly used items (like traits, types, and functions) that are automatically imported into every Rust module, making them readily available without needing explicit imports. This pattern is designed to improve convenience and usability for developers by providing access to the most commonly used functionality right away.
// The Rust standard library has a prelude that includes essential types and traits. For example, traits like Copy, Clone, Drop, and types like Option and Result are part of this prelude.
// This means you don't need to explicitly import these items in your code, as they are available by default.
// The prelude allows developers to use basic functionality without cluttering their code with numerous use statements, making it easier to read and write.
// Developers can create their own prelude modules to provide a similar convenience in their libraries or projects. This is particularly useful in larger codebases where certain types or traits are frequently used together.

// This is a custom prelude:

// // src/prelude.rs
// pub use std::io::{self, Read, Write};
// pub use std::collections::{HashMap, HashSet};
// pub use std::fmt::Debug;

// More common items can be added as needed

// items in rust:
// Functions
// Structs
// Enums
// Traits
// Modules
// Constants
// Static Variables
// Type Aliases
// Associated Items
// Macros

// There are several kinds of items:

// modules
// extern crate declarations
// use declarations
// function definitions
// type definitions
// struct definitions
// enumeration definitions
// union definitions
// constant items
// static items
// trait definitions
// implementations
// extern blocks

// Modules and Files: Rust allows you to organize your library into multiple files using modules, but these are purely for source code organization. During compilation, all module files are compiled into a single binary artifact.

// Output File: The output of a library crate is typically a .rlib file (e.g., libyourcrate.rlib), which contains the compiled code for all the modules in the library.

// Distribution: When you distribute your crate (e.g., via crates.io), the source code structure remains intact, but when the library is built or linked into another project, it is treated as a single unit.

// To extract modules into files, you should extract the code inside the curly brackets into a file with the same name of the module OR to a folder with the same name of the module containing a main.rs file ( and so on ( you can go to infinity ) ), and remove the curly brackets and the code inside of it.

// Note that you only need to load a file using a mod declaration once in your module tree. Once the compiler knows the file is part of the project ( and knows where in the module tree the code resides because of where you've put the mod statement ), other files in your projects should refer to the loaded file's code using a path to where it was declared, as covered in "Paths for Referring to an Item in the Module Tree" on page 125. In other words, mod is not an "include" operation that you may have seem in other programming languages.

// The key difference between Rust's mod keyword and C/C++'s #include directive is that mod declares a module and integrates it into a hierarchical module tree without copying its contents, allowing for organized code structure and single declaration usage across the project. In contrast, #include performs a literal textual inclusion of the file's contents at each inclusion point, leading to potential redundancy and lack of awareness of code structure. Rust's approach enforces compile-time safety and clarity in module ownership, while C/C++ relies on the programmer to manage file inclusions, often requiring additional mechanisms like include guards to prevent multiple inclusions.