use std::mem;
use std::ops::Deref;

pub struct ComRef<T> {
    _ptr: *const T
}

impl<T> ComRef<T> {
    pub fn new(ptr: *const T) -> Self {
        ComRef { _ptr: ptr }
    }
}
 
impl<T> Deref for ComRef<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        unsafe { mem::transmute(self._ptr) }
    }
}