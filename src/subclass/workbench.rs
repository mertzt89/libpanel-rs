use crate::{prelude::*, Workbench};
use glib::thread_guard::ThreadGuard;
use glib::translate::*;
use glib::GString;
use glib::Variant;
use gtk::subclass::prelude::*;
use std::collections::HashMap;
use std::{future::Future, pin::Pin};

#[derive(Debug, Default)]
struct Internal {
    actions: HashMap<String, glib::ffi::gpointer>,
}
unsafe impl Sync for Internal {}
unsafe impl Send for Internal {}

pub trait WorkbenchImpl: WindowGroupImpl {
    fn activate(&self) {
        WorkbenchImplExt::parent_activate(self)
    }
    fn unload_future(&self) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        self.parent_unload_future()
    }
}

pub trait WorkbenchImplExt: ObjectSubclass {
    fn parent_activate(&self);
    fn parent_unload_future(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>>;
}

impl<T: WorkbenchImpl> WorkbenchImplExt for T {
    fn parent_activate(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelWorkbenchClass;
            if let Some(f) = (*parent_class).activate {
                f(self.obj().unsafe_cast_ref::<Workbench>().to_glib_none().0);
            }
        }
    }
    fn parent_unload_future(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        unsafe {
            let type_data = T::type_data();
            let parent_class = type_data.as_ref().parent_class() as *mut ffi::PanelWorkbenchClass;
            let unload_async = (*parent_class)
                .unload_async
                .expect("No parent class implementation for \"unload_async\"");

            unsafe extern "C" fn parent_unload_async_callback<T>(
                source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) where
                T: WorkbenchImpl,
            {
                let type_data = T::type_data();
                let parent_class =
                    type_data.as_ref().parent_class() as *mut ffi::PanelWorkbenchClass;
                let unload_finish = (*parent_class)
                    .unload_finish
                    .expect("No parent class implementation for \"unload_finish\"");

                let ret: Box<ThreadGuard<gio::GioFutureResult<Result<(), glib::Error>>>> =
                    Box::from_raw(user_data as *mut _);
                let ret = ret.into_inner();

                let mut error = std::ptr::null_mut();
                unload_finish(source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };

                ret.resolve(result);
            }

            Box::pin(gio::GioFuture::new(
                &*self.obj(),
                move |obj, cancellable, res| {
                    let user_data = Box::new(ThreadGuard::new(res));
                    unload_async(
                        obj.unsafe_cast_ref::<Workbench>().to_glib_none().0,
                        cancellable.to_glib_none().0,
                        Some(parent_unload_async_callback::<T>),
                        Box::into_raw(user_data) as *mut _,
                    );
                },
            ))
        }
    }
}

unsafe impl<T: WorkbenchImpl> IsSubclassable<T> for Workbench {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        unsafe {
            let mut data = T::type_data();
            let data = data.as_mut();
            // Used to store actions for `install_action` and `rust_builder_scope`
            data.set_class_data(<T as ObjectSubclassType>::type_(), Internal::default());
        }
        let klass = class.as_mut();
        klass.activate = Some(panel_workbench_activate::<T>);
        klass.unload_async = Some(panel_workbench_unload_async::<T>);
        klass.unload_finish = Some(panel_workbench_unload_finish);
    }
}

unsafe extern "C" fn panel_workbench_activate<T: WorkbenchImpl>(ptr: *mut ffi::PanelWorkbench) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    WorkbenchImpl::activate(imp);
}

unsafe extern "C" fn panel_workbench_unload_async<T: WorkbenchImpl>(
    workbench: *mut ffi::PanelWorkbench,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(workbench as *mut T::Instance);
    let imp = instance.imp();
    let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable);
    let workbench: Option<Workbench> = callback.map(|_| from_glib_none(workbench));

    let fut = imp.unload_future();
    glib::MainContext::default().spawn_local(async move {
        let res = fut.await;
        if let Some(callback) = callback {
            let t = gio::LocalTask::new(
                Some(workbench.unwrap_unchecked().upcast_ref::<glib::Object>()),
                cancellable.as_ref(),
                move |task: gio::LocalTask<bool>, source_object: Option<&glib::Object>| {
                    let result: *mut gio::ffi::GAsyncResult =
                        task.upcast_ref::<gio::AsyncResult>().to_glib_none().0;
                    let source_object: *mut glib::gobject_ffi::GObject =
                        source_object.to_glib_none().0;
                    callback(source_object, result, user_data)
                },
            );
            t.return_result(res.map(|_| true));
        }
    });
}

unsafe extern "C" fn panel_workbench_unload_finish(
    workbench: *mut ffi::PanelWorkbench,
    res: *mut gio::ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let workbench = from_glib_borrow::<_, Workbench>(workbench);
    let res: gio::AsyncResult = from_glib_none(res);
    let t = res.downcast::<gio::LocalTask<bool>>().unwrap();
    assert!(gio::LocalTask::<bool>::is_valid(
        &t,
        Some(workbench.as_ref())
    ));
    let ret = t.propagate();
    match ret {
        Ok(v) => {
            assert!(v);
            true.into_glib()
        }
        Err(e) => {
            if !error.is_null() {
                *error = e.into_glib_ptr();
            }
            false.into_glib()
        }
    }
}

pub unsafe trait WorkbenchClassSubclassExt: ClassStruct {
    fn install_action_async<Fut, F>(
        &mut self,
        action_name: &str,
        parameter_type: Option<&str>,
        activate: F,
    ) where
        F: Fn(
                <<Self as ClassStruct>::Type as ObjectSubclass>::Type,
                String,
                Option<Variant>,
            ) -> Fut
            + 'static
            + Clone,
        Fut: Future<Output = ()>,
    {
        self.install_action(
            action_name,
            parameter_type,
            move |this, action_name, parameter_type| {
                let ctx = glib::MainContext::default();
                let action_name = action_name.to_owned();
                let parameter_type = parameter_type.map(ToOwned::to_owned);
                ctx.spawn_local(glib::clone!(
                    #[strong]
                    this,
                    #[strong]
                    action_name,
                    #[strong]
                    parameter_type,
                    #[strong]
                    activate,
                    async move {
                        activate(this, action_name, parameter_type).await;
                    }
                ));
            },
        );
    }

    #[doc(alias = "panel_workbench_class_install_action")]
    fn install_action<F>(&mut self, action_name: &str, parameter_type: Option<&str>, activate: F)
    where
        F: Fn(&<<Self as ClassStruct>::Type as ObjectSubclass>::Type, &str, Option<&Variant>)
            + 'static,
    {
        unsafe {
            // We store the activate callbacks in a HashMap<action_name, activate>
            // so that we can retrieve f later on the activate_trampoline call
            let mut data = <Self::Type as ObjectSubclassType>::type_data();
            let data = data.as_mut();

            let f: Box<F> = Box::new(activate);

            let internal = data
                .class_data_mut::<Internal>(<Self::Type as ObjectSubclassType>::type_())
                .expect("Something bad happened at class_init, the internal class_data is missing");
            let callback_ptr = Box::into_raw(f) as glib::ffi::gpointer;
            internal
                .actions
                .insert(action_name.to_string(), callback_ptr);

            unsafe extern "C" fn activate_trampoline<F, S>(
                this: *mut libc::c_void,
                action_name: *const libc::c_char,
                parameter: *mut glib::ffi::GVariant,
            ) where
                S: ClassStruct,
                <S as ClassStruct>::Type: ObjectSubclass,
                F: Fn(&<<S as ClassStruct>::Type as ObjectSubclass>::Type, &str, Option<&Variant>)
                    + 'static,
            {
                let action_name = GString::from_glib_borrow(action_name);

                let data = <S::Type as ObjectSubclassType>::type_data();
                let internal = data
                    .as_ref()
                    .class_data::<Internal>(<S::Type as ObjectSubclassType>::type_())
                    .unwrap();
                let activate_callback =
                    *internal
                        .actions
                        .get(action_name.as_str())
                        .unwrap_or_else(|| {
                            panic!("Action name '{}' was not found", action_name.as_str());
                        });

                let workbench = Workbench::from_glib_borrow(this as *mut ffi::PanelWorkbench);

                let f: &F = &*(activate_callback as *const F);
                f(
                    workbench.unsafe_cast_ref(),
                    &action_name,
                    Option::<Variant>::from_glib_borrow(parameter)
                        .as_ref()
                        .as_ref(),
                )
            }
            let workbench_class = self as *mut _ as *mut ffi::PanelWorkbenchClass;
            let callback = activate_trampoline::<F, Self>;
            ffi::panel_workbench_class_install_action(
                workbench_class,
                action_name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                Some(callback),
            );
        }
    }

    #[cfg(any(feature = "v1_4", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_workbench_class_install_property_action")]
    fn install_property_action(&mut self, action_name: &str, property_name: &str) {
        unsafe {
            let workbench_class = self as *mut _ as *mut ffi::PanelWorkbenchClass;
            ffi::panel_workbench_class_install_property_action(
                workbench_class,
                action_name.to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }
}

unsafe impl<T: ClassStruct> WorkbenchClassSubclassExt for T where T::Type: WorkbenchImpl {}
