use std::mem;

#[repr(C)]
struct Dummy {
    pub vptr: *const ()
}
 
pub trait Vtable {
    type Vtable;
}

pub unsafe trait HasVtable<T: Vtable> : Sized {
    fn this(&self) -> &T {
        unsafe { mem::transmute(self) }
    }
    
    fn vtable(&self) -> &T::Vtable {
        unsafe {
            let d: &Dummy = mem::transmute(self);
            mem::transmute(d.vptr)
        }
    }
}