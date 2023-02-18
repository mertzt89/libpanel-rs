use crate::FrameHeader;
use crate::Widget;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait FrameHeaderImpl: WidgetImpl {
    fn page_changed(&self, widget: Option<&Widget>) {
        FrameHeaderImplExt::parent_page_changed(self, widget);
    }
    fn can_drop(&self, widget: &Widget) -> bool {
        FrameHeaderImplExt::parent_can_drop(self, widget)
    }
    fn add_prefix(&self, priority: i32, widget: &Widget) {
        FrameHeaderImplExt::parent_add_prefix(self, priority, widget);
    }
    fn add_suffix(&self, priority: i32, widget: &Widget) {
        FrameHeaderImplExt::parent_add_suffix(self, priority, widget);
    }
}

pub trait FrameHeaderImplExt: ObjectSubclass {
    fn parent_page_changed(&self, widget: Option<&Widget>);
    fn parent_can_drop(&self, widget: &Widget) -> bool;
    fn parent_add_prefix(&self, priority: i32, widget: &Widget);
    fn parent_add_suffix(&self, priority: i32, widget: &Widget);
}

impl<T: FrameHeaderImpl> FrameHeaderImplExt for T {
    fn parent_page_changed(&self, widget: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).page_changed {
                f(
                    self.obj().unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    widget
                        .map(|w| w.unsafe_cast_ref::<Widget>())
                        .to_glib_none()
                        .0,
                );
            }
        }
    }
    fn parent_can_drop(&self, widget: &Widget) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).can_drop {
                return f(
                    self.obj().unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                ) != 0;
            }
            false
        }
    }
    fn parent_add_prefix(&self, priority: i32, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).add_prefix {
                f(
                    self.obj().unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    priority,
                    widget.unsafe_cast_ref::<gtk::Widget>().to_glib_none().0,
                );
            }
        }
    }
    fn parent_add_suffix(&self, priority: i32, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).add_suffix {
                f(
                    self.obj().unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    priority,
                    widget.unsafe_cast_ref::<gtk::Widget>().to_glib_none().0,
                );
            }
        }
    }
}

unsafe impl<T: FrameHeaderImpl> IsImplementable<T> for FrameHeader {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();
        iface.page_changed = Some(frame_header_page_changed::<T>);
        iface.can_drop = Some(frame_header_can_drop::<T>);
        iface.add_prefix = Some(frame_header_add_prefix::<T>);
        iface.add_suffix = Some(frame_header_add_suffix::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn frame_header_page_changed<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Option<Borrowed<Widget>> = if widget.is_null() {
        None
    } else {
        Some(from_glib_borrow(widget))
    };

    FrameHeaderImpl::page_changed(imp, widget.as_ref().map(|w| w.unsafe_cast_ref()));
}

unsafe extern "C" fn frame_header_can_drop<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    widget: *mut ffi::PanelWidget,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::can_drop(imp, widget.unsafe_cast_ref()) as glib::ffi::gboolean
}

unsafe extern "C" fn frame_header_add_prefix<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    priority: std::os::raw::c_int,
    widget: *mut gtk::ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<gtk::Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::add_prefix(imp, priority, widget.unsafe_cast_ref());
}

unsafe extern "C" fn frame_header_add_suffix<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    priority: std::os::raw::c_int,
    widget: *mut gtk::ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<gtk::Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::add_suffix(imp, priority, widget.unsafe_cast_ref());
}
