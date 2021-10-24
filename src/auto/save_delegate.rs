// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "PanelSaveDelegate")]
    pub struct SaveDelegate(Object<ffi::PanelSaveDelegate, ffi::PanelSaveDelegateClass>);

    match fn {
        type_ => || ffi::panel_save_delegate_get_type(),
    }
}

impl SaveDelegate {
    #[doc(alias = "panel_save_delegate_new")]
    pub fn new() -> SaveDelegate {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::panel_save_delegate_new()) }
    }
}

impl Default for SaveDelegate {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SAVE_DELEGATE: Option<&SaveDelegate> = None;

pub trait SaveDelegateExt: 'static {
    #[doc(alias = "panel_save_delegate_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<gio::Icon>;

    #[doc(alias = "panel_save_delegate_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    fn icon_name(&self) -> Option<glib::GString>;

    #[doc(alias = "panel_save_delegate_get_progress")]
    #[doc(alias = "get_progress")]
    fn progress(&self) -> f64;

    #[doc(alias = "panel_save_delegate_get_subtitle")]
    #[doc(alias = "get_subtitle")]
    fn subtitle(&self) -> Option<glib::GString>;

    #[doc(alias = "panel_save_delegate_get_title")]
    #[doc(alias = "get_title")]
    fn title(&self) -> Option<glib::GString>;

    #[doc(alias = "panel_save_delegate_save_async")]
    fn save_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );

    fn save_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "panel_save_delegate_set_icon")]
    fn set_icon(&self, icon: Option<&impl IsA<gio::Icon>>);

    #[doc(alias = "panel_save_delegate_set_icon_name")]
    fn set_icon_name(&self, icon: Option<&str>);

    #[doc(alias = "panel_save_delegate_set_progress")]
    fn set_progress(&self, progress: f64);

    #[doc(alias = "panel_save_delegate_set_subtitle")]
    fn set_subtitle(&self, subtitle: Option<&str>);

    #[doc(alias = "panel_save_delegate_set_title")]
    fn set_title(&self, title: Option<&str>);

    #[doc(alias = "save")]
    fn connect_save<F: Fn(&Self, &gio::Task) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-name")]
    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "progress")]
    fn connect_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "subtitle")]
    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "title")]
    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SaveDelegate>> SaveDelegateExt for O {
    fn icon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_none(ffi::panel_save_delegate_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::panel_save_delegate_get_icon_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn progress(&self) -> f64 {
        unsafe { ffi::panel_save_delegate_get_progress(self.as_ref().to_glib_none().0) }
    }

    fn subtitle(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::panel_save_delegate_get_subtitle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn title(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::panel_save_delegate_get_title(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn save_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn save_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::panel_save_delegate_save_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = save_async_trampoline::<P>;
        unsafe {
            ffi::panel_save_delegate_save_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn save_async_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save_async(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    fn set_icon(&self, icon: Option<&impl IsA<gio::Icon>>) {
        unsafe {
            ffi::panel_save_delegate_set_icon(
                self.as_ref().to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_icon_name(&self, icon: Option<&str>) {
        unsafe {
            ffi::panel_save_delegate_set_icon_name(
                self.as_ref().to_glib_none().0,
                icon.to_glib_none().0,
            );
        }
    }

    fn set_progress(&self, progress: f64) {
        unsafe {
            ffi::panel_save_delegate_set_progress(self.as_ref().to_glib_none().0, progress);
        }
    }

    fn set_subtitle(&self, subtitle: Option<&str>) {
        unsafe {
            ffi::panel_save_delegate_set_subtitle(
                self.as_ref().to_glib_none().0,
                subtitle.to_glib_none().0,
            );
        }
    }

    fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::panel_save_delegate_set_title(
                self.as_ref().to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    fn connect_save<F: Fn(&Self, &gio::Task) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn save_trampoline<
            P: IsA<SaveDelegate>,
            F: Fn(&P, &gio::Task) -> bool + 'static,
        >(
            this: *mut ffi::PanelSaveDelegate,
            task: *mut gio::ffi::GTask,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                SaveDelegate::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(task),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"save\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    save_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P: IsA<SaveDelegate>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelSaveDelegate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SaveDelegate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<
            P: IsA<SaveDelegate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelSaveDelegate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SaveDelegate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_progress_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_progress_trampoline<
            P: IsA<SaveDelegate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelSaveDelegate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SaveDelegate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::progress\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_progress_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_subtitle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subtitle_trampoline<
            P: IsA<SaveDelegate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelSaveDelegate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SaveDelegate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subtitle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subtitle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<P: IsA<SaveDelegate>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelSaveDelegate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SaveDelegate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SaveDelegate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SaveDelegate")
    }
}
