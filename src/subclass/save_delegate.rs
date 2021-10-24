use crate::SaveDelegate;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait SaveDelegateImpl: ObjectImpl {
    fn save_async<Q: FnOnce(&gio::AsyncResult, Option<&glib::Object>) + 'static>(
        &self,
        delegate: &Self::Type,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        SaveDelegateImplExt::parent_save_async(self, delegate, cancellable, callback);
    }

    fn save_finish(
        &self,
        delegate: &Self::Type,
        result: &gio::AsyncResult,
    ) -> Result<bool, glib::Error> {
        SaveDelegateImplExt::parent_save_finish(self, delegate, result)
    }

    fn save(&self, delegate: &Self::Type, task: &gio::Task) -> bool {
        SaveDelegateImplExt::parent_save(self, delegate, task)
    }
}

pub trait SaveDelegateImplExt: ObjectSubclass {
    fn parent_save_async<Q: FnOnce(&gio::AsyncResult, Option<&glib::Object>) + 'static>(
        &self,
        delegate: &Self::Type,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    );
    fn parent_save_finish(
        &self,
        delegate: &Self::Type,
        result: &gio::AsyncResult,
    ) -> Result<bool, glib::Error>;
    fn parent_save(&self, delegate: &Self::Type, task: &gio::Task) -> bool;
}

impl<T: SaveDelegateImpl> SaveDelegateImplExt for T {
    fn parent_save_async<Q: FnOnce(&gio::AsyncResult, Option<&glib::Object>) + 'static>(
        &self,
        delegate: &Self::Type,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelSaveDelegateClass;
            if let Some(f) = (*parent_class).save_async {
                let user_data: Box<Q> = Box::new(callback);
                unsafe extern "C" fn trampoline<
                    P: FnOnce(&gio::AsyncResult, Option<&glib::Object>) + 'static,
                >(
                    source_object: *mut glib::gobject_ffi::GObject,
                    res: *mut gio::ffi::GAsyncResult,
                    user_data: glib::ffi::gpointer,
                ) {
                    let obj: Option<Borrowed<glib::Object>> = if source_object.is_null() {
                        None
                    } else {
                        Some(from_glib_borrow(source_object))
                    };
                    let result: Borrowed<gio::AsyncResult> = from_glib_borrow(res);
                    let callback: Box<P> = Box::from_raw(user_data as *mut _);
                    let obj = obj.as_ref().map(|o| o.unsafe_cast_ref());
                    callback(result.unsafe_cast_ref(), obj);
                }

                f(
                    delegate.unsafe_cast_ref::<SaveDelegate>().to_glib_none().0,
                    cancellable
                        .map(|p| p.unsafe_cast_ref::<gio::Cancellable>())
                        .to_glib_none()
                        .0,
                    Some(trampoline::<Q>),
                    Box::into_raw(user_data) as *mut _,
                );
            }
        }
    }
    fn parent_save_finish(
        &self,
        delegate: &Self::Type,
        result: &gio::AsyncResult,
    ) -> Result<bool, glib::Error> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelSaveDelegateClass;
            if let Some(f) = (*parent_class).save_finish {
                let mut error = std::ptr::null_mut();

                let res = f(
                    delegate.unsafe_cast_ref::<SaveDelegate>().to_glib_none().0,
                    result
                        .unsafe_cast_ref::<gio::AsyncResult>()
                        .to_glib_none()
                        .0,
                    &mut error,
                ) != 0;
                if error.is_null() {
                    return Ok(res);
                } else {
                    return Err(from_glib_full(error));
                }
            }
            Ok(true)
        }
    }
    fn parent_save(&self, delegate: &Self::Type, task: &gio::Task) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelSaveDelegateClass;
            if let Some(f) = (*parent_class).save {
                return f(
                    delegate.unsafe_cast_ref::<SaveDelegate>().to_glib_none().0,
                    task.unsafe_cast_ref::<gio::Task>().to_glib_none().0,
                ) != 0;
            }
            true
        }
    }
}

unsafe impl<T: SaveDelegateImpl> IsSubclassable<T> for SaveDelegate {
    fn class_init(class: &mut glib::Class<Self>) {
        <glib::Object as IsSubclassable<T>>::class_init(class);
        let klass = class.as_mut();
        klass.save_async = Some(save_delegate_save_async::<T>);
        klass.save_finish = Some(save_delegate_save_finish::<T>);
        klass.save = Some(save_delegate_save::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <glib::Object as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn save_delegate_save_async<T: SaveDelegateImpl>(
    ptr: *mut ffi::PanelSaveDelegate,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<SaveDelegate> = from_glib_borrow(ptr);
    let cancellable: Option<Borrowed<gio::Cancellable>> = if cancellable.is_null() {
        None
    } else {
        Some(from_glib_borrow(cancellable))
    };

    SaveDelegateImpl::save_async(
        imp,
        wrap.unsafe_cast_ref(),
        cancellable.as_ref().map(|c| c.unsafe_cast_ref()),
        move |result, source_object| {
            if let Some(callback) = callback {
                callback(
                    source_object.to_glib_none().0,
                    result.to_glib_none().0,
                    user_data,
                );
            }
        },
    );
}

unsafe extern "C" fn save_delegate_save_finish<T: SaveDelegateImpl>(
    ptr: *mut ffi::PanelSaveDelegate,
    result: *mut gio::ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<SaveDelegate> = from_glib_borrow(ptr);
    let result: Borrowed<gio::AsyncResult> = from_glib_borrow(result);

    let res = SaveDelegateImpl::save_finish(imp, wrap.unsafe_cast_ref(), result.unsafe_cast_ref());
    match res {
        Ok(res) => res as glib::ffi::gboolean,
        Err(err) => {
            glib::ffi::g_propagate_error(error, err.to_glib_full() as *mut glib::ffi::GError);
            false as glib::ffi::gboolean
        }
    }
}

unsafe extern "C" fn save_delegate_save<T: SaveDelegateImpl>(
    ptr: *mut ffi::PanelSaveDelegate,
    task: *mut gio::ffi::GTask,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<SaveDelegate> = from_glib_borrow(ptr);
    let task: Borrowed<gio::Task> = from_glib_borrow(task);

    SaveDelegateImpl::save(imp, wrap.unsafe_cast_ref(), task.unsafe_cast_ref())
        as glib::ffi::gboolean
}
