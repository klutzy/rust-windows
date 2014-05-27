use ll::types::{LPVOID, DWORD, HRESULT};
use std::ptr;

#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;

//    fn CoCreateInstance(rclsid: REFCLSID, pUnkOuter: LPUNKNOWN, dwClsContext: DWORD,
//                        riid: REFIID, ppv: *mut LPVOID) -> HRESULT;
//
}

pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8, ..8],
}

// TODO check constness again
pub type REFGUID = *GUID;
pub type REFCLSID = *GUID;
pub type REFIID = *GUID;

//pub enum COINIT
pub static COINIT_APARTMENTTHREADED: DWORD  = 0x2;
pub static COINIT_MULTITHREADED: DWORD  = 0x0;
pub static COINIT_DISABLE_OLE1DDE: DWORD  = 0x4;
pub static COINIT_SPEED_OVER_MEMORY: DWORD  = 0x8;

pub static CLSCTX_INPROC_SERVER: DWORD = 0x1;
pub static CLSCTX_INPROC_HANDLER: DWORD = 0x2;
pub static CLSCTX_LOCAL_SERVER: DWORD = 0x4;
pub static CLSCTX_INPROC_SERVER16: DWORD = 0x8;
pub static CLSCTX_REMOTE_SERVER: DWORD = 0x10;
pub static CLSCTX_INPROC_HANDLER16: DWORD = 0x20;
pub static CLSCTX_RESERVED1: DWORD = 0x40;
pub static CLSCTX_RESERVED2: DWORD = 0x80;
pub static CLSCTX_RESERVED3: DWORD = 0x100;
pub static CLSCTX_RESERVED4: DWORD = 0x200;
pub static CLSCTX_NO_CODE_DOWNLOAD: DWORD = 0x400;
pub static CLSCTX_RESERVED5: DWORD = 0x800;
pub static CLSCTX_NO_CUSTOM_MARSHAL: DWORD = 0x1000;
pub static CLSCTX_ENABLE_CODE_DOWNLOAD: DWORD = 0x2000;
pub static CLSCTX_NO_FAILURE_LOG: DWORD = 0x4000;
pub static CLSCTX_DISABLE_AAA: DWORD = 0x8000;
pub static CLSCTX_ENABLE_AAA: DWORD = 0x10000;
pub static CLSCTX_FROM_DEFAULT_CONTEXT: DWORD = 0x20000;
pub static CLSCTX_ACTIVATE_32_BIT_SERVER: DWORD = 0x40000;
pub static CLSCTX_ACTIVATE_64_BIT_SERVER: DWORD = 0x80000;
pub static CLSCTX_ENABLE_CLOAKING: DWORD = 0x100000;
pub static CLSCTX_APPCONTAINER: DWORD = 0x400000;
pub static CLSCTX_ACTIVATE_AAA_AS_IU: DWORD = 0x800000;
pub static CLSCTX_PS_DLL: DWORD = 0x80000000;

pub static CLSCTX_ALL: DWORD = CLSCTX_INPROC_SERVER | CLSCTX_INPROC_HANDLER |
                               CLSCTX_LOCAL_SERVER | CLSCTX_REMOTE_SERVER;

pub fn init(flags: DWORD) -> Result<(), HRESULT> {
    let ret = unsafe { CoInitializeEx(ptr::mut_null(), flags) };
    if ret == 0 {
        Ok(())
    } else {
        Err(ret)
    }
}
