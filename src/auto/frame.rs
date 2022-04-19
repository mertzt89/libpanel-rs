// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::FrameHeader;
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
    #[doc(alias = "PanelFrame")]
    pub struct Frame(Object<ffi::PanelFrame, ffi::PanelFrameClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;

    match fn {
        type_ => || ffi::panel_frame_get_type(),
    }
}

impl Frame {
    pub const NONE: Option<&'static Frame> = None;

    #[doc(alias = "panel_frame_new")]
    pub fn new() -> Frame {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::panel_frame_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Frame`] objects.
    ///
    /// This method returns an instance of [`FrameBuilder`](crate::builders::FrameBuilder) which can be used to create [`Frame`] objects.
    pub fn builder() -> FrameBuilder {
        FrameBuilder::default()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Frame`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FrameBuilder {
    placeholder: Option<gtk::Widget>,
    visible_child: Option<Widget>,
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
}

impl FrameBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FrameBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Frame`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Frame {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref placeholder) = self.placeholder {
            properties.push(("placeholder", placeholder));
        }
        if let Some(ref visible_child) = self.visible_child {
            properties.push(("visible-child", visible_child));
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
        glib::Object::new::<Frame>(&properties).expect("Failed to create an instance of Frame")
    }

    pub fn placeholder(mut self, placeholder: &impl IsA<gtk::Widget>) -> Self {
        self.placeholder = Some(placeholder.clone().upcast());
        self
    }

    pub fn visible_child(mut self, visible_child: &impl IsA<Widget>) -> Self {
        self.visible_child = Some(visible_child.clone().upcast());
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

pub trait FrameExt: 'static {
    #[doc(alias = "panel_frame_add")]
    fn add(&self, panel: &impl IsA<Widget>);

    #[doc(alias = "panel_frame_add_before")]
    fn add_before(&self, panel: &impl IsA<Widget>, sibling: &impl IsA<Widget>);

    #[doc(alias = "panel_frame_get_empty")]
    #[doc(alias = "get_empty")]
    fn is_empty(&self) -> bool;

    #[doc(alias = "panel_frame_get_header")]
    #[doc(alias = "get_header")]
    fn header(&self) -> Option<FrameHeader>;

    #[doc(alias = "panel_frame_get_n_pages")]
    #[doc(alias = "get_n_pages")]
    fn n_pages(&self) -> u32;

    #[doc(alias = "panel_frame_get_page")]
    #[doc(alias = "get_page")]
    fn page(&self, n: u32) -> Option<Widget>;

    #[doc(alias = "panel_frame_get_pages")]
    #[doc(alias = "get_pages")]
    fn pages(&self) -> gtk::SelectionModel;

    #[doc(alias = "panel_frame_get_placeholder")]
    #[doc(alias = "get_placeholder")]
    fn placeholder(&self) -> Option<gtk::Widget>;

    #[doc(alias = "panel_frame_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    fn visible_child(&self) -> Option<Widget>;

    #[doc(alias = "panel_frame_remove")]
    fn remove(&self, panel: &impl IsA<Widget>);

    #[doc(alias = "panel_frame_set_header")]
    fn set_header(&self, header: Option<&impl IsA<FrameHeader>>);

    #[doc(alias = "panel_frame_set_placeholder")]
    fn set_placeholder(&self, placeholder: Option<&impl IsA<gtk::Widget>>);

    #[doc(alias = "panel_frame_set_visible_child")]
    fn set_visible_child(&self, widget: &impl IsA<Widget>);

    #[doc(alias = "empty")]
    fn connect_empty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "placeholder")]
    fn connect_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-child")]
    fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Frame>> FrameExt for O {
    fn add(&self, panel: &impl IsA<Widget>) {
        unsafe {
            ffi::panel_frame_add(
                self.as_ref().to_glib_none().0,
                panel.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_before(&self, panel: &impl IsA<Widget>, sibling: &impl IsA<Widget>) {
        unsafe {
            ffi::panel_frame_add_before(
                self.as_ref().to_glib_none().0,
                panel.as_ref().to_glib_none().0,
                sibling.as_ref().to_glib_none().0,
            );
        }
    }

    fn is_empty(&self) -> bool {
        unsafe { from_glib(ffi::panel_frame_get_empty(self.as_ref().to_glib_none().0)) }
    }

    fn header(&self) -> Option<FrameHeader> {
        unsafe { from_glib_none(ffi::panel_frame_get_header(self.as_ref().to_glib_none().0)) }
    }

    fn n_pages(&self) -> u32 {
        unsafe { ffi::panel_frame_get_n_pages(self.as_ref().to_glib_none().0) }
    }

    fn page(&self, n: u32) -> Option<Widget> {
        unsafe { from_glib_none(ffi::panel_frame_get_page(self.as_ref().to_glib_none().0, n)) }
    }

    fn pages(&self) -> gtk::SelectionModel {
        unsafe { from_glib_full(ffi::panel_frame_get_pages(self.as_ref().to_glib_none().0)) }
    }

    fn placeholder(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::panel_frame_get_placeholder(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::panel_frame_get_visible_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove(&self, panel: &impl IsA<Widget>) {
        unsafe {
            ffi::panel_frame_remove(
                self.as_ref().to_glib_none().0,
                panel.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_header(&self, header: Option<&impl IsA<FrameHeader>>) {
        unsafe {
            ffi::panel_frame_set_header(
                self.as_ref().to_glib_none().0,
                header.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_placeholder(&self, placeholder: Option<&impl IsA<gtk::Widget>>) {
        unsafe {
            ffi::panel_frame_set_placeholder(
                self.as_ref().to_glib_none().0,
                placeholder.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_visible_child(&self, widget: &impl IsA<Widget>) {
        unsafe {
            ffi::panel_frame_set_visible_child(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn connect_empty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_empty_trampoline<P: IsA<Frame>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelFrame,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Frame::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::empty\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_empty_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_placeholder_trampoline<P: IsA<Frame>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelFrame,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Frame::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::placeholder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_placeholder_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_child_trampoline<P: IsA<Frame>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelFrame,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Frame::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible-child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Frame")
    }
}
