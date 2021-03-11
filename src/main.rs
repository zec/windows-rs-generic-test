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

    //#[::windows::implement(windows::foundation::collections::IIterator<HString>)]
    struct TestIterator {
        v: Arc<Vec<HString>>,
        idx: usize,
    }

    // IIterator impl
    impl TestIterator {
        fn current(&self) -> Result<HString, Error> {
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

        fn get_many(&mut self, items: &mut [HString]) -> Result<u32, Error> {
            let base = self.idx;
            let num_to_copy = min(self.v.len() - base, items.len());
            for i in 0..num_to_copy {
                items[i] = self.v[base + i].clone();
            }
            self.idx += num_to_copy;
            Ok(num_to_copy as u32)
        }
    }

    impl ::std::convert::From<TestIterator> for windows::foundation::collections::IIterator<HString> {
        fn from(inner: TestIterator) -> Self {
            let com = TestIterator_box::new(inner);
            unsafe {
                let ptr =
                    ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                ::std::mem::transmute_copy(&::std::ptr::NonNull::new_unchecked(&mut (*ptr).vtable.0 as  *mut _ as _))
            }
        }
    }
    #[repr(C)]
    struct TestIterator_box {
        vtable: (*const windows::foundation::collections::IIterator_abi,),
        inner: TestIterator,
        count: ::windows::RefCount,
    }
    impl TestIterator_box {
        const VTABLE: (windows::foundation::collections::IIterator_abi<HString>) =
            (windows::foundation::collections::IIterator_abi<HString>(
                Self::QueryInterface_abi0,
                Self::AddRef_abi0,
                Self::Release_abi0,
                Self::GetIids,
                Self::GetRuntimeClassName,
                Self::GetTrustLevel,
                Self::Current_abi0,
                Self::HasCurrent_abi0,
                Self::MoveNext_abi0,
                Self::GetMany_abi0,
                PhantomData<HString>
            ));
        fn new(inner: TestIterator) -> Self {
            Self {
                vtable: (&Self::VTABLE.0),
                inner,
                count: ::windows::RefCount::new(),
            }
        }
        fn QueryInterface(&mut self, iid: &::windows::Guid,
                          interface: *mut ::windows::RawPtr)
         -> ::windows::ErrorCode {
    // TODO
        }
        fn AddRef(&mut self) -> u32 { self.count.add_ref() }
        fn Release(&mut self) -> u32 {
            let remaining = self.count.release();
            if remaining == 0 {
                unsafe { ::std::boxed::Box::from_raw(self); }
            }
            remaining
        }
        unsafe extern "system" fn GetIids(_: ::windows::RawPtr,
                                          count: *mut u32,
                                          values: *mut *mut ::windows::Guid)
         -> ::windows::ErrorCode {
            *count = 0;
            *values = ::std::ptr::null_mut();
            ::windows::ErrorCode(0)
        }
        unsafe extern "system" fn GetRuntimeClassName(_: ::windows::RawPtr,
                                                      value:
                                                          *mut ::windows::RawPtr)
         -> ::windows::ErrorCode {
            let h: ::windows::HString = "Thing".into();
            *value = ::std::mem::transmute(h);
            ::windows::ErrorCode::S_OK
        }
        unsafe extern "system" fn GetTrustLevel(_: ::windows::RawPtr,
                                                value: *mut i32)
         -> ::windows::ErrorCode {
            *value = 0;
            ::windows::ErrorCode(0)
        }
        unsafe extern "system" fn QueryInterface_abi0(this: ::windows::RawPtr,
                                                      iid: &::windows::Guid,
                                                      interface:
                                                          *mut ::windows::RawPtr)
         -> ::windows::ErrorCode {
            let this =
                (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            (*this).QueryInterface(iid, interface)
        }
        unsafe extern "system" fn AddRef_abi0(this: ::windows::RawPtr)
         -> u32 {
            let this =
                (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            (*this).AddRef()
        }
        unsafe extern "system" fn Release_abi0(this: ::windows::RawPtr)
         -> u32 {
            let this =
                (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            (*this).Release()
        }
        unsafe extern "system" fn Current_abi0(
            this: ::windows::RawPtr,
            result__: *mut <T as ::windows::Abi>::Abi,
        ) -> ::windows::ErrorCode {
            let this = (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            match (*this).inner.current() {
                ::std::result::Result::Ok(ok__) => {
                    *result__ = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::ErrorCode(0)
                }
                ::std::result::Result::Err(err) => err.into(),
            }
        }
        pub  unsafe extern "system" fn HasCurrent_abi0(
            this: ::windows::RawPtr,
            result__: *mut bool,
        ) -> ::windows::ErrorCode {
            let this = (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            match (*this).inner.has_current() {
                ::std::result::Result::Ok(ok__) => {
                    *result__ = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::ErrorCode(0)
                }
                ::std::result::Result::Err(err) => err.into(),
            }
        }
        pub  unsafe extern "system" fn MoveNext_abi0(
            this: ::windows::RawPtr,
            result__: *mut bool,
        ) -> ::windows::ErrorCode {
            let this = (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            match (*this).inner.move_next() {
                ::std::result::Result::Ok(ok__) => {
                    *result__ = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::ErrorCode(0)
                }
                ::std::result::Result::Err(err) => err.into(),
            }
        }
        pub  unsafe extern "system" fn GetMany_abi0(
            this: ::windows::RawPtr,
            items_array_size: u32,
            items: *mut <T as ::windows::Abi>::Abi,
            result__: *mut u32,
        ) -> ::windows::ErrorCode {
            let this = (this as *mut ::windows::RawPtr).sub(0usize) as *mut Self;
            let mut_slice: &mut [HString] = ::str::ptr::slice_from_raw_parts(items, items_array_size as usize);
            match (*this).inner.move_next() {
                ::std::result::Result::Ok(ok__) => {
                    *result__ = ::std::mem::transmute_copy(&ok__);
                    ::std::mem::forget(ok__);
                    ::windows::ErrorCode(0)
                }
                ::std::result::Result::Err(err) => err.into(),
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
