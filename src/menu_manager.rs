use crate::MenuManager;
use glib::translate::*;
use std::path::Path;

impl MenuManager {
    #[doc(alias = "panel_menu_manager_add_filename")]
    pub fn add_filename(&self, filename: impl AsRef<Path>) -> Result<u32, glib::Error> {
        let filename = filename.as_ref();
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::panel_menu_manager_add_filename(
                self.to_glib_none().0,
                filename.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_menu_manager_add_resource")]
    pub fn add_resource(&self, resource: &str) -> Result<u32, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::panel_menu_manager_add_resource(
                self.to_glib_none().0,
                resource.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
