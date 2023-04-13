use crate::Dock;
use crate::Widget;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait DockImpl: WidgetImpl {
    fn panel_drag_begin(&self, widget: &Widget) {
        DockImplExt::parent_panel_drag_begin(self, widget);
    }
    fn panel_drag_end(&self, widget: &Widget) {
        DockImplExt::parent_panel_drag_end(self, widget);
    }
}

pub trait DockImplExt: ObjectSubclass {
    fn parent_panel_drag_begin(&self, widget: &Widget);
    fn parent_panel_drag_end(&self, widget: &Widget);
}

impl<T: DockImpl> DockImplExt for T {
    fn parent_panel_drag_begin(&self, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelDockClass;
            if let Some(f) = (*parent_class).panel_drag_begin {
                f(
                    self.obj().unsafe_cast_ref::<Dock>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                );
            }
        }
    }
    fn parent_panel_drag_end(&self, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelDockClass;
            if let Some(f) = (*parent_class).panel_drag_end {
                f(
                    self.obj().unsafe_cast_ref::<Dock>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: DockImpl> IsSubclassable<T> for Dock {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.panel_drag_begin = Some(dock_panel_drag_begin::<T>);
        klass.panel_drag_end = Some(dock_panel_drag_end::<T>);
    }
}

unsafe extern "C" fn dock_panel_drag_begin<T: DockImpl>(
    ptr: *mut ffi::PanelDock,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    DockImpl::panel_drag_begin(imp, widget.unsafe_cast_ref());
}

unsafe extern "C" fn dock_panel_drag_end<T: DockImpl>(
    ptr: *mut ffi::PanelDock,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    DockImpl::panel_drag_end(imp, widget.unsafe_cast_ref());
}
