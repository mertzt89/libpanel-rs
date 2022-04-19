use crate::Widget;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait PanelWidgetImpl: WidgetImpl {
    fn default_focus(&self, widget: &Self::Type) -> Option<gtk::Widget> {
        PanelWidgetImplExt::parent_default_focus(self, widget)
    }
    fn presented(&self, widget: &Self::Type) {
        PanelWidgetImplExt::parent_presented(self, widget)
    }
}

pub trait PanelWidgetImplExt: ObjectSubclass {
    fn parent_default_focus(&self, widget: &Self::Type) -> Option<gtk::Widget>;
    fn parent_presented(&self, widget: &Self::Type);
}

impl<T: PanelWidgetImpl> PanelWidgetImplExt for T {
    fn parent_default_focus(&self, widget: &Self::Type) -> Option<gtk::Widget> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelWidgetClass;
            if let Some(f) = (*parent_class).get_default_focus {
                return from_glib_none(f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0));
            }
            None
        }
    }
    fn parent_presented(&self, widget: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelWidgetClass;
            if let Some(f) = (*parent_class).presented {
                f(widget.unsafe_cast_ref::<Widget>().to_glib_none().0);
            }
        }
    }
}

unsafe impl<T: PanelWidgetImpl> IsSubclassable<T> for Widget {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);
        let klass = class.as_mut();
        klass.get_default_focus = Some(widget_get_default_focus::<T>);
        klass.presented = Some(widget_presented::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn widget_get_default_focus<T: PanelWidgetImpl>(
    ptr: *mut ffi::PanelWidget,
) -> *mut gtk::ffi::GtkWidget {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    PanelWidgetImpl::default_focus(imp, wrap.unsafe_cast_ref())
        .to_glib_none()
        .0
}

unsafe extern "C" fn widget_presented<T: PanelWidgetImpl>(ptr: *mut ffi::PanelWidget) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Widget> = from_glib_borrow(ptr);

    PanelWidgetImpl::presented(imp, wrap.unsafe_cast_ref());
}
