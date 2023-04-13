use crate::{Frame, Widget};
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait PanelFrameImpl: WidgetImpl {
    fn page_closed(&self, widget: &Widget) {
        PanelFrameImplExt::parent_page_closed(self, widget)
    }
    fn adopt_widget(&self, widget: &Widget) -> bool {
        PanelFrameImplExt::parent_adopt_widget(self, widget)
    }
}

pub trait PanelFrameImplExt: ObjectSubclass {
    fn parent_page_closed(&self, widget: &Widget);
    fn parent_adopt_widget(&self, widget: &Widget) -> bool;
}

impl<T: PanelFrameImpl> PanelFrameImplExt for T {
    fn parent_page_closed(&self, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelFrameClass;
            if let Some(f) = (*parent_class).page_closed {
                f(
                    self.obj().unsafe_cast_ref::<Frame>().to_glib_none().0,
                    widget.to_glib_none().0,
                );
            }
        }
    }
    fn parent_adopt_widget(&self, widget: &Widget) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelFrameClass;
            if let Some(f) = (*parent_class).adopt_widget {
                return f(
                    self.obj().unsafe_cast_ref::<Frame>().to_glib_none().0,
                    widget.to_glib_none().0,
                ) != 0;
            }
        }
        false
    }
}

unsafe impl<T: PanelFrameImpl> IsSubclassable<T> for Frame {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.page_closed = Some(panel_frame_page_closed::<T>);
        klass.adopt_widget = Some(panel_frame_adopt_widget::<T>);
    }
}

unsafe extern "C" fn panel_frame_page_closed<T: PanelFrameImpl>(
    ptr: *mut ffi::PanelFrame,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    PanelFrameImpl::page_closed(imp, &widget);
}

unsafe extern "C" fn panel_frame_adopt_widget<T: PanelFrameImpl>(
    ptr: *mut ffi::PanelFrame,
    widget: *mut ffi::PanelWidget,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    PanelFrameImpl::adopt_widget(imp, &widget) as glib::ffi::gboolean
}
