// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "PanelActionMuxer")]
    pub struct ActionMuxer(Object<ffi::PanelActionMuxer, ffi::PanelActionMuxerClass>) @implements gio::ActionGroup;

    match fn {
        type_ => || ffi::panel_action_muxer_get_type(),
    }
}

impl ActionMuxer {
    #[doc(alias = "panel_action_muxer_new")]
    pub fn new() -> ActionMuxer {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::panel_action_muxer_new()) }
    }

    #[doc(alias = "panel_action_muxer_get_action_group")]
    #[doc(alias = "get_action_group")]
    pub fn action_group(&self, prefix: &str) -> Option<gio::ActionGroup> {
        unsafe {
            from_glib_none(ffi::panel_action_muxer_get_action_group(
                self.to_glib_none().0,
                prefix.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_action_muxer_insert_action_group")]
    pub fn insert_action_group(&self, prefix: &str, action_group: &impl IsA<gio::ActionGroup>) {
        unsafe {
            ffi::panel_action_muxer_insert_action_group(
                self.to_glib_none().0,
                prefix.to_glib_none().0,
                action_group.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_action_muxer_list_groups")]
    pub fn list_groups(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::panel_action_muxer_list_groups(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_action_muxer_remove_action_group")]
    pub fn remove_action_group(&self, prefix: &str) {
        unsafe {
            ffi::panel_action_muxer_remove_action_group(
                self.to_glib_none().0,
                prefix.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_action_muxer_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::panel_action_muxer_remove_all(self.to_glib_none().0);
        }
    }
}

impl Default for ActionMuxer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ActionMuxer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActionMuxer")
    }
}
