#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::ptr;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_trustm_chipinfo() {
    let return_status: u32 = unsafe { _trustm_Open() }.into();

    if return_status != OPTIGA_LIB_SUCCESS {
        println!("error opening trustm i2c interface");
    }

    println!("trustm open status {:?}", return_status);

    let m_UID = ptr::null_mut();
    let return_status: u32 = unsafe { trustm_readUID(m_UID) }.into();

    if return_status != OPTIGA_LIB_SUCCESS {
        println!("error reading trustm  UID");
    }

    let UID: _tag_utrustm_UID = unsafe { *m_UID };

    println!("Chip Identifier is {:?}", unsafe { UID.st});
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::mem;

//     #[test]
//     fn test_trustm_chipinfo() {
//         unsafe {
//             let x = _trustm_Open();

//             println!("trustm_open {:?}", x);
//         }
//     }
// }