use crate::{prelude::*, Workspace};
use adw::subclass::prelude::*;
use glib::translate::*;
use glib::GString;
use glib::Variant;
use std::collections::HashMap;
use std::future::Future;

#[derive(Debug, Default)]
struct Internal {
    actions: HashMap<String, glib::ffi::gpointer>,
}
unsafe impl Sync for Internal {}
unsafe impl Send for Internal {}

pub trait WorkspaceImpl: AdwApplicationWindowImpl {}

unsafe impl<T: WorkspaceImpl> IsSubclassable<T> for Workspace {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        unsafe {
            let mut data = T::type_data();
            let data = data.as_mut();
            // Used to store actions for `install_action` and `rust_builder_scope`
            data.set_class_data(<T as ObjectSubclassType>::type_(), Internal::default());
        }
    }
}

pub unsafe trait WorkspaceClassSubclassExt: ClassStruct {
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
                ctx.spawn_local(glib::clone!(@strong this, @strong action_name, @strong parameter_type, @strong activate => async move {
                    activate(this, action_name, parameter_type).await;
                }));
            },
        );
    }

    #[doc(alias = "panel_workspace_class_install_action")]
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

                let workspace = Workspace::from_glib_borrow(this as *mut ffi::PanelWorkspace);

                let f: &F = &*(activate_callback as *const F);
                f(
                    workspace.unsafe_cast_ref(),
                    &action_name,
                    Option::<Variant>::from_glib_borrow(parameter)
                        .as_ref()
                        .as_ref(),
                )
            }
            let workspace_class = self as *mut _ as *mut ffi::PanelWorkspaceClass;
            let callback = activate_trampoline::<F, Self>;
            ffi::panel_workspace_class_install_action(
                workspace_class,
                action_name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                Some(callback),
            );
        }
    }

    #[cfg(any(feature = "v1_4", docsrs))]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_4")))]
    #[doc(alias = "panel_workspace_class_install_property_action")]
    fn install_property_action(&mut self, action_name: &str, property_name: &str) {
        unsafe {
            let workspace_class = self as *mut _ as *mut ffi::PanelWorkspaceClass;
            ffi::panel_workspace_class_install_property_action(
                workspace_class,
                action_name.to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }
}

unsafe impl<T: ClassStruct> WorkspaceClassSubclassExt for T where T::Type: WorkspaceImpl {}
