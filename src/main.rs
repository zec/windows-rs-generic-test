extern crate windows;

#[allow(dead_code)]
mod bindings {
    ::windows::include_bindings!();

    use std::sync::Arc;
    use ::windows::{HString, Error};

    #[::windows::implement(windows::foundation::IStringable)]
    pub struct TestStringable {
        pub x: u8,
    }

    // IStringable impl
    impl TestStringable {
        pub fn to_string(&self) -> Result<HString, Error> {
            Ok("aaah.".into())
        }
    }
}

fn main() {
    println!("Hello, world!");
}
