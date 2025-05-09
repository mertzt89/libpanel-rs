// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

#[doc(alias = "panel_check_version")]
pub fn check_version(major: u32, minor: u32, micro: u32) -> bool {
    assert_initialized_main_thread!();
    unsafe { from_glib(ffi::panel_check_version(major, minor, micro)) }
}

#[doc(alias = "panel_finalize")]
pub fn finalize() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::panel_finalize();
    }
}

#[doc(alias = "panel_get_major_version")]
#[doc(alias = "get_major_version")]
pub fn major_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::panel_get_major_version() }
}

#[doc(alias = "panel_get_micro_version")]
#[doc(alias = "get_micro_version")]
pub fn micro_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::panel_get_micro_version() }
}

#[doc(alias = "panel_get_minor_version")]
#[doc(alias = "get_minor_version")]
pub fn minor_version() -> u32 {
    assert_initialized_main_thread!();
    unsafe { ffi::panel_get_minor_version() }
}

#[doc(alias = "panel_get_resource")]
#[doc(alias = "get_resource")]
pub fn resource() -> gio::Resource {
    assert_initialized_main_thread!();
    unsafe { from_glib_full(ffi::panel_get_resource()) }
}

#[doc(alias = "panel_init")]
pub fn init() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::panel_init();
    }
}
