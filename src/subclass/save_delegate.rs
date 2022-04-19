use crate::SaveDelegate;
use glib::subclass::prelude::*;
use glib::thread_guard::ThreadGuard;
use glib::translate::*;
use glib::Cast;
use std::{future::Future, pin::Pin};

pub trait SaveDelegateImpl: ObjectImpl {
    fn save_future(
        &self,
        delegate: &Self::Type,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        self.parent_save_future(delegate)
    }
}

pub trait SaveDelegateImplExt: ObjectSubclass {
    fn parent_save_future(
        &self,
        delegate: &Self::Type,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>>;
}

impl<T: SaveDelegateImpl> SaveDelegateImplExt for T {
    fn parent_save_future(
        &self,
        delegate: &Self::Type,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        unsafe {
            let type_data = T::type_data();
            let parent_class =
                type_data.as_ref().parent_class() as *mut ffi::PanelSaveDelegateClass;
            let save_async = (*parent_class)
                .save_async
                .expect("No parent class implementation for \"save_async\"");

            unsafe extern "C" fn parent_save_async_callback<T>(
                source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) where
                T: SaveDelegateImpl,
            {
                let type_data = T::type_data();
                let parent_class =
                    type_data.as_ref().parent_class() as *mut ffi::PanelSaveDelegateClass;
                let save_finish = (*parent_class)
                    .save_finish
                    .expect("No parent class implementation for \"save_finish\"");

                let ret: Box<ThreadGuard<gio::GioFutureResult<(), glib::Error>>> =
                    Box::from_raw(user_data as *mut _);
                let ret = ret.into_inner();

                let mut error = std::ptr::null_mut();
                save_finish(source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };

                ret.resolve(result);
            }

            Box::pin(gio::GioFuture::new(
                delegate,
                move |obj, cancellable, res| {
                    let user_data = Box::new(ThreadGuard::new(res));
                    save_async(
                        obj.unsafe_cast_ref::<SaveDelegate>().to_glib_none().0,
                        cancellable.to_glib_none().0,
                        Some(parent_save_async_callback::<T>),
                        Box::into_raw(user_data) as *mut _,
                    );
                },
            ))
        }
    }
}

unsafe impl<T: SaveDelegateImpl> IsSubclassable<T> for SaveDelegate {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.save_async = Some(save_delegate_save_async::<T>);
        klass.save_finish = Some(save_delegate_save_finish::<T>);
    }
}

unsafe extern "C" fn save_delegate_save_async<T: SaveDelegateImpl>(
    delegate: *mut ffi::PanelSaveDelegate,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(delegate as *mut T::Instance);
    let imp = instance.imp();
    let delegate: SaveDelegate = from_glib_none(delegate);
    let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable);

    let fut = imp.save_future(delegate.unsafe_cast_ref());
    glib::MainContext::default().spawn_local(async move {
        let res = fut.await;
        if let Some(callback) = callback {
            let t = gio::LocalTask::new(
                Some(delegate.upcast_ref::<glib::Object>()),
                cancellable.as_ref(),
                move |task: gio::LocalTask<bool>, source_object: Option<&glib::Object>| {
                    let result: *mut gio::ffi::GAsyncResult =
                        task.upcast_ref::<gio::AsyncResult>().to_glib_none().0;
                    let source_object: *mut glib::object::GObject = source_object.to_glib_none().0;
                    callback(source_object, result, user_data)
                },
            );
            t.return_result(res.map(|_| true));
        }
    });
}

unsafe extern "C" fn save_delegate_save_finish<T: SaveDelegateImpl>(
    delegate: *mut ffi::PanelSaveDelegate,
    res: *mut gio::ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let delegate = from_glib_borrow::<_, SaveDelegate>(delegate);
    let res: gio::AsyncResult = from_glib_none(res);
    let t = res.downcast::<gio::LocalTask<bool>>().unwrap();
    assert!(gio::LocalTask::<bool>::is_valid(
        &t,
        Some(delegate.as_ref())
    ));
    let ret = t.propagate();
    match ret {
        Ok(v) => {
            assert!(v);
            true.into_glib()
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_raw();
            }
            false.into_glib()
        }
    }
}
