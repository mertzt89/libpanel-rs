// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Area;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "PanelPosition")]
    pub struct Position(Object<ffi::PanelPosition, ffi::PanelPositionClass>);

    match fn {
        type_ => || ffi::panel_position_get_type(),
    }
}

impl Position {
    #[doc(alias = "panel_position_new")]
    pub fn new() -> Position {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::panel_position_new()) }
    }

    #[doc(alias = "panel_position_new_from_variant")]
    #[doc(alias = "new_from_variant")]
    pub fn from_variant(variant: &glib::Variant) -> Position {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::panel_position_new_from_variant(
                variant.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Position`] objects.
    ///
    /// This method returns an instance of [`PositionBuilder`](crate::builders::PositionBuilder) which can be used to create [`Position`] objects.
    pub fn builder() -> PositionBuilder {
        PositionBuilder::default()
    }

    #[doc(alias = "panel_position_equal")]
    pub fn equal(&self, b: &Position) -> bool {
        unsafe {
            from_glib(ffi::panel_position_equal(
                self.to_glib_none().0,
                b.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_position_get_area")]
    #[doc(alias = "get_area")]
    pub fn area(&self) -> Area {
        unsafe { from_glib(ffi::panel_position_get_area(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_get_area_set")]
    #[doc(alias = "get_area_set")]
    pub fn is_area_set(&self) -> bool {
        unsafe { from_glib(ffi::panel_position_get_area_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_get_column")]
    #[doc(alias = "get_column")]
    pub fn column(&self) -> u32 {
        unsafe { ffi::panel_position_get_column(self.to_glib_none().0) }
    }

    #[doc(alias = "panel_position_get_column_set")]
    #[doc(alias = "get_column_set")]
    pub fn is_column_set(&self) -> bool {
        unsafe { from_glib(ffi::panel_position_get_column_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_get_depth")]
    #[doc(alias = "get_depth")]
    pub fn depth(&self) -> u32 {
        unsafe { ffi::panel_position_get_depth(self.to_glib_none().0) }
    }

    #[doc(alias = "panel_position_get_depth_set")]
    #[doc(alias = "get_depth_set")]
    pub fn is_depth_set(&self) -> bool {
        unsafe { from_glib(ffi::panel_position_get_depth_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_get_row")]
    #[doc(alias = "get_row")]
    pub fn row(&self) -> u32 {
        unsafe { ffi::panel_position_get_row(self.to_glib_none().0) }
    }

    #[doc(alias = "panel_position_get_row_set")]
    #[doc(alias = "get_row_set")]
    pub fn is_row_set(&self) -> bool {
        unsafe { from_glib(ffi::panel_position_get_row_set(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_is_indeterminate")]
    pub fn is_indeterminate(&self) -> bool {
        unsafe { from_glib(ffi::panel_position_is_indeterminate(self.to_glib_none().0)) }
    }

    #[doc(alias = "panel_position_set_area")]
    pub fn set_area(&self, area: Area) {
        unsafe {
            ffi::panel_position_set_area(self.to_glib_none().0, area.into_glib());
        }
    }

    #[doc(alias = "panel_position_set_area_set")]
    pub fn set_area_set(&self, area_set: bool) {
        unsafe {
            ffi::panel_position_set_area_set(self.to_glib_none().0, area_set.into_glib());
        }
    }

    #[doc(alias = "panel_position_set_column")]
    pub fn set_column(&self, column: u32) {
        unsafe {
            ffi::panel_position_set_column(self.to_glib_none().0, column);
        }
    }

    #[doc(alias = "panel_position_set_column_set")]
    pub fn set_column_set(&self, column_set: bool) {
        unsafe {
            ffi::panel_position_set_column_set(self.to_glib_none().0, column_set.into_glib());
        }
    }

    #[doc(alias = "panel_position_set_depth")]
    pub fn set_depth(&self, depth: u32) {
        unsafe {
            ffi::panel_position_set_depth(self.to_glib_none().0, depth);
        }
    }

    #[doc(alias = "panel_position_set_depth_set")]
    pub fn set_depth_set(&self, depth_set: bool) {
        unsafe {
            ffi::panel_position_set_depth_set(self.to_glib_none().0, depth_set.into_glib());
        }
    }

    #[doc(alias = "panel_position_set_row")]
    pub fn set_row(&self, row: u32) {
        unsafe {
            ffi::panel_position_set_row(self.to_glib_none().0, row);
        }
    }

    #[doc(alias = "panel_position_set_row_set")]
    pub fn set_row_set(&self, row_set: bool) {
        unsafe {
            ffi::panel_position_set_row_set(self.to_glib_none().0, row_set.into_glib());
        }
    }

    #[doc(alias = "panel_position_to_variant")]
    pub fn to_variant(&self) -> glib::Variant {
        unsafe { from_glib_full(ffi::panel_position_to_variant(self.to_glib_none().0)) }
    }

    #[doc(alias = "area")]
    pub fn connect_area_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_area_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::area\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_area_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "area-set")]
    pub fn connect_area_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_area_set_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::area-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_area_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column")]
    pub fn connect_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "column-set")]
    pub fn connect_column_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_set_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::column-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "depth")]
    pub fn connect_depth_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_depth_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::depth\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_depth_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "depth-set")]
    pub fn connect_depth_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_depth_set_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::depth-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_depth_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row")]
    pub fn connect_row_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "row-set")]
    pub fn connect_row_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_row_set_trampoline<F: Fn(&Position) + 'static>(
            this: *mut ffi::PanelPosition,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::row-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_row_set_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Position`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PositionBuilder {
    area: Option<Area>,
    area_set: Option<bool>,
    column: Option<u32>,
    column_set: Option<bool>,
    depth: Option<u32>,
    depth_set: Option<bool>,
    row: Option<u32>,
    row_set: Option<bool>,
}

impl PositionBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PositionBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Position`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Position {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref area) = self.area {
            properties.push(("area", area));
        }
        if let Some(ref area_set) = self.area_set {
            properties.push(("area-set", area_set));
        }
        if let Some(ref column) = self.column {
            properties.push(("column", column));
        }
        if let Some(ref column_set) = self.column_set {
            properties.push(("column-set", column_set));
        }
        if let Some(ref depth) = self.depth {
            properties.push(("depth", depth));
        }
        if let Some(ref depth_set) = self.depth_set {
            properties.push(("depth-set", depth_set));
        }
        if let Some(ref row) = self.row {
            properties.push(("row", row));
        }
        if let Some(ref row_set) = self.row_set {
            properties.push(("row-set", row_set));
        }
        glib::Object::new::<Position>(&properties)
    }

    pub fn area(mut self, area: Area) -> Self {
        self.area = Some(area);
        self
    }

    pub fn area_set(mut self, area_set: bool) -> Self {
        self.area_set = Some(area_set);
        self
    }

    pub fn column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    pub fn column_set(mut self, column_set: bool) -> Self {
        self.column_set = Some(column_set);
        self
    }

    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn depth_set(mut self, depth_set: bool) -> Self {
        self.depth_set = Some(depth_set);
        self
    }

    pub fn row(mut self, row: u32) -> Self {
        self.row = Some(row);
        self
    }

    pub fn row_set(mut self, row_set: bool) -> Self {
        self.row_set = Some(row_set);
        self
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Position")
    }
}
