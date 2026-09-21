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

    pub fn push(&mut self, value: T) {
        if self.len == 0 || self.cap <= self.len {
            self.grow();
        }

        let last_index = self.len - 1;

        let target_ptr = unsafe { self.ptr.add(last_index) };

        unsafe { std::ptr::write(target_ptr, value) };
        self.len += 1;
    }

    pub fn get_imm(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&*self.ptr.add(index)) }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        unsafe { Some(&mut *self.ptr.add(index)) }
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
