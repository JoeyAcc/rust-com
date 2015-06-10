use hr::HResult;
use unknown::{IUnknown, Unknown};
use vtable::{Vtable, HasVtable};

#[repr(C)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct DispatchVtbl {
    base: ::abi::UnknownVtbl,
    
    // HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
    //     __RPC__in IDispatch * This,
    //     /* [out] */ __RPC__out UINT *pctinfo);
    GetTypeInfoCount: extern "stdcall" fn(*const Dispatch, &mut u32) -> i32,
    
    // HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
    //     __RPC__in IDispatch * This,
    //     /* [in] */ UINT iTInfo,
    //     /* [in] */ LCID lcid,
    //     /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
    _get_type_info: extern "stdcall" fn(*const Dispatch) -> i32,
    
    // HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
    //     __RPC__in IDispatch * This,
    //     /* [in] */ __RPC__in REFIID riid,
    //     /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
    //     /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
    //     /* [in] */ LCID lcid,
    //     /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
    _get_ids_of_names: extern "stdcall" fn(*const Dispatch) -> i32,
    
    // /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )(
    //     IDispatch * This,
    //     /* [annotation][in] */ 
    //     _In_  DISPID dispIdMember,
    //     /* [annotation][in] */ 
    //     _In_  REFIID riid,
    //     /* [annotation][in] */ 
    //     _In_  LCID lcid,
    //     /* [annotation][in] */ 
    //     _In_  WORD wFlags,
    //     /* [annotation][out][in] */ 
    //     _In_  DISPPARAMS *pDispParams,
    //     /* [annotation][out] */ 
    //     _Out_opt_  VARIANT *pVarResult,
    //     /* [annotation][out] */ 
    //     _Out_opt_  EXCEPINFO *pExcepInfo,
    //     /* [annotation][out] */ 
    //     _Out_opt_  UINT *puArgErr);
    _invoke: extern "stdcall" fn(*const Dispatch) -> i32,
}

impl_iid!{ Dispatch, 0x00020400, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 }

#[repr(C)]
pub struct Dispatch {
    _vptr: *const DispatchVtbl
}

pub trait IDispatch : IUnknown {
    fn get_type_info_count(&self) -> HResult<u32>;
    // FIXME: implement the remainings
}

impl<T: HasVtable<Dispatch> + IUnknown> IDispatch for T {
    fn get_type_info_count(&self) -> HResult<u32> {
        let mut count = 0;
        let hr = (self.vtable().GetTypeInfoCount)(self.this(), &mut count);
        
        //FIXME
        if hr >= 0 { Ok(count) } else { Err(hr) }
    }
}

impl Vtable for Dispatch { type Vtable = DispatchVtbl; }

unsafe impl HasVtable<Unknown> for Dispatch { }
unsafe impl HasVtable<Dispatch> for Dispatch { } 

unsafe impl Send for Dispatch {}