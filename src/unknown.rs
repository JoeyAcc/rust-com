use iid::{IID, HasIID};
use hr::HResult;
use vtable::{Vtable, HasVtable};
use std::ptr::null_mut;

#[repr(C)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct UnknownVtbl {
    QueryInterface: extern "stdcall" fn(*const Unknown, *const IID, *mut *const Unknown) -> i32,
    AddRef: extern "stdcall" fn(*const Unknown) -> u32,
    Release: extern "stdcall" fn(*const Unknown) -> u32
}

impl_iid!{ Unknown, 0x00000000, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 }

#[repr(C)]
pub struct Unknown {
    _vptr: *const UnknownVtbl
}

/// Hello, world!
pub trait IUnknown {
    fn query_interface<T>(&self) -> HResult<()/*ComRef<T>*/> where T: IUnknown + HasIID;
    
    #[doc(hidden)]
    fn _add_ref(&self) -> u32;
    
    #[doc(hidden)]
    fn _release(&self) -> u32;
}

impl<T: HasVtable<Unknown>> IUnknown for T {
    // FIXME
    fn query_interface<I>(&self) -> HResult<()/*ComRef<T>*/> where I: IUnknown + HasIID {
        let mut interface: *const Unknown = null_mut();
        let result = (self.vtable().QueryInterface)(self.this(), I::iid(), &mut interface);

        // FIXME
        //wrap_result! { result => ComRef::new(interface as *const I) }
        Err( -1 )
    }

    fn _add_ref(&self) -> u32 {
        (self.vtable().AddRef)(self.this())
    }

    fn _release(&self) -> u32 {
        (self.vtable().Release)(self.this())
    }
}

impl Vtable for Unknown { type Vtable = UnknownVtbl; }

unsafe impl HasVtable<Unknown> for Unknown { }

unsafe impl Send for Unknown {}