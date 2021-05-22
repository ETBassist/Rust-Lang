// nested modules; hosting and serving mods are siblings to each other, but both nested within
// the front_of_house module
mod front_of_house {
    pub mod hosting { // all items are private by default; must explicitly be made public
        // child modules have access to their parent module's behaviour, but not vice versa unless
        // declared
        pub fn add_to_waitlist() {} // even if the module is public, functions within it must
        // still be declared as public to be used outside the module

        fn seath_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// public function; as name would imply, available for use publicly (exposed API function)
pub fn eat_at_restaurant() {
    // Absolute path to module functions from crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path from within module; absolute paths are probably safer, since they are
    // less likely to break if code is moved
    front_of_house::hosting::add_to_waitlist();
}
