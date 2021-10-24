use crate::FrameHeader;
use crate::Widget;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait FrameHeaderImpl: WidgetImpl {
    fn page_changed(&self, header: &Self::Type, widget: Option<&Widget>) {
        FrameHeaderImplExt::parent_page_changed(self, header, widget);
    }
    fn can_drop(&self, header: &Self::Type, widget: &Widget) -> bool {
        FrameHeaderImplExt::parent_can_drop(self, header, widget)
    }
    fn pack_start(&self, header: &Self::Type, priority: i32, widget: &Widget) {
        FrameHeaderImplExt::parent_pack_start(self, header, priority, widget);
    }
    fn pack_end(&self, header: &Self::Type, priority: i32, widget: &Widget) {
        FrameHeaderImplExt::parent_pack_end(self, header, priority, widget);
    }
}

pub trait FrameHeaderImplExt: ObjectSubclass {
    fn parent_page_changed(&self, header: &Self::Type, widget: Option<&Widget>);
    fn parent_can_drop(&self, header: &Self::Type, widget: &Widget) -> bool;
    fn parent_pack_start(&self, header: &Self::Type, priority: i32, widget: &Widget);
    fn parent_pack_end(&self, header: &Self::Type, priority: i32, widget: &Widget);
}

impl<T: FrameHeaderImpl> FrameHeaderImplExt for T {
    fn parent_page_changed(&self, header: &Self::Type, widget: Option<&Widget>) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).page_changed {
                f(
                    header.unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    widget
                        .map(|w| w.unsafe_cast_ref::<Widget>())
                        .to_glib_none()
                        .0,
                );
            }
        }
    }
    fn parent_can_drop(&self, header: &Self::Type, widget: &Widget) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).can_drop {
                return f(
                    header.unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    widget.unsafe_cast_ref::<Widget>().to_glib_none().0,
                ) != 0;
            }
            false
        }
    }
    fn parent_pack_start(&self, header: &Self::Type, priority: i32, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).pack_start {
                f(
                    header.unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
                    priority,
                    widget.unsafe_cast_ref::<gtk::Widget>().to_glib_none().0,
                );
            }
        }
    }
    fn parent_pack_end(&self, header: &Self::Type, priority: i32, widget: &Widget) {
        unsafe {
            let data = T::type_data();
            let parent_iface = data.as_ref().parent_interface::<FrameHeader>()
                as *const ffi::PanelFrameHeaderInterface;
            if let Some(f) = (*parent_iface).pack_end {
                f(
                    header.unsafe_cast_ref::<FrameHeader>().to_glib_none().0,
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
        iface.pack_start = Some(frame_header_pack_start::<T>);
        iface.pack_end = Some(frame_header_pack_end::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn frame_header_page_changed<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    widget: *mut ffi::PanelWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<FrameHeader> = from_glib_borrow(ptr);
    let widget: Option<Borrowed<Widget>> = if widget.is_null() {
        None
    } else {
        Some(from_glib_borrow(widget))
    };

    FrameHeaderImpl::page_changed(
        imp,
        wrap.unsafe_cast_ref(),
        widget.as_ref().map(|w| w.unsafe_cast_ref()),
    );
}

unsafe extern "C" fn frame_header_can_drop<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    widget: *mut ffi::PanelWidget,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<FrameHeader> = from_glib_borrow(ptr);
    let widget: Borrowed<Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::can_drop(imp, wrap.unsafe_cast_ref(), widget.unsafe_cast_ref())
        as glib::ffi::gboolean
}

unsafe extern "C" fn frame_header_pack_start<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    priority: std::os::raw::c_int,
    widget: *mut gtk::ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<FrameHeader> = from_glib_borrow(ptr);
    let widget: Borrowed<gtk::Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::pack_start(
        imp,
        wrap.unsafe_cast_ref(),
        priority,
        widget.unsafe_cast_ref(),
    );
}

unsafe extern "C" fn frame_header_pack_end<T: FrameHeaderImpl>(
    ptr: *mut ffi::PanelFrameHeader,
    priority: std::os::raw::c_int,
    widget: *mut gtk::ffi::GtkWidget,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.impl_();
    let wrap: Borrowed<FrameHeader> = from_glib_borrow(ptr);
    let widget: Borrowed<gtk::Widget> = from_glib_borrow(widget);

    FrameHeaderImpl::pack_end(
        imp,
        wrap.unsafe_cast_ref(),
        priority,
        widget.unsafe_cast_ref(),
    );
}
