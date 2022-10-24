// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::SaveDelegate;
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
use std::pin::Pin;
use std::ptr;

#[cfg(any(feature = "adw_v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
glib::wrapper! {
    #[doc(alias = "PanelSaveDialog")]
    pub struct SaveDialog(Object<ffi::PanelSaveDialog, ffi::PanelSaveDialogClass>) @extends adw::MessageDialog, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_save_dialog_get_type(),
    }
}

#[cfg(not(any(feature = "adw_v1_2", feature = "dox")))]
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
        unsafe { gtk::Widget::from_glib_full(ffi::panel_save_dialog_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`SaveDialog`] objects.
    ///
    /// This method returns an instance of [`SaveDialogBuilder`](crate::builders::SaveDialogBuilder) which can be used to create [`SaveDialog`] objects.
    pub fn builder() -> SaveDialogBuilder {
        SaveDialogBuilder::default()
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

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`SaveDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct SaveDialogBuilder {
    close_after_save: Option<bool>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    body: Option<String>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    body_use_markup: Option<bool>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    close_response: Option<String>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    default_response: Option<String>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    extra_child: Option<gtk::Widget>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    heading: Option<String>,
    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    heading_use_markup: Option<bool>,
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

impl SaveDialogBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`SaveDialogBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`SaveDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> SaveDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref close_after_save) = self.close_after_save {
            properties.push(("close-after-save", close_after_save));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref body) = self.body {
            properties.push(("body", body));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref body_use_markup) = self.body_use_markup {
            properties.push(("body-use-markup", body_use_markup));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref close_response) = self.close_response {
            properties.push(("close-response", close_response));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref default_response) = self.default_response {
            properties.push(("default-response", default_response));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref extra_child) = self.extra_child {
            properties.push(("extra-child", extra_child));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref heading) = self.heading {
            properties.push(("heading", heading));
        }
        #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
        if let Some(ref heading_use_markup) = self.heading_use_markup {
            properties.push(("heading-use-markup", heading_use_markup));
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
        glib::Object::new::<SaveDialog>(&properties)
    }

    pub fn close_after_save(mut self, close_after_save: bool) -> Self {
        self.close_after_save = Some(close_after_save);
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn body_use_markup(mut self, body_use_markup: bool) -> Self {
        self.body_use_markup = Some(body_use_markup);
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn close_response(mut self, close_response: &str) -> Self {
        self.close_response = Some(close_response.to_string());
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn default_response(mut self, default_response: &str) -> Self {
        self.default_response = Some(default_response.to_string());
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn extra_child(mut self, extra_child: &impl IsA<gtk::Widget>) -> Self {
        self.extra_child = Some(extra_child.clone().upcast());
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn heading(mut self, heading: &str) -> Self {
        self.heading = Some(heading.to_string());
        self
    }

    #[cfg(any(feature = "adw_v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "adw_v1_2")))]
    pub fn heading_use_markup(mut self, heading_use_markup: bool) -> Self {
        self.heading_use_markup = Some(heading_use_markup);
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

impl fmt::Display for SaveDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SaveDialog")
    }
}
