//! This crate exposes automatically generated raw bindings to [SCIP](https://scipopt.org/)'s C-API. The documentation is automatically generated from the C-API docs, for further info please refer to SCIP's original [documentation](https://scipopt.org/doc/html/).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;
    use super::*;

    #[test]
    fn test_create() {
        let mut scip_ptr = MaybeUninit::uninit();
        unsafe { SCIPcreate(scip_ptr.as_mut_ptr()) };
        let mut scip_ptr = unsafe { scip_ptr.assume_init() };
        unsafe { SCIPfree(&mut scip_ptr) };
    }
}
