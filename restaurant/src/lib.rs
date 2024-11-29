mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// without using pub keyword this compile-time error will arise "module `hosting` is private"
// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
// Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they're defined.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}