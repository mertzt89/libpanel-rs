// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::DockPosition;
use crate::Frame;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "PanelDock")]
    pub struct Dock(Object<ffi::PanelDock, ffi::PanelDockClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_dock_get_type(),
    }
}

impl Dock {
    pub const NONE: Option<&'static Dock> = None;

    #[doc(alias = "panel_dock_new")]
    pub fn new() -> Dock {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::panel_dock_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Dock`] objects.
    ///
    /// This method returns an instance of [`DockBuilder`](crate::builders::DockBuilder) which can be used to create [`Dock`] objects.
    pub fn builder() -> DockBuilder {
        DockBuilder::default()
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Dock`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DockBuilder {
    bottom_height: Option<i32>,
    end_width: Option<i32>,
    reveal_bottom: Option<bool>,
    reveal_end: Option<bool>,
    reveal_start: Option<bool>,
    reveal_top: Option<bool>,
    start_width: Option<i32>,
    top_height: Option<i32>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    //cursor: /*Unknown type*/,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    //halign: /*Unknown type*/,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    //layout-manager: /*Unknown type*/,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    //overflow: /*Unknown type*/,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    //valign: /*Unknown type*/,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    //accessible-role: /*Unknown type*/,
}

impl DockBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`DockBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Dock`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Dock {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref bottom_height) = self.bottom_height {
            properties.push(("bottom-height", bottom_height));
        }
        if let Some(ref end_width) = self.end_width {
            properties.push(("end-width", end_width));
        }
        if let Some(ref reveal_bottom) = self.reveal_bottom {
            properties.push(("reveal-bottom", reveal_bottom));
        }
        if let Some(ref reveal_end) = self.reveal_end {
            properties.push(("reveal-end", reveal_end));
        }
        if let Some(ref reveal_start) = self.reveal_start {
            properties.push(("reveal-start", reveal_start));
        }
        if let Some(ref reveal_top) = self.reveal_top {
            properties.push(("reveal-top", reveal_top));
        }
        if let Some(ref start_width) = self.start_width {
            properties.push(("start-width", start_width));
        }
        if let Some(ref top_height) = self.top_height {
            properties.push(("top-height", top_height));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new::<Dock>(&properties).expect("Failed to create an instance of Dock")
    }

    pub fn bottom_height(mut self, bottom_height: i32) -> Self {
        self.bottom_height = Some(bottom_height);
        self
    }

    pub fn end_width(mut self, end_width: i32) -> Self {
        self.end_width = Some(end_width);
        self
    }

    pub fn reveal_bottom(mut self, reveal_bottom: bool) -> Self {
        self.reveal_bottom = Some(reveal_bottom);
        self
    }

    pub fn reveal_end(mut self, reveal_end: bool) -> Self {
        self.reveal_end = Some(reveal_end);
        self
    }

    pub fn reveal_start(mut self, reveal_start: bool) -> Self {
        self.reveal_start = Some(reveal_start);
        self
    }

    pub fn reveal_top(mut self, reveal_top: bool) -> Self {
        self.reveal_top = Some(reveal_top);
        self
    }

    pub fn start_width(mut self, start_width: i32) -> Self {
        self.start_width = Some(start_width);
        self
    }

    pub fn top_height(mut self, top_height: i32) -> Self {
        self.top_height = Some(top_height);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub trait DockExt: 'static {
    #[doc(alias = "panel_dock_foreach_frame")]
    fn foreach_frame<P: FnMut(&Frame)>(&self, callback: P);

    #[doc(alias = "panel_dock_get_can_reveal_bottom")]
    #[doc(alias = "get_can_reveal_bottom")]
    fn can_reveal_bottom(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_edge")]
    #[doc(alias = "get_can_reveal_edge")]
    fn can_reveal_edge(&self, edge: DockPosition) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_end")]
    #[doc(alias = "get_can_reveal_end")]
    fn can_reveal_end(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_start")]
    #[doc(alias = "get_can_reveal_start")]
    fn can_reveal_start(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_top")]
    #[doc(alias = "get_can_reveal_top")]
    fn can_reveal_top(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_bottom")]
    #[doc(alias = "get_reveal_bottom")]
    fn reveals_bottom(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_edge")]
    #[doc(alias = "get_reveal_edge")]
    fn reveals_edge(&self, edge: DockPosition) -> bool;

    #[doc(alias = "panel_dock_get_reveal_end")]
    #[doc(alias = "get_reveal_end")]
    fn reveals_end(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_start")]
    #[doc(alias = "get_reveal_start")]
    fn reveals_start(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_top")]
    #[doc(alias = "get_reveal_top")]
    fn reveals_top(&self) -> bool;

    #[doc(alias = "panel_dock_remove")]
    fn remove(&self, child: &impl IsA<gtk::Widget>);

    #[doc(alias = "panel_dock_set_bottom_height")]
    fn set_bottom_height(&self, height: i32);

    #[doc(alias = "panel_dock_set_end_width")]
    fn set_end_width(&self, width: i32);

    #[doc(alias = "panel_dock_set_reveal_bottom")]
    fn set_reveal_bottom(&self, reveal_bottom: bool);

    #[doc(alias = "panel_dock_set_reveal_edge")]
    fn set_reveal_edge(&self, position: DockPosition, reveal: bool);

    #[doc(alias = "panel_dock_set_reveal_end")]
    fn set_reveal_end(&self, reveal_end: bool);

    #[doc(alias = "panel_dock_set_reveal_start")]
    fn set_reveal_start(&self, reveal_start: bool);

    #[doc(alias = "panel_dock_set_reveal_top")]
    fn set_reveal_top(&self, reveal_top: bool);

    #[doc(alias = "panel_dock_set_start_width")]
    fn set_start_width(&self, width: i32);

    #[doc(alias = "panel_dock_set_top_height")]
    fn set_top_height(&self, height: i32);

    #[doc(alias = "bottom-height")]
    fn bottom_height(&self) -> i32;

    #[doc(alias = "end-width")]
    fn end_width(&self) -> i32;

    #[doc(alias = "start-width")]
    fn start_width(&self) -> i32;

    #[doc(alias = "top-height")]
    fn top_height(&self) -> i32;

    #[doc(alias = "panel-drag-begin")]
    fn connect_panel_drag_begin<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "panel-drag-end")]
    fn connect_panel_drag_end<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "bottom-height")]
    fn connect_bottom_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-bottom")]
    fn connect_can_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-end")]
    fn connect_can_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-start")]
    fn connect_can_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-top")]
    fn connect_can_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "end-width")]
    fn connect_end_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-bottom")]
    fn connect_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-end")]
    fn connect_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-start")]
    fn connect_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-top")]
    fn connect_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "start-width")]
    fn connect_start_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "top-height")]
    fn connect_top_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Dock>> DockExt for O {
    fn foreach_frame<P: FnMut(&Frame)>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Frame)>(
            frame: *mut ffi::PanelFrame,
            user_data: glib::ffi::gpointer,
        ) {
            let frame = from_glib_borrow(frame);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&frame);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::panel_dock_foreach_frame(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn can_reveal_bottom(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_bottom(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_edge(&self, edge: DockPosition) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_edge(
                self.as_ref().to_glib_none().0,
                edge.into_glib(),
            ))
        }
    }

    fn can_reveal_end(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_end(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_start(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_top(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_top(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_bottom(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_bottom(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_edge(&self, edge: DockPosition) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_edge(
                self.as_ref().to_glib_none().0,
                edge.into_glib(),
            ))
        }
    }

    fn reveals_end(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_end(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_start(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_top(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_top(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove(&self, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::panel_dock_remove(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_bottom_height(&self, height: i32) {
        unsafe {
            ffi::panel_dock_set_bottom_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_end_width(&self, width: i32) {
        unsafe {
            ffi::panel_dock_set_end_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_reveal_bottom(&self, reveal_bottom: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_bottom(
                self.as_ref().to_glib_none().0,
                reveal_bottom.into_glib(),
            );
        }
    }

    fn set_reveal_edge(&self, position: DockPosition, reveal: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_edge(
                self.as_ref().to_glib_none().0,
                position.into_glib(),
                reveal.into_glib(),
            );
        }
    }

    fn set_reveal_end(&self, reveal_end: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_end(self.as_ref().to_glib_none().0, reveal_end.into_glib());
        }
    }

    fn set_reveal_start(&self, reveal_start: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_start(
                self.as_ref().to_glib_none().0,
                reveal_start.into_glib(),
            );
        }
    }

    fn set_reveal_top(&self, reveal_top: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_top(self.as_ref().to_glib_none().0, reveal_top.into_glib());
        }
    }

    fn set_start_width(&self, width: i32) {
        unsafe {
            ffi::panel_dock_set_start_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_top_height(&self, height: i32) {
        unsafe {
            ffi::panel_dock_set_top_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn bottom_height(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "bottom-height")
    }

    fn end_width(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "end-width")
    }

    fn start_width(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "start-width")
    }

    fn top_height(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "top-height")
    }

    fn connect_panel_drag_begin<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn panel_drag_begin_trampoline<
            P: IsA<Dock>,
            F: Fn(&P, &Widget) + 'static,
        >(
            this: *mut ffi::PanelDock,
            panel: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Dock::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(panel),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"panel-drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    panel_drag_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_panel_drag_end<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn panel_drag_end_trampoline<
            P: IsA<Dock>,
            F: Fn(&P, &Widget) + 'static,
        >(
            this: *mut ffi::PanelDock,
            panel: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Dock::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(panel),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"panel-drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    panel_drag_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_bottom_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_height_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bottom-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bottom_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_bottom_trampoline<
            P: IsA<Dock>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-bottom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_bottom_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_end_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_start_trampoline<
            P: IsA<Dock>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_top_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-top\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_top_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_width_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::end-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_bottom_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-bottom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_bottom_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_end_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_start_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_top_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-top\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_top_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_start_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_width_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_top_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_top_height_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::top-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_top_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Dock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Dock")
    }
}
