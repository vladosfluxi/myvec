use std::alloc::{
    Layout,
    alloc,
    dealloc,
    realloc,
};

use std::{
    mem::ManuallyDrop,
    ptr,
};

use std::ops::{
    Index,
    IndexMut,
};

use std::convert::From;

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

        let last_index = self.len;

        let target_ptr = unsafe { self.ptr.add(last_index) };

        unsafe { std::ptr::write(target_ptr, value) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let last_index = self.len - 1;

        self.len -= 1;

        unsafe { Some(std::ptr::read(self.ptr.add(last_index))) }
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

impl<T> Index<usize> for MyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index + 1 > self.len {
            panic!("Index: {} out of bounds", index);
        }
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index > self.len - 1 {
            panic!("Index: {} out of bounds", index);
        }

        unsafe { &mut *self.ptr.add(index) }
    }
}

impl<T, const N: usize> From<[T; N]> for MyVec<T> {
    fn from(values: [T; N]) -> Self {
        let size = N;
        let values = ManuallyDrop::new(values);

        let mut vec: MyVec<T> = MyVec::with_capacity(size);

        unsafe {
            let src_len = values.len();
            let src_ptr = values.as_ptr();

            ptr::copy_nonoverlapping(src_ptr, vec.ptr, src_len);
        }

        vec.len = size;

        vec
    }
}
