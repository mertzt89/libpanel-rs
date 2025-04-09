use crate::{prelude::*, Widget};
use glib::translate::*;
use glib::GString;
use glib::Variant;
use gtk::subclass::prelude::*;
use std::collections::HashMap;
use std::future::Future;

#[derive(Debug, Default)]
struct Internal {
    actions: HashMap<String, glib::ffi::gpointer>,
}
unsafe impl Sync for Internal {}
unsafe impl Send for Internal {}

pub trait PanelWidgetImpl: WidgetImpl {
    fn default_focus(&self) -> Option<gtk::Widget> {
        PanelWidgetImplExt::parent_default_focus(self)
    }
    fn presented(&self) {
        PanelWidgetImplExt::parent_presented(self)
    }
}

pub trait PanelWidgetImplExt: ObjectSubclass {
    fn parent_default_focus(&self) -> Option<gtk::Widget>;
    fn parent_presented(&self);
}

impl<T: PanelWidgetImpl> PanelWidgetImplExt for T {
    fn parent_default_focus(&self) -> Option<gtk::Widget> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelWidgetClass;
            if let Some(f) = (*parent_class).get_default_focus {
                return from_glib_none(f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0));
            }
            None
        }
    }
    fn parent_presented(&self) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelWidgetClass;
            if let Some(f) = (*parent_class).presented {
                f(self.obj().unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }
}

unsafe impl<T: PanelWidgetImpl> IsSubclassable<T> for Widget {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        unsafe {
            let mut data = T::type_data();
            let data = data.as_mut();
            // Used to store actions for `install_action` and `rust_builder_scope`
            data.set_class_data(<T as ObjectSubclassType>::type_(), Internal::default());
        }
        let klass = class.as_mut();
        klass.get_default_focus = Some(widget_get_default_focus::<T>);
        klass.presented = Some(widget_presented::<T>);
    }
}

unsafe extern "C" fn widget_get_default_focus<T: PanelWidgetImpl>(
    ptr: *mut ffi::PanelWidget,
) -> *mut gtk::ffi::GtkWidget {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    PanelWidgetImpl::default_focus(imp).to_glib_none().0
}

unsafe extern "C" fn widget_presented<T: PanelWidgetImpl>(ptr: *mut ffi::PanelWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    PanelWidgetImpl::presented(imp);
}

pub unsafe trait PanelWidgetClassSubclassExt: ClassStruct {
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

    #[doc(alias = "panel_widget_class_install_action")]
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
                this: *mut gtk::ffi::GtkWidget,
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

                let widget = gtk::Widget::from_glib_borrow(this);

                let f: &F = &*(activate_callback as *const F);
                f(
                    widget.unsafe_cast_ref(),
                    &action_name,
                    Option::<Variant>::from_glib_borrow(parameter)
                        .as_ref()
                        .as_ref(),
                )
            }
            let widget_class = self as *mut _ as *mut ffi::PanelWidgetClass;
            let callback = activate_trampoline::<F, Self>;
            ffi::panel_widget_class_install_action(
                widget_class,
                action_name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                Some(callback),
            );
        }
    }
    #[doc(alias = "panel_widget_class_install_property_action")]
    fn install_property_action(&mut self, action_name: &str, property_name: &str) {
        unsafe {
            let widget_class = self as *mut _ as *mut ffi::PanelWidgetClass;
            ffi::panel_widget_class_install_property_action(
                widget_class,
                action_name.to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }
}

unsafe impl<T: ClassStruct> PanelWidgetClassSubclassExt for T where T::Type: PanelWidgetImpl {}
