// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{Dock, Grid, Statusbar, Workspace};
#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
use crate::{Frame, Position, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PanelDocumentWorkspace")]
    pub struct DocumentWorkspace(Object<ffi::PanelDocumentWorkspace, ffi::PanelDocumentWorkspaceClass>) @extends Workspace, adw::ApplicationWindow, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gio::ActionGroup;

    match fn {
        type_ => || ffi::panel_document_workspace_get_type(),
    }
}

impl DocumentWorkspace {
    pub const NONE: Option<&'static DocumentWorkspace> = None;

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_new")]
    pub fn new() -> DocumentWorkspace {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::panel_document_workspace_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`DocumentWorkspace`] objects.
    ///
    /// This method returns an instance of [`DocumentWorkspaceBuilder`](crate::builders::DocumentWorkspaceBuilder) which can be used to create [`DocumentWorkspace`] objects.
    pub fn builder() -> DocumentWorkspaceBuilder {
        DocumentWorkspaceBuilder::new()
    }
}

#[cfg(feature = "v1_4")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
impl Default for DocumentWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`DocumentWorkspace`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct DocumentWorkspaceBuilder {
    builder: glib::object::ObjectBuilder<'static, DocumentWorkspace>,
}

impl DocumentWorkspaceBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("id", id.into()),
        }
    }

    pub fn content(self, content: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self.builder.property("content", content.clone().upcast()),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`DocumentWorkspace`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> DocumentWorkspace {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::DocumentWorkspace>> Sealed for T {}
}

pub trait DocumentWorkspaceExt: IsA<DocumentWorkspace> + sealed::Sealed + 'static {
    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_add_widget")]
    fn add_widget(&self, widget: &impl IsA<Widget>, position: Option<&Position>) -> bool {
        unsafe {
            from_glib(ffi::panel_document_workspace_add_widget(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                position.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_get_dock")]
    #[doc(alias = "get_dock")]
    fn dock(&self) -> Dock {
        unsafe {
            from_glib_none(ffi::panel_document_workspace_get_dock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_get_grid")]
    #[doc(alias = "get_grid")]
    fn grid(&self) -> Grid {
        unsafe {
            from_glib_none(ffi::panel_document_workspace_get_grid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_get_statusbar")]
    #[doc(alias = "get_statusbar")]
    fn statusbar(&self) -> Option<Statusbar> {
        unsafe {
            from_glib_none(ffi::panel_document_workspace_get_statusbar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_document_workspace_get_titlebar")]
    #[doc(alias = "get_titlebar")]
    fn titlebar(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::panel_document_workspace_get_titlebar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_document_workspace_set_titlebar")]
    fn set_titlebar(&self, titlebar: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::panel_document_workspace_set_titlebar(
                self.as_ref().to_glib_none().0,
                titlebar.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_property_dock(&self) -> Option<Dock> {
        ObjectExt::property(self.as_ref(), "dock")
    }

    fn get_property_grid(&self) -> Option<Grid> {
        ObjectExt::property(self.as_ref(), "grid")
    }

    fn get_property_statusbar(&self) -> Option<Statusbar> {
        ObjectExt::property(self.as_ref(), "statusbar")
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "add-widget")]
    fn connect_add_widget<F: Fn(&Self, &Widget, &Position) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn add_widget_trampoline<
            P: IsA<DocumentWorkspace>,
            F: Fn(&P, &Widget, &Position) -> bool + 'static,
        >(
            this: *mut ffi::PanelDocumentWorkspace,
            widget: *mut ffi::PanelWidget,
            position: *mut ffi::PanelPosition,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                DocumentWorkspace::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(widget),
                &from_glib_borrow(position),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"add-widget\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    add_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "create-frame")]
    fn connect_create_frame<F: Fn(&Self, &Position) -> Frame + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_frame_trampoline<
            P: IsA<DocumentWorkspace>,
            F: Fn(&P, &Position) -> Frame + 'static,
        >(
            this: *mut ffi::PanelDocumentWorkspace,
            position: *mut ffi::PanelPosition,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::PanelFrame {
            let f: &F = &*(f as *const F);
            f(
                DocumentWorkspace::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(position),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-frame\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    create_frame_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "dock")]
    fn connect_dock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dock_trampoline<
            P: IsA<DocumentWorkspace>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDocumentWorkspace,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DocumentWorkspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dock\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_dock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "grid")]
    fn connect_grid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_grid_trampoline<
            P: IsA<DocumentWorkspace>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDocumentWorkspace,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DocumentWorkspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::grid\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_grid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "statusbar")]
    fn connect_statusbar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_statusbar_trampoline<
            P: IsA<DocumentWorkspace>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDocumentWorkspace,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DocumentWorkspace::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::statusbar\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_statusbar_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<DocumentWorkspace>> DocumentWorkspaceExt for O {}
