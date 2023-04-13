// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{FrameHeader, Position, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

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
        FrameBuilder::new()
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Frame`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FrameBuilder {
    builder: glib::object::ObjectBuilder<'static, Frame>,
}

impl FrameBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn placeholder(self, placeholder: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder", placeholder.clone().upcast()),
        }
    }

    pub fn visible_child(self, visible_child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("visible-child", visible_child.clone().upcast()),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<gtk::LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: gtk::Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: gtk::Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: gtk::AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn orientation(self, orientation: gtk::Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Frame`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Frame {
        self.builder.build()
    }
}

pub trait PanelFrameExt: 'static {
    #[doc(alias = "panel_frame_add")]
    fn add(&self, panel: &impl IsA<Widget>);

    #[doc(alias = "panel_frame_add_before")]
    fn add_before(&self, panel: &impl IsA<Widget>, sibling: &impl IsA<Widget>);

    #[doc(alias = "panel_frame_get_closeable")]
    #[doc(alias = "get_closeable")]
    fn is_closeable(&self) -> bool;

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

    #[doc(alias = "panel_frame_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> Position;

    #[doc(alias = "panel_frame_get_requested_size")]
    #[doc(alias = "get_requested_size")]
    fn requested_size(&self) -> i32;

    #[doc(alias = "panel_frame_get_visible_child")]
    #[doc(alias = "get_visible_child")]
    fn visible_child(&self) -> Option<Widget>;

    #[doc(alias = "panel_frame_remove")]
    fn remove(&self, panel: &impl IsA<Widget>);

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "panel_frame_set_child_pinned")]
    fn set_child_pinned(&self, child: &impl IsA<Widget>, pinned: bool);

    #[doc(alias = "panel_frame_set_header")]
    fn set_header(&self, header: Option<&impl IsA<FrameHeader>>);

    #[doc(alias = "panel_frame_set_placeholder")]
    fn set_placeholder(&self, placeholder: Option<&impl IsA<gtk::Widget>>);

    #[doc(alias = "panel_frame_set_requested_size")]
    fn set_requested_size(&self, requested_size: i32);

    #[doc(alias = "panel_frame_set_visible_child")]
    fn set_visible_child(&self, widget: &impl IsA<Widget>);

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "adopt-widget")]
    fn connect_adopt_widget<F: Fn(&Self, &Widget) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    #[doc(alias = "page-closed")]
    fn connect_page_closed<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "closeable")]
    fn connect_closeable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "empty")]
    fn connect_empty_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "placeholder")]
    fn connect_placeholder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible-child")]
    fn connect_visible_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Frame>> PanelFrameExt for O {
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

    fn is_closeable(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_frame_get_closeable(
                self.as_ref().to_glib_none().0,
            ))
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

    fn position(&self) -> Position {
        unsafe {
            from_glib_full(ffi::panel_frame_get_position(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn requested_size(&self) -> i32 {
        unsafe { ffi::panel_frame_get_requested_size(self.as_ref().to_glib_none().0) }
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

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    fn set_child_pinned(&self, child: &impl IsA<Widget>, pinned: bool) {
        unsafe {
            ffi::panel_frame_set_child_pinned(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                pinned.into_glib(),
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

    fn set_requested_size(&self, requested_size: i32) {
        unsafe {
            ffi::panel_frame_set_requested_size(self.as_ref().to_glib_none().0, requested_size);
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

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    fn connect_adopt_widget<F: Fn(&Self, &Widget) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn adopt_widget_trampoline<
            P: IsA<Frame>,
            F: Fn(&P, &Widget) -> bool + 'static,
        >(
            this: *mut ffi::PanelFrame,
            widget: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                Frame::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(widget),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"adopt-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    adopt_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_2")))]
    fn connect_page_closed<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn page_closed_trampoline<P: IsA<Frame>, F: Fn(&P, &Widget) + 'static>(
            this: *mut ffi::PanelFrame,
            widget: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Frame::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(widget),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"page-closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    page_closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_closeable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_closeable_trampoline<P: IsA<Frame>, F: Fn(&P) + 'static>(
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
                b"notify::closeable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_closeable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
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
