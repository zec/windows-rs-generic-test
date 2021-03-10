extern crate windows;

#[allow(dead_code)]
mod bindings {
    ::windows::include_bindings!();

    use std::cmp::min;
    use std::sync::Arc;
    use ::windows::{HString, Error, ErrorCode, RuntimeType};
    use self::windows::win32::debug::SetLastError;
    use self::windows::win32::system_services::E_BOUNDS;

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

    #[::windows::implement(windows::foundation::collections::IIterator<T>)]
    struct TestIterator<T: RuntimeType + Clone + 'static> {
        v: Arc<Vec<T>>,
        idx: usize,
    }

    // IIterator impl
    impl<T: RuntimeType + Clone + 'static> TestIterator<T> {
        fn current(&self) -> Result<T, Error> {
            if self.idx < self.v.len() {
                Ok(self.v[self.idx].clone())
            } else {
                unsafe { SetLastError(E_BOUNDS as u32); }
                Err(ErrorCode::from_thread().into())
            }
        }

        fn has_current(&self) -> Result<bool, Error> {
            Ok(self.idx < self.v.len())
        }

        fn move_next(&mut self) -> Result<bool, Error> {
            if self.idx >= self.v.len() {
                unsafe { SetLastError(E_BOUNDS as u32); }
                return Err(ErrorCode::from_thread().into())
            }

            self.idx += 1;
            Ok(self.idx < self.v.len())
        }

        fn get_many(&mut self, items: &mut [T]) -> Result<u32, Error> {
            let base = self.idx;
            let num_to_copy = min(self.v.len() - base, items.len());
            for i in 0..num_to_copy {
                items[i] = self.v[base + i].clone();
            }
            self.idx += num_to_copy;
            Ok(num_to_copy as u32)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
