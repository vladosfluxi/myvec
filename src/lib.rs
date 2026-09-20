use std::alloc::{
    Layout,
    alloc,
    dealloc,
    realloc,
};

use std::ptr;

pub struct MyVec<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
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

    pub fn grow(&mut self) {
        // let current_cap = self.cap;
        let new_capacity: usize = if self.cap == 0 {
            4
        } else {
            self.cap * 2
        };

        let new_layout = Layout::array::<T>(new_capacity).unwrap();

        let new_raw_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(self.ptr as *mut u8, old_layout, new_layout.size()) }
        };

        if new_raw_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        let ptr = new_raw_ptr as *mut T;

        self.ptr = ptr;
        self.cap = new_capacity;
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
