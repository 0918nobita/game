//! バリデーションレイヤ関連

use ash::{version::EntryV1_0, Entry};
use once_cell::sync::Lazy;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

static VALIDATION_LAYERS: Lazy<Vec<CString>> =
    Lazy::new(|| vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()]);

pub fn check_validation_layer_support(entry: &Entry) {
    assert!(
        (*VALIDATION_LAYERS).iter().all(|layer_name| {
            entry
                .enumerate_instance_layer_properties()
                .unwrap()
                .iter()
                .any(|layer| {
                    let name = unsafe { CStr::from_ptr(layer.layer_name.as_ptr()).to_owned() };
                    name == *layer_name
                })
        }),
        "Some validation layer not supported"
    )
}

pub fn get_layer_name_ptrs() -> Vec<*const c_char> {
    (*VALIDATION_LAYERS)
        .iter()
        .map(|name| name.as_ptr())
        .collect()
}
