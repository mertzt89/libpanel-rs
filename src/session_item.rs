use crate::{prelude::*, SessionItem};
use glib::translate::*;
use std::ptr;

impl SessionItem {
    #[doc(alias = "panel_session_item_get_metadata")]
    #[doc(alias = "get_metadata")]
    #[inline]
    pub fn metadata<T: FromVariant>(&self, key: &str) -> Option<T> {
        self.metadata_value(key, Some(&*T::static_variant_type()))
            .and_then(|t| t.get())
    }

    #[doc(alias = "panel_session_item_has_metadata")]
    pub fn has_metadata(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::panel_session_item_has_metadata(
                self.to_glib_none().0,
                key.to_glib_none().0,
                ptr::null_mut(),
            ))
        }
    }

    #[doc(alias = "panel_session_item_has_metadata")]
    pub fn metadata_type(&self, key: &str) -> Option<glib::VariantType> {
        unsafe {
            let mut value_type = ptr::null();
            let ret = from_glib(ffi::panel_session_item_has_metadata(
                self.to_glib_none().0,
                key.to_glib_none().0,
                &mut value_type,
            ));
            if ret {
                Some(from_glib_none(value_type))
            } else {
                None
            }
        }
    }

    #[doc(alias = "panel_session_item_set_metadata")]
    #[inline]
    pub fn set_metadata<T: ToVariant>(&self, key: &str, value: Option<T>) {
        self.set_metadata_value(key, value.map(|v| v.to_variant()).as_ref())
    }
}
