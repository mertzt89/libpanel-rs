// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::SaveDelegate;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

#[cfg(any(feature = "adw_v1_2", docsrs))]
#[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
glib::wrapper! {
    #[doc(alias = "PanelSaveDialog")]
    pub struct SaveDialog(Object<ffi::PanelSaveDialog, ffi::PanelSaveDialogClass>) @extends adw::MessageDialog, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_save_dialog_get_type(),
    }
}

#[cfg(not(any(feature = "adw_v1_2", docsrs)))]
glib::wrapper! {
    #[doc(alias = "PanelSaveDialog")]
    pub struct SaveDialog(Object<ffi::PanelSaveDialog, ffi::PanelSaveDialogClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_save_dialog_get_type(),
    }
}

impl SaveDialog {
    #[doc(alias = "panel_save_dialog_new")]
    pub fn new() -> SaveDialog {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::panel_save_dialog_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SaveDialog`] objects.
    ///
    /// This method returns an instance of [`SaveDialogBuilder`](crate::builders::SaveDialogBuilder) which can be used to create [`SaveDialog`] objects.
    pub fn builder() -> SaveDialogBuilder {
        SaveDialogBuilder::new()
    }

    #[doc(alias = "panel_save_dialog_add_delegate")]
    pub fn add_delegate(&self, delegate: &impl IsA<SaveDelegate>) {
        unsafe {
            ffi::panel_save_dialog_add_delegate(
                self.to_glib_none().0,
                delegate.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_save_dialog_get_close_after_save")]
    #[doc(alias = "get_close_after_save")]
    pub fn closes_after_save(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_save_dialog_get_close_after_save(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "panel_save_dialog_run_async")]
    pub fn run_async<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn run_async_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::panel_save_dialog_run_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = run_async_trampoline::<P>;
        unsafe {
            ffi::panel_save_dialog_run_async(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn run_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.run_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "panel_save_dialog_set_close_after_save")]
    pub fn set_close_after_save(&self, close_after_save: bool) {
        unsafe {
            ffi::panel_save_dialog_set_close_after_save(
                self.to_glib_none().0,
                close_after_save.into_glib(),
            );
        }
    }

    #[doc(alias = "close-after-save")]
    pub fn connect_close_after_save_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_close_after_save_trampoline<F: Fn(&SaveDialog) + 'static>(
            this: *mut ffi::PanelSaveDialog,
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
                b"notify::close-after-save\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_close_after_save_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SaveDialog {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SaveDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SaveDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, SaveDialog>,
}

impl SaveDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn close_after_save(self, close_after_save: bool) -> Self {
        Self {
            builder: self.builder.property("close-after-save", close_after_save),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn body(self, body: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("body", body.into()),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn body_use_markup(self, body_use_markup: bool) -> Self {
        Self {
            builder: self.builder.property("body-use-markup", body_use_markup),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn close_response(self, close_response: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("close-response", close_response.into()),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn default_response(self, default_response: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("default-response", default_response.into()),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn extra_child(self, extra_child: &impl IsA<gtk::Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("extra-child", extra_child.clone().upcast()),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn heading(self, heading: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("heading", heading.into()),
        }
    }

    #[cfg(any(feature = "adw_v1_2", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "adw_v1_2")))]
    pub fn heading_use_markup(self, heading_use_markup: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("heading-use-markup", heading_use_markup),
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
    /// Build the [`SaveDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SaveDialog {
        self.builder.build()
    }
}

impl fmt::Display for SaveDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SaveDialog")
    }
}
