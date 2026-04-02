use std::ffi::CString;
use std::mem::MaybeUninit;

use scip_sys::*;

static OBJ_VALUE: std::sync::Mutex<f64> = std::sync::Mutex::new(0.0);
static STATUS: std::sync::Mutex<u32> = std::sync::Mutex::new(0);

#[no_mangle]
pub extern "C" fn scip_wasm_solve(filename_ptr: *const u8, filename_len: usize) -> i32 {
    *OBJ_VALUE.lock().unwrap() = 0.0;
    *STATUS.lock().unwrap() = 0;

    let filename = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(filename_ptr, filename_len))
    };
    let c_filename = CString::new(filename).unwrap();

    unsafe {
        let mut scip = MaybeUninit::uninit();
        let ret = SCIPcreate(scip.as_mut_ptr());
        if ret != SCIP_Retcode_SCIP_OKAY {
            return ret;
        }
        let mut scip = scip.assume_init();

        let ret = SCIPincludeDefaultPlugins(scip);
        if ret != SCIP_Retcode_SCIP_OKAY {
            SCIPfree(&mut scip);
            return ret;
        }

        let ret = SCIPreadProb(scip, c_filename.as_ptr(), std::ptr::null());
        if ret != SCIP_Retcode_SCIP_OKAY {
            SCIPfree(&mut scip);
            return ret;
        }

        let ret = SCIPsolve(scip);
        if ret != SCIP_Retcode_SCIP_OKAY {
            SCIPfree(&mut scip);
            return ret;
        }

        let sol = SCIPgetBestSol(scip);
        if !sol.is_null() {
            *OBJ_VALUE.lock().unwrap() = SCIPgetSolOrigObj(scip, sol);
        }

        *STATUS.lock().unwrap() = SCIPgetStatus(scip);

        SCIPfree(&mut scip);
        SCIP_Retcode_SCIP_OKAY
    }
}

#[no_mangle]
pub extern "C" fn scip_wasm_get_obj_value() -> f64 {
    *OBJ_VALUE.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn scip_wasm_get_status() -> u32 {
    *STATUS.lock().unwrap()
}

#[no_mangle]
pub extern "C" fn scip_wasm_alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn scip_wasm_free(ptr: *mut u8, size: usize) {
    unsafe {
        drop(Vec::from_raw_parts(ptr, 0, size));
    }
}
