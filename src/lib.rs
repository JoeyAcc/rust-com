pub mod ffi;
pub mod hr;
pub mod iid;

mod com_ref;
mod dispatch;
mod unknown;
mod vtable;

pub use com_ref::ComRef;
pub use dispatch::IDispatch;
pub use unknown::IUnknown;
pub use vtable::{Vtable, HasVtable};

pub mod abi {
    pub use unknown::{Unknown, UnknownVtbl};
    pub use dispatch::{Dispatch, DispatchVtbl};
}

//
//
//

pub struct ComInitialize {
    _priv: ()
}

pub fn initialize(init: ffi::CoInit) -> hr::HResult<ComInitialize> {
    let hr = unsafe { ffi::CoInitializeEx(std::ptr::null(), init) };
    if hr >= 0 { Ok(ComInitialize { _priv: () }) } else { Err(hr) } 
}

impl Drop for ComInitialize {
    fn drop(&mut self) {
        unsafe { ffi::CoUninitialize(); }
    }
}

//
//
//

pub fn create_instance<C, I>(context: ffi::ClsContext) -> hr::HResult<*const I>
    where C: iid::HasClsID,
          I: iid::HasIID
{
    let mut ppv = std::ptr::null();
    let hr = unsafe {
        ffi::CoCreateInstance(
            C::clsid(),
            std::ptr::null(),
            context,
            I::iid(),
            &mut ppv)
    };
    
    if hr >= 0 { Ok( ppv as *const I ) } else { Err(hr) }
}