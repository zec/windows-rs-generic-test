fn main() {
    windows::build!(
        windows::foundation::IStringable,
        windows::foundation::collections::IIterable,
        windows::foundation::collections::IIterator,

        windows::win32::debug::SetLastError,
        windows::win32::system_services::E_BOUNDS,
    );
}
