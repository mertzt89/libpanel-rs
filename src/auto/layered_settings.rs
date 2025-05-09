// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PanelLayeredSettings")]
    pub struct LayeredSettings(Object<ffi::PanelLayeredSettings, ffi::PanelLayeredSettingsClass>);

    match fn {
        type_ => || ffi::panel_layered_settings_get_type(),
    }
}

impl LayeredSettings {
    #[doc(alias = "panel_layered_settings_new")]
    pub fn new(schema_id: &str, path: &str) -> LayeredSettings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_layered_settings_new(
                schema_id.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_append")]
    pub fn append(&self, settings: &impl IsA<gio::Settings>) {
        unsafe {
            ffi::panel_layered_settings_append(
                self.to_glib_none().0,
                settings.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_layered_settings_get_boolean")]
    #[doc(alias = "get_boolean")]
    pub fn is_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::panel_layered_settings_get_boolean(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_get_default_value")]
    #[doc(alias = "get_default_value")]
    pub fn default_value(&self, key: &str) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::panel_layered_settings_get_default_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_get_double")]
    #[doc(alias = "get_double")]
    pub fn double(&self, key: &str) -> f64 {
        unsafe {
            ffi::panel_layered_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[doc(alias = "panel_layered_settings_get_int")]
    #[doc(alias = "get_int")]
    pub fn int(&self, key: &str) -> i32 {
        unsafe { ffi::panel_layered_settings_get_int(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "panel_layered_settings_get_key")]
    #[doc(alias = "get_key")]
    pub fn key(&self, key: &str) -> gio::SettingsSchemaKey {
        unsafe {
            from_glib_full(ffi::panel_layered_settings_get_key(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self, key: &str) -> glib::GString {
        unsafe {
            from_glib_full(ffi::panel_layered_settings_get_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_get_uint")]
    #[doc(alias = "get_uint")]
    pub fn uint(&self, key: &str) -> u32 {
        unsafe { ffi::panel_layered_settings_get_uint(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "panel_layered_settings_get_user_value")]
    #[doc(alias = "get_user_value")]
    pub fn user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::panel_layered_settings_get_user_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, key: &str) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::panel_layered_settings_get_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_list_keys")]
    pub fn list_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::panel_layered_settings_list_keys(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_layered_settings_set_boolean")]
    pub fn set_boolean(&self, key: &str, val: bool) {
        unsafe {
            ffi::panel_layered_settings_set_boolean(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.into_glib(),
            );
        }
    }

    #[doc(alias = "panel_layered_settings_set_double")]
    pub fn set_double(&self, key: &str, val: f64) {
        unsafe {
            ffi::panel_layered_settings_set_double(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val,
            );
        }
    }

    #[doc(alias = "panel_layered_settings_set_int")]
    pub fn set_int(&self, key: &str, val: i32) {
        unsafe {
            ffi::panel_layered_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, val);
        }
    }

    #[doc(alias = "panel_layered_settings_set_string")]
    pub fn set_string(&self, key: &str, val: &str) {
        unsafe {
            ffi::panel_layered_settings_set_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_layered_settings_set_uint")]
    pub fn set_uint(&self, key: &str, val: u32) {
        unsafe {
            ffi::panel_layered_settings_set_uint(self.to_glib_none().0, key.to_glib_none().0, val);
        }
    }

    #[doc(alias = "panel_layered_settings_set_value")]
    pub fn set_value(&self, key: &str, value: &glib::Variant) {
        unsafe {
            ffi::panel_layered_settings_set_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_layered_settings_unbind")]
    pub fn unbind(&self, property: &str) {
        unsafe {
            ffi::panel_layered_settings_unbind(self.to_glib_none().0, property.to_glib_none().0);
        }
    }

    pub fn path(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "path")
    }

    #[doc(alias = "schema-id")]
    pub fn schema_id(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "schema-id")
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&LayeredSettings, &str) + 'static>(
            this: *mut ffi::PanelLayeredSettings,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("changed::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"changed\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
