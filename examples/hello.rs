#![feature(globs, macro_rules, phase)]

#[phase(syntax, link)]
extern crate log;

#[phase(syntax, link)]
extern crate windows = "rust-windows";

use std::ptr;
use std::cell::RefCell;
use std::default::Default;
use std::owned::Box;

use windows::main_window_loop;
use windows::ll::types::{UINT, HBRUSH};
use windows::ll::all::CREATESTRUCT;
use windows::instance::Instance;
use windows::resource::*;
use windows::window::{WindowImpl, Window, WndClass, WindowParams};
use windows::window::{OnCreate, OnSize, OnDestroy, OnPaint, OnFocus};
use windows::window;
use windows::gdi::PaintDc;
use windows::font::Font;
use windows::font;

// TMP TODO
use windows::ll::types::{HRESULT, LPCWSTR, DWORD, HWND, ULONG, c_void, LPWSTR, LPVOID, c_int};
use windows::com::{GUID, REFCLSID, REFGUID, REFIID};

extern "system" {
    fn CoCreateInstance(rclsid: REFCLSID, pUnkOuter: LPUNKNOWN, dwClsContext: DWORD,
                        riid: REFIID, ppv: *mut LPVOID) -> HRESULT;
}

// TODO
type LPUNKNOWN = *mut ();

// opaque
struct IShellItemArray;
struct IShellItemFilter;
struct IFileDialogEvents;
struct IBindCtx;

type SFGAOF = ULONG;

//enum SIGDN
static SIGDN_NORMALDISPLAY: c_int = 0x00000000;
static SIGDN_PARENTRELATIVEPARSING: c_int = 0x80018001;
static SIGDN_DESKTOPABSOLUTEPARSING: c_int = 0x80028000;
static SIGDN_PARENTRELATIVEEDITING: c_int = 0x80031001;
static SIGDN_DESKTOPABSOLUTEEDITING: c_int = 0x8004c000;
static SIGDN_FILESYSPATH: c_int = 0x80058000;
static SIGDN_URL: c_int = 0x80068000;
static SIGDN_PARENTRELATIVEFORADDRESSBAR: c_int = 0x8007c001;
static SIGDN_PARENTRELATIVE: c_int = 0x80080001;
static SIGDN_PARENTRELATIVEFORUI: c_int = 0x80094001;
type SIGDN = c_int;

//typedef struct IShellItemArrayVtbl {
//    BEGIN_INTERFACE
//
//    /*** IUnknown methods ***/
//    HRESULT (STDMETHODCALLTYPE *QueryInterface)(
//        IShellItemArray* This,
//        REFIID riid,
//        void **ppvObject);
//
//    ULONG (STDMETHODCALLTYPE *AddRef)(
//        IShellItemArray* This);
//
//    ULONG (STDMETHODCALLTYPE *Release)(
//        IShellItemArray* This);
//
//    /*** IShellItemArray methods ***/
//    HRESULT (STDMETHODCALLTYPE *BindToHandler)(
//        IShellItemArray* This,
//        IBindCtx *pbc,
//        REFGUID bhid,
//        REFIID riid,
//        void **ppvOut);
//
//    HRESULT (STDMETHODCALLTYPE *GetPropertyStore)(
//        IShellItemArray* This,
//        GETPROPERTYSTOREFLAGS flags,
//        REFIID riid,
//        void **ppv);
//
//    HRESULT (STDMETHODCALLTYPE *GetPropertyDescriptionList)(
//        IShellItemArray* This,
//        REFPROPERTYKEY keyType,
//        REFIID riid,
//        void **ppv);
//
//    HRESULT (STDMETHODCALLTYPE *GetAttributes)(
//        IShellItemArray* This,
//        SIATTRIBFLAGS AttribFlags,
//        SFGAOF sfgaoMask,
//        SFGAOF *psfgaoAttribs);
//
//    HRESULT (STDMETHODCALLTYPE *GetCount)(
//        IShellItemArray* This,
//        DWORD *pdwNumItems);
//
//    HRESULT (STDMETHODCALLTYPE *GetItemAt)(
//        IShellItemArray* This,
//        DWORD dwIndex,
//        IShellItem **ppsi);
//
//    HRESULT (STDMETHODCALLTYPE *EnumItems)(
//        IShellItemArray* This,
//        IEnumShellItems **ppenumShellItems);
//
//    END_INTERFACE
//} IShellItemArrayVtbl;

#[repr(C)]
enum FDAP {
  FDAP_BOTTOM  = 0,
  FDAP_TOP     = 1,
}

struct COMDLG_FILTERSPEC {
    pszName: LPCWSTR,
    pszSpec: LPCWSTR,
}

//#[repr(C)]
//enum _FILEOPENDIALOGOPTIONS {
//  FOS_OVERWRITEPROMPT         = 0x00000002,
//  FOS_STRICTFILETYPES         = 0x00000004,
//  FOS_NOCHANGEDIR             = 0x00000008,
//  FOS_PICKFOLDERS             = 0x00000020,
//  FOS_FORCEFILESYSTEM         = 0x00000040,
//  FOS_ALLNONSTORAGEITEMS      = 0x00000080,
//  FOS_NOVALIDATE              = 0x00000100,
//  FOS_ALLOWMULTISELECT        = 0x00000200,
//  FOS_PATHMUSTEXIST           = 0x00000800,
//  FOS_FILEMUSTEXIST           = 0x00001000,
//  FOS_CREATEPROMPT            = 0x00002000,
//  FOS_SHAREAWARE              = 0x00004000,
//  FOS_NOREADONLYRETURN        = 0x00008000,
//  FOS_NOTESTFILECREATE        = 0x00010000,
//  FOS_HIDEMRUPLACES           = 0x00020000,
//  FOS_HIDEPINNEDPLACES        = 0x00040000,
//  FOS_NODEREFERENCELINKS      = 0x00100000,
//  FOS_DONTADDTORECENT         = 0x02000000,
//  FOS_FORCESHOWHIDDEN         = 0x10000000,
//  FOS_DEFAULTNOMINIMODE       = 0x20000000,
//  FOS_FORCEPREVIEWPANEON      = 0x40000000,
//  FOS_SUPPORTSTREAMABLEITEMS  = 0x80000000
//}
type FILEOPENDIALOGOPTIONS = DWORD;

//enum _SICHINTF
static SICHINT_DISPLAY: c_int = 0x00000000;
static SICHINT_ALLFIELDS: c_int = 0x80000000;
static SICHINT_CANONICAL: c_int = 0x10000000;
static SICHINT_TEST_FILESYSPATH_IF_NOT_EQUAL: c_int = 0x20000000;
type SICHINTF = c_int;

// C++ vtable structure of IFileOpenDialog
#[allow(uppercase_variables)]
struct IFileOpenDialogVtbl {
    /*** IUnknown methods ***/
    pub QueryInterface: extern "system" fn(
        This: *mut IFileOpenDialog,
        riid: REFIID,
        ppvObject: *mut *mut c_void) -> HRESULT,

    pub AddRef: extern "system" fn(This: *mut IFileOpenDialog) -> ULONG,

    pub Release: extern "system" fn(This: *mut IFileOpenDialog) -> ULONG,

    /*** IModalWindow methods ***/
    pub Show: extern "system" fn(This: *mut IFileOpenDialog, hwndOwner: HWND) -> HRESULT,

    /*** IFileDialog methods ***/
    pub SetFileTypes: extern "system" fn(
        This: *mut IFileOpenDialog,
        cFileTypes: UINT,
        rgFilterSpec: *COMDLG_FILTERSPEC) -> HRESULT,

    pub SetFileTypeIndex: extern "system" fn(
        This: *mut IFileOpenDialog,
        iFileType: UINT) -> HRESULT,

    pub GetFileTypeIndex: extern "system" fn(
        This: *mut IFileOpenDialog,
        piFileType: *mut UINT) -> HRESULT,

    pub Advise: extern "system" fn(
        This: *mut IFileOpenDialog,
        pfde: *mut IFileDialogEvents,
        pdwCookie: *mut DWORD) -> HRESULT,

    pub Unadvise: extern "system" fn(
        This: *mut IFileOpenDialog,
        dwCookie: DWORD) -> HRESULT,

    pub SetOptions: extern "system" fn(
        This: *mut IFileOpenDialog,
        fos: FILEOPENDIALOGOPTIONS) -> HRESULT,

    pub GetOptions: extern "system" fn(
        This: *mut IFileOpenDialog,
        pfos: *mut FILEOPENDIALOGOPTIONS) -> HRESULT,

    pub SetDefaultFolder: extern "system" fn(
        This: *mut IFileOpenDialog,
        psi: *mut IShellItem) -> HRESULT,

    pub SetFolder: extern "system" fn(
        This: *mut IFileOpenDialog,
        psi: *mut IShellItem) -> HRESULT,

    pub GetFolder: extern "system" fn(
        This: *mut IFileOpenDialog,
        ppsi: *mut *mut IShellItem) -> HRESULT,

    pub GetCurrentSelection: extern "system" fn(
        This: *mut IFileOpenDialog,
        ppsi: *mut *mut IShellItem) -> HRESULT,

    pub SetFileName: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszName: LPCWSTR) -> HRESULT,

    pub GetFileName: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszName: *mut LPWSTR) -> HRESULT,

    pub SetTitle: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszTitle: LPCWSTR) -> HRESULT,

    pub SetOkButtonLabel: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszText: LPCWSTR) -> HRESULT,

    pub SetFileNameLabel: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszLabel: LPCWSTR) -> HRESULT,

    pub GetResult: extern "system" fn(
        This: *mut IFileOpenDialog,
        ppsi: *mut *mut IShellItem) -> HRESULT,

    pub AddPlace: extern "system" fn(
        This: *mut IFileOpenDialog,
        psi: *mut IShellItem,
        fdap: FDAP) -> HRESULT,

    pub SetDefaultExtension: extern "system" fn(
        This: *mut IFileOpenDialog,
        pszDefaultExtension: LPCWSTR) -> HRESULT,

    pub Close: extern "system" fn(This: *mut IFileOpenDialog, hr: HRESULT) -> HRESULT,

    pub SetClientGuid: extern "system" fn(This: *mut IFileOpenDialog, guid: REFGUID) -> HRESULT,

    pub ClearClientData: extern "system" fn(This: *mut IFileOpenDialog) -> HRESULT,

    pub SetFilter: extern "system" fn(
        This: *mut IFileOpenDialog,
        pFilter: *mut IShellItemFilter) -> HRESULT,

    /*** IFileOpenDialog methods ***/
    pub GetResults: extern "system" fn(
        This: *mut IFileOpenDialog,
        ppenum: *mut *mut IShellItemArray) -> HRESULT,

    pub GetSelectedItems: extern "system" fn(
        This: *mut IFileOpenDialog,
        ppsai: *mut *mut IShellItemArray) -> HRESULT,
}

// "opaque"
struct IFileOpenDialog {
    pub vtable: *IFileOpenDialogVtbl
}

impl IFileOpenDialog {
    fn clsid() -> &'static GUID {
        static guid: GUID = guid!(0xdc1c5a9c, 0xe88a, 0x4dde,
                                  0xa5, 0xa1, 0x60, 0xf8, 0x2a, 0x20, 0xae, 0xf7);
        &guid
    }

    fn iid() -> &'static GUID {
        static iid: GUID = guid!(0xd57c7288, 0xd4ad, 0x4768,
                                 0xbe, 0x02, 0x9d, 0x96, 0x95, 0x32, 0xd9, 0x60);
        &iid
    }
}

struct IShellItemVtbl {
    /*** IUnknown methods ***/
    pub QueryInterface: extern "system" fn(
        This: *mut IFileOpenDialog,
        riid: REFIID,
        ppvObject: *mut *mut c_void) -> HRESULT,

    pub AddRef: extern "system" fn(This: *mut IFileOpenDialog) -> ULONG,

    pub Release: extern "system" fn(This: *mut IFileOpenDialog) -> ULONG,

    /*** IShellItem methods ***/
    pub BindToHandler: extern "system" fn(
        This: *mut IShellItem,
        pbc: *mut IBindCtx,
        bhid: REFGUID,
        riid: REFIID,
        ppv: *mut *mut c_void) -> HRESULT,

    pub GetParent: extern "system" fn(
        This: *mut IShellItem,
        ppsi: *mut *mut IShellItem) -> HRESULT,

    pub GetDisplayName: extern "system" fn(
        This: *mut IShellItem,
        sigdnName: SIGDN,
        ppszName: *mut LPWSTR) -> HRESULT,

    pub GetAttributes: extern "system" fn(
        This: *mut IShellItem,
        sfgaoMask: SFGAOF,
        psfgaoAttribs: *mut SFGAOF) -> HRESULT,

    pub Compare: extern "system" fn(
        This: *mut IShellItem,
        psi: *mut IShellItem,
        hint: SICHINTF,
        piOrder: *mut c_int) -> HRESULT,
}

struct IShellItem {
    pub vtable: *IShellItemVtbl
}

impl IShellItem {
    fn guid() -> &'static GUID {
        static guid: GUID = guid!(0x43826d1e, 0xe718, 0x42ee,
                                  0xbc, 0x55, 0xa1, 0xe2, 0x61, 0xc3, 0x7b, 0xfe);
        &guid
    }
}

// TODO duplicate of hello.rc
static IDI_ICON: int = 0x101;
static MENU_MAIN: int = 0x201;
//static MENU_NEW: int = 0x202;
//static MENU_EXIT: int = 0x203;

struct MainFrame {
    win: Window,
    title: String,
    text_height: int,
    edit: RefCell<Option<Window>>,
    font: RefCell<Option<Font>>,
}

wnd_proc!(MainFrame, win, WM_CREATE, WM_DESTROY, WM_SIZE, WM_SETFOCUS, WM_PAINT)

impl OnCreate for MainFrame {
    fn on_create(&self, _cs: &CREATESTRUCT) -> bool {
        let rect = self.win.client_rect().unwrap();
        let params = WindowParams {
            window_name: "Hello World".to_strbuf(),
            style: window::WS_CHILD | window::WS_VISIBLE | window::WS_BORDER | window::WS_VSCROLL |
                window::ES_AUTOVSCROLL | window::ES_MULTILINE | window::ES_NOHIDESEL,
            x: 0,
            y: self.text_height,
            width: rect.right as int,
            height: rect.bottom as int - self.text_height,
            parent: self.win,
            menu: ptr::mut_null(),
            ex_style: 0,
        };
        let edit = Window::new(Instance::main_instance(), None, "EDIT", &params);
        match edit {
            None => false,
            Some(e) => {
                let font_attr = Default::default();
                let font = font::Font::new(&font_attr);
                debug!("font: {:?}", font);
                match font {
                    None => false,
                    Some(f) => {
                        static WM_SETFONT: UINT = 0x0030;
                        unsafe {
                            e.send_message(WM_SETFONT, std::mem::transmute(f.font), 0);
                        }
                        *self.edit.borrow_mut() = Some(e);
                        *self.font.borrow_mut() = Some(f);
                        true
                    }
                }
            }
        }
    }
}

impl OnSize for MainFrame {
    fn on_size(&self, width: int, height: int) {
        // SWP_NOOWNERZORDER | SWP_NOZORDER
        let h = self.text_height;
        self.edit.borrow().expect("edit is empty")
            .set_window_pos(0, h, width, height - h, 0x200 | 0x4);
    }
}

impl OnDestroy for MainFrame {}

impl OnPaint for MainFrame {
    fn on_paint(&self) {
        let font = self.font.borrow();
        let pdc = PaintDc::new(self).expect("Paint DC");
        pdc.dc.select_font(&font.expect("font is empty"));
        pdc.dc.text_out(0, 0, self.title.as_slice());
    }
}

impl OnFocus for MainFrame {
    fn on_focus(&self, _w: Window) {
        self.edit.borrow().expect("edit is empty").set_focus();
    }
}

impl MainFrame {
    fn new(instance: Instance, title: String, text_height: int) -> Option<Window> {
        let icon = Image::load_resource(instance, IDI_ICON, IMAGE_ICON, 0, 0);
        let wnd_class = WndClass {
            classname: "MainFrame".to_strbuf(),
            style: 0x0001 | 0x0002, // CS_HREDRAW | CS_VREDRAW
            icon: icon,
            icon_small: None,
            cursor: Image::load_cursor_resource(32514), // hourglass
            background: (5 + 1) as HBRUSH,
            menu: MenuId(MENU_MAIN),
            cls_extra: 0,
            wnd_extra: 0,
        };
        let res = wnd_class.register(instance);
        if !res {
            return None;
        }

        let wproc = box MainFrame {
            win: Window::null(),
            title: title.clone(),
            text_height: text_height,
            edit: RefCell::new(None),
            font: RefCell::new(None),
        };

        let win_params = WindowParams {
            window_name: title,
            style: window::WS_OVERLAPPEDWINDOW,
            x: 0,
            y: 0,
            width: 400,
            height: 400,
            parent: Window::null(),
            menu: ptr::mut_null(),
            ex_style: 0,
        };

        Window::new(instance, Some(wproc as Box<WindowImpl:Send>),
                    wnd_class.classname.as_slice(), &win_params)
    }
}

fn main() {
    // FIXME https://github.com/mozilla/rust/issues/13259
    unsafe { ::std::rt::stack::record_sp_limit(0); }

    let init_flags = windows::com::COINIT_APARTMENTTHREADED |
                     windows::com::COINIT_DISABLE_OLE1DDE;
    windows::com::init(init_flags).unwrap();

    let fod: *mut IFileOpenDialog = unsafe {
        let mut ptr: *mut IFileOpenDialog = ptr::mut_null();
        CoCreateInstance(IFileOpenDialog::clsid(), ptr::mut_null(),
                         windows::com::CLSCTX_ALL, IFileOpenDialog::iid(), std::mem::transmute(&mut ptr));
        ptr
    };

    unsafe {
        let vtb = (*fod).vtable;
        let ret = ((*vtb).Show)(fod, ptr::mut_null());
        println!("ret: {}", ret);
        if ret == 0 {
            let mut item: *mut IShellItem = ptr::mut_null();
            let ret = ((*vtb).GetResult)(fod, &mut item);
            if ret == 0 {
                let mut buf = [0u16, ..100];
                let mut ptr: LPWSTR = ptr::mut_null();

                println!("item: {:?}", item);
                let vtb = (*item).vtable;
                let ret = ((*vtb).GetDisplayName)(item, SIGDN_NORMALDISPLAY, &mut ptr);
                println!("ret: {:?}", ret);
                let ptr_len = std::ptr::position(ptr as *u16, |c| *c == 0);
                let p: &[u16] = std::mem::transmute((ptr, ptr_len));
                let buf = std::str::from_utf16_lossy(p);
                println!("buf: {} / ret: {}", buf, ret);
            }
        }
    }

    window::init_window_map();

    let instance = Instance::main_instance();
    let main = MainFrame::new(instance, "Hello Rust".to_strbuf(), 20);
    let main = main.unwrap();

    main.show(1);
    main.update();

    let exit_code = main_window_loop();
    std::os::set_exit_status(exit_code as int);
}
