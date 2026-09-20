use std::alloc::{
    Layout,
    alloc,
    dealloc,
};

use std::ptr;

pub struct MyVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        MyVec {
            ptr: ptr::dangling_mut(),
            len: 0,
            cap: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        let raw_ptr = unsafe { alloc(layout) };

        let mut ptr = raw_ptr as *mut T;

        MyVec {
            ptr,
            len: 0,
            cap: capacity,
        }
    }

    pub fn grow() -> Self {
    }
}
