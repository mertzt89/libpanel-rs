// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::{ffi, Workspace};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "PanelWorkbench")]
    pub struct Workbench(Object<ffi::PanelWorkbench, ffi::PanelWorkbenchClass>);

    match fn {
        type_ => || ffi::panel_workbench_get_type(),
    }
}

impl Workbench {
    pub const NONE: Option<&'static Workbench> = None;

    #[doc(alias = "panel_workbench_new")]
    pub fn new() -> Workbench {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::panel_workbench_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Workbench`] objects.
    ///
    /// This method returns an instance of [`WorkbenchBuilder`](crate::builders::WorkbenchBuilder) which can be used to create [`Workbench`] objects.
    pub fn builder() -> WorkbenchBuilder {
        WorkbenchBuilder::new()
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_workbench_find_from_widget")]
    pub fn find_from_widget(widget: &impl IsA<gtk::Widget>) -> Option<Workbench> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::panel_workbench_find_from_widget(
                widget.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl Default for Workbench {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Workbench`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct WorkbenchBuilder {
    builder: glib::object::ObjectBuilder<'static, Workbench>,
}

impl WorkbenchBuilder {
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

    // rustdoc-stripper-ignore-next
    /// Build the [`Workbench`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Workbench {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Workbench>> Sealed for T {}
}

pub trait WorkbenchExt: IsA<Workbench> + sealed::Sealed + 'static {
    #[doc(alias = "panel_workbench_action_set_enabled")]
    fn action_set_enabled(&self, action_name: &str, enabled: bool) {
        unsafe {
            ffi::panel_workbench_action_set_enabled(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "panel_workbench_activate")]
    fn activate(&self) {
        unsafe {
            ffi::panel_workbench_activate(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "panel_workbench_add_workspace")]
    fn add_workspace(&self, workspace: &impl IsA<Workspace>) {
        unsafe {
            ffi::panel_workbench_add_workspace(
                self.as_ref().to_glib_none().0,
                workspace.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_workbench_find_workspace_typed")]
    fn find_workspace_typed(&self, workspace_type: glib::types::Type) -> Option<Workspace> {
        unsafe {
            from_glib_none(ffi::panel_workbench_find_workspace_typed(
                self.as_ref().to_glib_none().0,
                workspace_type.into_glib(),
            ))
        }
    }

    #[doc(alias = "panel_workbench_focus_workspace")]
    fn focus_workspace(&self, workspace: &impl IsA<Workspace>) {
        unsafe {
            ffi::panel_workbench_focus_workspace(
                self.as_ref().to_glib_none().0,
                workspace.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_workbench_foreach_workspace")]
    fn foreach_workspace<P: FnMut(&Workspace)>(&self, foreach_func: P) {
        let foreach_func_data: P = foreach_func;
        unsafe extern "C" fn foreach_func_func<P: FnMut(&Workspace)>(
            workspace: *mut ffi::PanelWorkspace,
            user_data: glib::ffi::gpointer,
        ) {
            let workspace = from_glib_borrow(workspace);
            let callback = user_data as *mut P;
            (*callback)(&workspace)
        }
        let foreach_func = Some(foreach_func_func::<P> as _);
        let super_callback0: &P = &foreach_func_data;
        unsafe {
            ffi::panel_workbench_foreach_workspace(
                self.as_ref().to_glib_none().0,
                foreach_func,
                super_callback0 as *const _ as *mut _,
            );
        }
    }

    #[doc(alias = "panel_workbench_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::panel_workbench_get_id(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "panel_workbench_remove_workspace")]
    fn remove_workspace(&self, workspace: &impl IsA<Workspace>) {
        unsafe {
            ffi::panel_workbench_remove_workspace(
                self.as_ref().to_glib_none().0,
                workspace.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "panel_workbench_set_id")]
    #[doc(alias = "id")]
    fn set_id(&self, id: &str) {
        unsafe {
            ffi::panel_workbench_set_id(self.as_ref().to_glib_none().0, id.to_glib_none().0);
        }
    }

    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<Workbench>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelWorkbench,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Workbench::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v1_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<Workbench>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelWorkbench,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Workbench::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Workbench>> WorkbenchExt for O {}
