use crate::SaveDelegate;
use futures_core::future::Future;
use glib::{signal::connect_raw, translate::*, Cast, IsA, SignalHandlerId};
use std::mem::transmute;

pub trait SaveDelegateExtManual {
    fn connect_save<F, R>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &gio::Task<bool>) -> R + 'static,
        R: Future<Output = Result<(), glib::Error>> + 'static;
}

impl<O: IsA<SaveDelegate>> SaveDelegateExtManual for O {
    fn connect_save<F, R>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &gio::Task<bool>) -> R + 'static,
        R: Future<Output = Result<(), glib::Error>> + 'static,
    {
        unsafe extern "C" fn save_trampoline<
            P: IsA<SaveDelegate>,
            F: Fn(&P, &gio::Task<bool>) -> R + 'static,
            R: Future<Output = Result<(), glib::Error>> + 'static,
        >(
            this: *mut ffi::PanelSaveDelegate,
            task: *mut gio::ffi::GTask,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            let task: gio::Task<bool> = from_glib_none(task);
            let delegate = SaveDelegate::from_glib_borrow(this);
            let fut = f(delegate.unsafe_cast_ref(), &task);
            task.context().spawn_local(async move {
                task.return_result(fut.await.map(|_| true));
            });
            true.into_glib()
        }
        unsafe {
            let f: Box<F> = Box::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"save\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    save_trampoline::<Self, F, R> as *const (),
                )),
                Box::into_raw(f),
            )
        }
    }
}
