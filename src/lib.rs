use std::alloc::{
    Layout,
    alloc,
    dealloc,
    realloc,
};

use std::fmt::{
    self,
    Debug,
};

use std::{
    mem::ManuallyDrop,
    ptr,
};

use std::iter::{
    IntoIterator,
    Iterator,
};

use std::ops::{
    Deref,
    DerefMut,
    Index,
    IndexMut,
};

use std::convert::From;

pub struct MyVec<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub cap: usize,
}

pub struct IntoIter<T> {
    ptr: *const T,
    end: *const T,
    _buf: MyVec<T>,
}

pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    _marker: std::marker::PhantomData<&'a mut T>,
}
pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: std::marker::PhantomData<&'a T>,
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

    pub fn iter(&self) -> Iter<'_, T> {
        let ptr = self.ptr as *const T;
        let end = unsafe { self.ptr.add(self.len) as *const T };

        Iter {
            ptr,
            end,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let ptr = self.ptr as *mut T;
        let end = unsafe { self.ptr.add(self.len) as *mut T };

        IterMut {
            ptr,
            end,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            while let Some(_) = self.pop() {}
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
        if index + 1 > self.len {
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

impl<T> std::iter::IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let _buf = self;

        let ptr = _buf.ptr;
        let end = unsafe { _buf.ptr.add(_buf.len) };

        IntoIter {
            ptr,
            end,
            _buf,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }
        unsafe {
            let old_ptr = self.ptr;
            self.ptr = self.ptr.add(1);

            Some(std::ptr::read(old_ptr))
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<T> FromIterator<T> for MyVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut store = MyVec::new();

        for i in iter {
            store.push(i);
        }
        store
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }
        unsafe {
            let old_ptr = self.ptr;
            self.ptr = self.ptr.add(1);

            Some(&*old_ptr)
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }
        unsafe {
            let old_ptr = self.ptr;
            self.ptr = self.ptr.add(1);

            Some(&mut *old_ptr)
        }
    }
}

impl<T: Debug> std::fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Clone> std::clone::Clone for MyVec<T> {
    fn clone(&self) -> Self {
        let mut vec: MyVec<T> = MyVec::with_capacity(self.len);

        for i in self.iter() {
            vec.push((*i).clone());
        }

        vec
    }
}
