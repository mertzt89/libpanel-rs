// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Frame;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "PanelFrameHeader")]
    pub struct FrameHeader(Interface<ffi::PanelFrameHeader, ffi::PanelFrameHeaderInterface>) @requires gtk::Widget, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_frame_header_get_type(),
    }
}

impl FrameHeader {
    pub const NONE: Option<&'static FrameHeader> = None;
}

pub trait FrameHeaderExt: 'static {
    #[doc(alias = "panel_frame_header_add_prefix")]
    fn add_prefix(&self, priority: i32, child: &impl IsA<gtk::Widget>);

    #[doc(alias = "panel_frame_header_add_suffix")]
    fn add_suffix(&self, priority: i32, child: &impl IsA<gtk::Widget>);

    #[doc(alias = "panel_frame_header_can_drop")]
    fn can_drop(&self, widget: &impl IsA<Widget>) -> bool;

    #[doc(alias = "panel_frame_header_get_frame")]
    #[doc(alias = "get_frame")]
    fn frame(&self) -> Option<Frame>;

    #[doc(alias = "panel_frame_header_page_changed")]
    fn page_changed(&self, widget: Option<&impl IsA<Widget>>);

    #[doc(alias = "panel_frame_header_set_frame")]
    fn set_frame(&self, frame: Option<&impl IsA<Frame>>);

    #[doc(alias = "frame")]
    fn connect_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FrameHeader>> FrameHeaderExt for O {
    fn add_prefix(&self, priority: i32, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::panel_frame_header_add_prefix(
                self.as_ref().to_glib_none().0,
                priority,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_suffix(&self, priority: i32, child: &impl IsA<gtk::Widget>) {
        unsafe {
            ffi::panel_frame_header_add_suffix(
                self.as_ref().to_glib_none().0,
                priority,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn can_drop(&self, widget: &impl IsA<Widget>) -> bool {
        unsafe {
            from_glib(ffi::panel_frame_header_can_drop(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
            ))
        }
    }

    fn frame(&self) -> Option<Frame> {
        unsafe {
            from_glib_none(ffi::panel_frame_header_get_frame(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn page_changed(&self, widget: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::panel_frame_header_page_changed(
                self.as_ref().to_glib_none().0,
                widget.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_frame(&self, frame: Option<&impl IsA<Frame>>) {
        unsafe {
            ffi::panel_frame_header_set_frame(
                self.as_ref().to_glib_none().0,
                frame.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_frame_trampoline<P: IsA<FrameHeader>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelFrameHeader,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(FrameHeader::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::frame\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_frame_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FrameHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FrameHeader")
    }
}
