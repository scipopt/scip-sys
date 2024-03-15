use std::mem::MaybeUninit;
use scip_sys::{SCIPcreate, SCIPfree, SCIPprintVersion};

fn main() {
    let mut scip_ptr = MaybeUninit::uninit();
    unsafe { SCIPcreate(scip_ptr.as_mut_ptr()) };
    let mut scip_ptr = unsafe { scip_ptr.assume_init() };
    unsafe {SCIPprintVersion(scip_ptr, std::ptr::null_mut())};
    unsafe { SCIPfree(&mut scip_ptr) };
}