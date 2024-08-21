use std::ffi::{c_int, CStr, CString};
use std::ptr;
use wiredtiger_sys::{wiredtiger_open, wiredtiger_strerror, WT_CONNECTION};

fn main() {
    println!(
        "wt_version={}.{}.{}",
        wiredtiger_sys::WIREDTIGER_VERSION_MAJOR,
        wiredtiger_sys::WIREDTIGER_VERSION_MINOR,
        wiredtiger_sys::WIREDTIGER_VERSION_PATCH,
    );

    let dbpath = CString::new("/tmp/wt-example").unwrap();
    let opts = CString::new("create,statistics=(all)").unwrap();

    let mut conn: *mut WT_CONNECTION = ptr::null_mut();
    unsafe {
        make_result(wiredtiger_open(
            dbpath.as_ptr(),
            ptr::null_mut(),
            opts.as_ptr(),
            &mut conn,
        ))
        .expect("open");
    }

    eprintln!("created wiredtiger database at {dbpath:?}");
}

fn make_result(code: c_int) -> Result<(), String> {
    if code == 0 {
        return Ok(());
    }
    let s = unsafe { CStr::from_ptr(wiredtiger_strerror(code)) }
        .to_str()
        .expect("wiredtiger_strerror should return a valid utf8 string")
        .to_owned();
    Err(s)
}
