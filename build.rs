#[link(name = "kernel32")]
extern "stdcall" {
    fn GetSystemDirectoryW(
        lpBuffer: *const u16,
        uSize: u32
    ) -> u32;
    
    fn GetLastError(
        //void
    ) -> u32;
}

const MAX_PATH: u32 = 260;

fn main() {
    let buf: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let len = unsafe { ::GetSystemDirectoryW(buf.as_ptr(), MAX_PATH) };
    if len == 0 {
        let err = unsafe { ::GetLastError() };
        panic!("GetSystemDirectoryW failed with GetLastError={}", err);
    }
    
    let system_dir = String::from_utf16_lossy(&buf[..len as usize]);
    println!("cargo:rustc-link-search=native={}", system_dir);
}