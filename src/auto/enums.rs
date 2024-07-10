// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::ffi;
use glib::{prelude::*, translate::*};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "PanelArea")]
pub enum Area {
    #[doc(alias = "PANEL_AREA_START")]
    Start,
    #[doc(alias = "PANEL_AREA_END")]
    End,
    #[doc(alias = "PANEL_AREA_TOP")]
    Top,
    #[doc(alias = "PANEL_AREA_BOTTOM")]
    Bottom,
    #[doc(alias = "PANEL_AREA_CENTER")]
    Center,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl IntoGlib for Area {
    type GlibType = ffi::PanelArea;

    #[inline]
    fn into_glib(self) -> ffi::PanelArea {
        match self {
            Self::Start => ffi::PANEL_AREA_START,
            Self::End => ffi::PANEL_AREA_END,
            Self::Top => ffi::PANEL_AREA_TOP,
            Self::Bottom => ffi::PANEL_AREA_BOTTOM,
            Self::Center => ffi::PANEL_AREA_CENTER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PanelArea> for Area {
    #[inline]
    unsafe fn from_glib(value: ffi::PanelArea) -> Self {
        skip_assert_initialized!();

        match value {
            ffi::PANEL_AREA_START => Self::Start,
            ffi::PANEL_AREA_END => Self::End,
            ffi::PANEL_AREA_TOP => Self::Top,
            ffi::PANEL_AREA_BOTTOM => Self::Bottom,
            ffi::PANEL_AREA_CENTER => Self::Center,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Area {
    #[inline]
    #[doc(alias = "panel_area_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::panel_area_get_type()) }
    }
}

impl glib::HasParamSpec for Area {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder_with_default
    }
}

impl glib::value::ValueType for Area {
    type Type = Self;
}

unsafe impl<'a> glib::value::FromValue<'a> for Area {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Area {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Area> for glib::Value {
    #[inline]
    fn from(v: Area) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
