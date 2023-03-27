use crate::LayeredSettings;
use glib::{translate::*, Cast, IsA};
use std::ffi::c_void;

impl LayeredSettings {
    #[doc(alias = "panel_layered_settings_bind")]
    #[doc(alias = "panel_layered_settings_bind_with_mapping")]
    pub fn bind<'a, P: IsA<glib::Object>>(
        &'a self,
        key: &'a str,
        object: &'a P,
        property: &'a str,
    ) -> LayeredBindingBuilder<'a> {
        LayeredBindingBuilder {
            settings: self.upcast_ref(),
            key,
            object: object.upcast_ref(),
            property,
            flags: gio::SettingsBindFlags::DEFAULT,
            get_mapping: None,
            set_mapping: None,
        }
    }
}

#[must_use = "The builder must be built to be used"]
pub struct LayeredBindingBuilder<'a> {
    settings: &'a LayeredSettings,
    key: &'a str,
    object: &'a glib::Object,
    property: &'a str,
    flags: gio::SettingsBindFlags,
    #[allow(clippy::type_complexity)]
    get_mapping: Option<Box<dyn Fn(&glib::Variant, glib::Type) -> Option<glib::Value>>>,
    #[allow(clippy::type_complexity)]
    set_mapping: Option<Box<dyn Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant>>>,
}

impl<'a> LayeredBindingBuilder<'a> {
    pub fn flags(mut self, flags: gio::SettingsBindFlags) -> Self {
        self.flags = flags;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`GET`][gio::SettingsBindFlags::GET].
    pub fn get(mut self) -> Self {
        self.flags |= gio::SettingsBindFlags::GET;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`SET`][gio::SettingsBindFlags::SET].
    pub fn set(mut self) -> Self {
        self.flags |= gio::SettingsBindFlags::SET;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Unsets the default [`GET`][gio::SettingsBindFlags::GET] flag.
    pub fn set_only(mut self) -> Self {
        self.flags = (self.flags - gio::SettingsBindFlags::GET) | gio::SettingsBindFlags::SET;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Unsets the default [`SET`][gio::SettingsBindFlags::SET] flag.
    pub fn get_only(mut self) -> Self {
        self.flags = (self.flags - gio::SettingsBindFlags::SET) | gio::SettingsBindFlags::GET;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`NO_SENSITIVITY`][gio::SettingsBindFlags::NO_SENSITIVITY].
    pub fn no_sensitivity(mut self) -> Self {
        self.flags |= gio::SettingsBindFlags::NO_SENSITIVITY;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`GET_NO_CHANGES`][gio::SettingsBindFlags::GET_NO_CHANGES].
    pub fn get_no_changes(mut self) -> Self {
        self.flags |= gio::SettingsBindFlags::GET_NO_CHANGES;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to [`INVERT_BOOLEAN`][gio::SettingsBindFlags::INVERT_BOOLEAN].
    pub fn invert_boolean(mut self) -> Self {
        self.flags |= gio::SettingsBindFlags::INVERT_BOOLEAN;
        self
    }

    #[doc(alias = "get_mapping")]
    pub fn mapping<F: Fn(&glib::Variant, glib::Type) -> Option<glib::Value> + 'static>(
        mut self,
        f: F,
    ) -> Self {
        self.get_mapping = Some(Box::new(f));
        self
    }

    pub fn set_mapping<
        F: Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant> + 'static,
    >(
        mut self,
        f: F,
    ) -> Self {
        self.set_mapping = Some(Box::new(f));
        self
    }

    pub fn build(self) {
        type Mappings = (
            Option<Box<dyn Fn(&glib::Variant, glib::Type) -> Option<glib::Value>>>,
            Option<Box<dyn Fn(&glib::Value, glib::VariantType) -> Option<glib::Variant>>>,
        );
        unsafe extern "C" fn bind_with_mapping_get_trampoline(
            value: *mut glib::gobject_ffi::GValue,
            variant: *mut glib::ffi::GVariant,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let user_data = &*(user_data as *const Mappings);
            let f = user_data.0.as_ref().unwrap();
            let value = &mut *(value as *mut glib::Value);
            if let Some(v) = f(&from_glib_borrow(variant), value.type_()) {
                *value = v;
                true
            } else {
                false
            }
            .into_glib()
        }
        unsafe extern "C" fn bind_with_mapping_set_trampoline(
            value: *const glib::gobject_ffi::GValue,
            variant_type: *const glib::ffi::GVariantType,
            user_data: glib::ffi::gpointer,
        ) -> *mut glib::ffi::GVariant {
            let user_data = &*(user_data as *const Mappings);
            let f = user_data.1.as_ref().unwrap();
            let value = &*(value as *const glib::Value);
            f(value, from_glib_none(variant_type)).into_glib_ptr()
        }
        unsafe extern "C" fn destroy_closure(ptr: *mut c_void) {
            let _ = Box::<Mappings>::from_raw(ptr as *mut _);
        }

        if self.get_mapping.is_none() && self.set_mapping.is_none() {
            unsafe {
                ffi::panel_layered_settings_bind(
                    self.settings.to_glib_none().0,
                    self.key.to_glib_none().0,
                    ToGlibPtr::<*mut glib::gobject_ffi::GObject>::to_glib_none(&self.object).0
                        as *mut _,
                    self.property.to_glib_none().0,
                    self.flags.into_glib(),
                );
            }
        } else {
            let get_trampoline: Option<unsafe extern "C" fn(_, _, _) -> _> =
                if self.get_mapping.is_none() {
                    None
                } else {
                    Some(bind_with_mapping_get_trampoline)
                };
            let set_trampoline: Option<unsafe extern "C" fn(_, _, _) -> _> =
                if self.set_mapping.is_none() {
                    None
                } else {
                    Some(bind_with_mapping_set_trampoline)
                };
            let mappings: Mappings = (self.get_mapping, self.set_mapping);
            unsafe {
                ffi::panel_layered_settings_bind_with_mapping(
                    self.settings.to_glib_none().0,
                    self.key.to_glib_none().0,
                    ToGlibPtr::<*mut glib::gobject_ffi::GObject>::to_glib_none(&self.object).0
                        as *mut _,
                    self.property.to_glib_none().0,
                    self.flags.into_glib(),
                    get_trampoline,
                    set_trampoline,
                    Box::into_raw(Box::new(mappings)) as *mut c_void,
                    Some(destroy_closure),
                )
            }
        }
    }
}
