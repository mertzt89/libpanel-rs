use crate::Dock;
use crate::Widget;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait DockImpl: WidgetImpl {
    fn panel_drag_begin(&self, dock: &Self::Type, widget: &Widget) {
        DockImplExt::parent_panel_drag_begin(self, dock, widget);
    }
    fn panel_drag_end(&self, dock: &Self::Type, widget: &Widget) {
        DockImplExt::parent_panel_drag_end(self, dock, widget);
    }
}

pub trait DockImplExt: ObjectSubclass {
    fn parent_panel_drag_begin(&self, dock: &Self::Type, widget: &Widget);
    fn parent_panel_drag_end(&self, dock: &Self::Type, widget: &Widget);
}

impl<T: DockImpl> DockImplExt for T {
    fn parent_panel_drag_begin(&self, dock: &Self::Type, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelDockClass;
            if let Some(f) = (*parent_class).panel_drag_begin {
                f(
                    dock.unsafe_cast_ref::<Dock>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                );
            }
        }
    }
    fn parent_panel_drag_end(&self, dock: &Self::Type, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelDockClass;
            if let Some(f) = (*parent_class).panel_drag_end {
                f(
                    dock.unsafe_cast_ref::<Dock>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: DockImpl> IsSubclassable<T> for Dock {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);
        let klass = class.as_mut();
        klass.panel_drag_begin = Some(dock_panel_drag_begin::<T>);
        klass.panel_drag_end = Some(dock_panel_drag_end::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn dock_panel_drag_begin<T: DockImpl>(
    ptr: *mut ffi::PanelDock,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Dock> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    DockImpl::panel_drag_begin(imp, wrap.unsafe_cast_ref(), widget.unsafe_cast_ref());
}

unsafe extern "C" fn dock_panel_drag_end<T: DockImpl>(
    ptr: *mut ffi::PanelDock,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Dock> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    DockImpl::panel_drag_end(imp, wrap.unsafe_cast_ref(), widget.unsafe_cast_ref());
}
