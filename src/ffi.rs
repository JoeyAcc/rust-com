#[link(name = "ole32")]
extern "stdcall" {
    pub fn CoInitializeEx(
        pvReserved: *const (),
        dwCoInit: CoInit
    ) -> i32;
    
    pub fn CoUninitialize(
        //void
    );
    
    // HRESULT CoCreateInstance(
    //     _In_  REFCLSID  rclsid,
    //     _In_  LPUNKNOWN pUnkOuter,
    //     _In_  DWORD     dwClsContext,
    //     _In_  REFIID    riid,
    //     _Out_ LPVOID    *ppv
    // );
    pub fn CoCreateInstance(
        rclsid: *const ::iid::IID,
        pUnkOuter: *const ::abi::Unknown,
        dwClsContext: ClsContext,
        riid: *const ::iid::IID,
        ppv: &mut *const ::abi::Unknown
    ) -> i32;
}

#[repr(C)]
pub enum CoInit {
    ApartmentThreaded = 0x2,
    Multithreaded     = 0x0,
    DisableOle1Dde    = 0x4,
    SpeedOverMemory   = 0x8
}

#[repr(C)]
pub enum ClsContext { 
    InProcServer         = 0x1,
    InProcHandler        = 0x2,
    LocalServer          = 0x4,
    InProcServer16       = 0x8,
    RemoteServer         = 0x10,
    InProcHandler16      = 0x20,
    Reserved1            = 0x40,
    Reserved2            = 0x80,
    Reserved3            = 0x100,
    Reserved4            = 0x200,
    NoCodeDownload       = 0x400,
    Reserved5            = 0x800,
    NoCustomMarshal      = 0x1000,
    EnableCodeDownload   = 0x2000,
    NoFailureLog         = 0x4000,
    DisableAaa           = 0x8000,
    EnableAaa            = 0x10000,
    FromDefaultContext   = 0x20000,
    Activate32BitServer  = 0x40000,
    Activate64BitServer  = 0x80000,
    EnableCloaking       = 0x100000,
    AppContainer         = 0x400000,
    ActivateAaaAsIU      = 0x800000,
    ProxyStubDll         = 0x80000000
}