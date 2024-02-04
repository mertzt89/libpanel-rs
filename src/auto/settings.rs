// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PanelSettings")]
    pub struct Settings(Object<ffi::PanelSettings, ffi::PanelSettingsClass>) @implements gio::ActionGroup;

    match fn {
        type_ => || ffi::panel_settings_get_type(),
    }
}

impl Settings {
    #[doc(alias = "panel_settings_new")]
    pub fn new(identifier: &str, schema_id: &str) -> Settings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_settings_new(
                identifier.to_glib_none().0,
                schema_id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_new_relocatable")]
    pub fn new_relocatable(
        identifier: &str,
        schema_id: &str,
        schema_id_prefix: &str,
        path_prefix: &str,
        path_suffix: &str,
    ) -> Settings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_settings_new_relocatable(
                identifier.to_glib_none().0,
                schema_id.to_glib_none().0,
                schema_id_prefix.to_glib_none().0,
                path_prefix.to_glib_none().0,
                path_suffix.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_new_with_path")]
    #[doc(alias = "new_with_path")]
    pub fn with_path(identifier: &str, schema_id: &str, path: &str) -> Settings {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_settings_new_with_path(
                identifier.to_glib_none().0,
                schema_id.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_get_boolean")]
    #[doc(alias = "get_boolean")]
    pub fn is_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::panel_settings_get_boolean(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_get_default_value")]
    #[doc(alias = "get_default_value")]
    pub fn default_value(&self, key: &str) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::panel_settings_get_default_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_get_double")]
    #[doc(alias = "get_double")]
    pub fn double(&self, key: &str) -> f64 {
        unsafe { ffi::panel_settings_get_double(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "panel_settings_get_int")]
    #[doc(alias = "get_int")]
    pub fn int(&self, key: &str) -> i32 {
        unsafe { ffi::panel_settings_get_int(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "panel_settings_get_string")]
    #[doc(alias = "get_string")]
    pub fn string(&self, key: &str) -> glib::GString {
        unsafe {
            from_glib_full(ffi::panel_settings_get_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_get_uint")]
    #[doc(alias = "get_uint")]
    pub fn uint(&self, key: &str) -> u32 {
        unsafe { ffi::panel_settings_get_uint(self.to_glib_none().0, key.to_glib_none().0) }
    }

    #[doc(alias = "panel_settings_get_user_value")]
    #[doc(alias = "get_user_value")]
    pub fn user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::panel_settings_get_user_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, key: &str) -> glib::Variant {
        unsafe {
            from_glib_full(ffi::panel_settings_get_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_settings_set_boolean")]
    pub fn set_boolean(&self, key: &str, val: bool) {
        unsafe {
            ffi::panel_settings_set_boolean(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.into_glib(),
            );
        }
    }

    #[doc(alias = "panel_settings_set_double")]
    pub fn set_double(&self, key: &str, val: f64) {
        unsafe {
            ffi::panel_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, val);
        }
    }

    #[doc(alias = "panel_settings_set_int")]
    pub fn set_int(&self, key: &str, val: i32) {
        unsafe {
            ffi::panel_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, val);
        }
    }

    #[doc(alias = "panel_settings_set_string")]
    pub fn set_string(&self, key: &str, val: &str) {
        unsafe {
            ffi::panel_settings_set_string(
                self.to_glib_none().0,
                key.to_glib_none().0,
                val.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_settings_set_uint")]
    pub fn set_uint(&self, key: &str, val: u32) {
        unsafe {
            ffi::panel_settings_set_uint(self.to_glib_none().0, key.to_glib_none().0, val);
        }
    }

    #[doc(alias = "panel_settings_set_value")]
    pub fn set_value(&self, key: &str, value: &glib::Variant) {
        unsafe {
            ffi::panel_settings_set_value(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_settings_unbind")]
    pub fn unbind(&self, property: &str) {
        unsafe {
            ffi::panel_settings_unbind(self.to_glib_none().0, property.to_glib_none().0);
        }
    }

    pub fn identifier(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "identifier")
    }

    pub fn path(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "path")
    }

    #[doc(alias = "path-prefix")]
    pub fn path_prefix(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "path-prefix")
    }

    #[doc(alias = "path-suffix")]
    pub fn path_suffix(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "path-suffix")
    }

    #[doc(alias = "schema-id-prefix")]
    pub fn schema_id_prefix(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "schema-id-prefix")
    }

    #[doc(alias = "panel_settings_resolve_schema_path")]
    pub fn resolve_schema_path(
        schema_id_prefix: &str,
        schema_id: &str,
        identifier: &str,
        path_prefix: &str,
        path_suffix: &str,
    ) -> glib::GString {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_settings_resolve_schema_path(
                schema_id_prefix.to_glib_none().0,
                schema_id.to_glib_none().0,
                identifier.to_glib_none().0,
                path_prefix.to_glib_none().0,
                path_suffix.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Settings, &str) + 'static>(
            this: *mut ffi::PanelSettings,
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
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
