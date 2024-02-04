// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "PanelGSettingsActionGroup")]
    pub struct GSettingsActionGroup(Object<ffi::PanelGSettingsActionGroup, ffi::PanelGSettingsActionGroupClass>) @implements gio::ActionGroup;

    match fn {
        type_ => || ffi::panel_gsettings_action_group_get_type(),
    }
}

impl GSettingsActionGroup {
    pub fn settings(&self) -> Option<gio::Settings> {
        ObjectExt::property(self, "settings")
    }
}
