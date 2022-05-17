// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Frame;
use crate::FrameHeader;
use glib::object::Cast;
use glib::object::IsA;
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
    #[doc(alias = "PanelFrameSwitcher")]
    pub struct FrameSwitcher(Object<ffi::PanelFrameSwitcher, ffi::PanelFrameSwitcherClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, FrameHeader;

    match fn {
        type_ => || ffi::panel_frame_switcher_get_type(),
    }
}

impl FrameSwitcher {
    #[doc(alias = "panel_frame_switcher_new")]
    pub fn new() -> FrameSwitcher {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::panel_frame_switcher_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FrameSwitcher`] objects.
    ///
    /// This method returns an instance of [`FrameSwitcherBuilder`](crate::builders::FrameSwitcherBuilder) which can be used to create [`FrameSwitcher`] objects.
    pub fn builder() -> FrameSwitcherBuilder {
        FrameSwitcherBuilder::default()
    }

    #[doc(alias = "panel_frame_switcher_get_background_rgba")]
    #[doc(alias = "get_background_rgba")]
    pub fn background_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            from_glib_none(ffi::panel_frame_switcher_get_background_rgba(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_frame_switcher_get_foreground_rgba")]
    #[doc(alias = "get_foreground_rgba")]
    pub fn foreground_rgba(&self) -> Option<gdk::RGBA> {
        unsafe {
            from_glib_none(ffi::panel_frame_switcher_get_foreground_rgba(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_frame_switcher_set_background_rgba")]
    pub fn set_background_rgba(&self, background_rgba: Option<&gdk::RGBA>) {
        unsafe {
            ffi::panel_frame_switcher_set_background_rgba(
                self.to_glib_none().0,
                background_rgba.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_frame_switcher_set_foreground_rgba")]
    pub fn set_foreground_rgba(&self, foreground_rgba: Option<&gdk::RGBA>) {
        unsafe {
            ffi::panel_frame_switcher_set_foreground_rgba(
                self.to_glib_none().0,
                foreground_rgba.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "background-rgba")]
    pub fn connect_background_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_background_rgba_trampoline<F: Fn(&FrameSwitcher) + 'static>(
            this: *mut ffi::PanelFrameSwitcher,
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
                b"notify::background-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_background_rgba_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "foreground-rgba")]
    pub fn connect_foreground_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_foreground_rgba_trampoline<F: Fn(&FrameSwitcher) + 'static>(
            this: *mut ffi::PanelFrameSwitcher,
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
                b"notify::foreground-rgba\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_foreground_rgba_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for FrameSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FrameSwitcher`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FrameSwitcherBuilder {
    background_rgba: Option<gdk::RGBA>,
    foreground_rgba: Option<gdk::RGBA>,
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
    //orientation: /*Unknown type*/,
    frame: Option<Frame>,
}

impl FrameSwitcherBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FrameSwitcherBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FrameSwitcher`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FrameSwitcher {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref background_rgba) = self.background_rgba {
            properties.push(("background-rgba", background_rgba));
        }
        if let Some(ref foreground_rgba) = self.foreground_rgba {
            properties.push(("foreground-rgba", foreground_rgba));
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
        if let Some(ref frame) = self.frame {
            properties.push(("frame", frame));
        }
        glib::Object::new::<FrameSwitcher>(&properties)
            .expect("Failed to create an instance of FrameSwitcher")
    }

    pub fn background_rgba(mut self, background_rgba: &gdk::RGBA) -> Self {
        self.background_rgba = Some(*background_rgba);
        self
    }

    pub fn foreground_rgba(mut self, foreground_rgba: &gdk::RGBA) -> Self {
        self.foreground_rgba = Some(*foreground_rgba);
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

    pub fn frame(mut self, frame: &impl IsA<Frame>) -> Self {
        self.frame = Some(frame.clone().upcast());
        self
    }
}

impl fmt::Display for FrameSwitcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FrameSwitcher")
    }
}
