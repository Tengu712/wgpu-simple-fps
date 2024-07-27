use std::mem;

pub fn anything_to_u8slice<T>(a: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts((a as *const T).cast::<u8>(), mem::size_of::<T>()) }
}

pub fn slice_to_u8slice<T>(a: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(a.as_ptr().cast::<u8>(), mem::size_of::<T>() * a.len()) }
}
