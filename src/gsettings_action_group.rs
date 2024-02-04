use crate::{prelude::*, GSettingsActionGroup};
use glib::translate::*;

impl GSettingsActionGroup {
    #[doc(alias = "panel_gsettings_action_group_new")]
    pub fn new(settings: &impl IsA<gio::Settings>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            gio::ActionGroup::from_glib_full(ffi::panel_gsettings_action_group_new(
                settings.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}
