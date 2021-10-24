// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files
// DO NOT EDIT

use crate::Frame;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "PanelDock")]
    pub struct Dock(Object<ffi::PanelDock, ffi::PanelDockClass>) @extends gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;

    match fn {
        type_ => || ffi::panel_dock_get_type(),
    }
}

impl Dock {
    #[doc(alias = "panel_dock_new")]
    pub fn new() -> Dock {
        assert_initialized_main_thread!();
        unsafe { gtk::Widget::from_glib_none(ffi::panel_dock_new()).unsafe_cast() }
    }
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DOCK: Option<&Dock> = None;

pub trait DockExt: 'static {
    #[doc(alias = "panel_dock_foreach_frame")]
    fn foreach_frame<P: FnMut(&Frame)>(&self, callback: P);

    #[doc(alias = "panel_dock_get_can_reveal_bottom")]
    #[doc(alias = "get_can_reveal_bottom")]
    fn can_reveal_bottom(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_end")]
    #[doc(alias = "get_can_reveal_end")]
    fn can_reveal_end(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_start")]
    #[doc(alias = "get_can_reveal_start")]
    fn can_reveal_start(&self) -> bool;

    #[doc(alias = "panel_dock_get_can_reveal_top")]
    #[doc(alias = "get_can_reveal_top")]
    fn can_reveal_top(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_bottom")]
    #[doc(alias = "get_reveal_bottom")]
    fn reveals_bottom(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_end")]
    #[doc(alias = "get_reveal_end")]
    fn reveals_end(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_start")]
    #[doc(alias = "get_reveal_start")]
    fn reveals_start(&self) -> bool;

    #[doc(alias = "panel_dock_get_reveal_top")]
    #[doc(alias = "get_reveal_top")]
    fn reveals_top(&self) -> bool;

    #[doc(alias = "panel_dock_set_bottom_height")]
    fn set_bottom_height(&self, height: i32);

    #[doc(alias = "panel_dock_set_end_width")]
    fn set_end_width(&self, width: i32);

    #[doc(alias = "panel_dock_set_reveal_bottom")]
    fn set_reveal_bottom(&self, reveal_bottom: bool);

    #[doc(alias = "panel_dock_set_reveal_end")]
    fn set_reveal_end(&self, reveal_end: bool);

    #[doc(alias = "panel_dock_set_reveal_start")]
    fn set_reveal_start(&self, reveal_start: bool);

    #[doc(alias = "panel_dock_set_reveal_top")]
    fn set_reveal_top(&self, reveal_top: bool);

    #[doc(alias = "panel_dock_set_start_width")]
    fn set_start_width(&self, width: i32);

    #[doc(alias = "panel_dock_set_top_height")]
    fn set_top_height(&self, height: i32);

    #[doc(alias = "bottom-height")]
    fn bottom_height(&self) -> i32;

    #[doc(alias = "end-width")]
    fn end_width(&self) -> i32;

    #[doc(alias = "start-width")]
    fn start_width(&self) -> i32;

    #[doc(alias = "top-height")]
    fn top_height(&self) -> i32;

    #[doc(alias = "panel-drag-begin")]
    fn connect_panel_drag_begin<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "panel-drag-end")]
    fn connect_panel_drag_end<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "bottom-height")]
    fn connect_bottom_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-bottom")]
    fn connect_can_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-end")]
    fn connect_can_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-start")]
    fn connect_can_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "can-reveal-top")]
    fn connect_can_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "end-width")]
    fn connect_end_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-bottom")]
    fn connect_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-end")]
    fn connect_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-start")]
    fn connect_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "reveal-top")]
    fn connect_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "start-width")]
    fn connect_start_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "top-height")]
    fn connect_top_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Dock>> DockExt for O {
    fn foreach_frame<P: FnMut(&Frame)>(&self, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&Frame)>(
            frame: *mut ffi::PanelFrame,
            user_data: glib::ffi::gpointer,
        ) {
            let frame = from_glib_borrow(frame);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&frame);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::panel_dock_foreach_frame(
                self.as_ref().to_glib_none().0,
                callback,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn can_reveal_bottom(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_bottom(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_end(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_end(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_start(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn can_reveal_top(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_can_reveal_top(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_bottom(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_bottom(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_end(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_end(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_start(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_start(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reveals_top(&self) -> bool {
        unsafe {
            from_glib(ffi::panel_dock_get_reveal_top(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_bottom_height(&self, height: i32) {
        unsafe {
            ffi::panel_dock_set_bottom_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn set_end_width(&self, width: i32) {
        unsafe {
            ffi::panel_dock_set_end_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_reveal_bottom(&self, reveal_bottom: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_bottom(
                self.as_ref().to_glib_none().0,
                reveal_bottom.into_glib(),
            );
        }
    }

    fn set_reveal_end(&self, reveal_end: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_end(self.as_ref().to_glib_none().0, reveal_end.into_glib());
        }
    }

    fn set_reveal_start(&self, reveal_start: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_start(
                self.as_ref().to_glib_none().0,
                reveal_start.into_glib(),
            );
        }
    }

    fn set_reveal_top(&self, reveal_top: bool) {
        unsafe {
            ffi::panel_dock_set_reveal_top(self.as_ref().to_glib_none().0, reveal_top.into_glib());
        }
    }

    fn set_start_width(&self, width: i32) {
        unsafe {
            ffi::panel_dock_set_start_width(self.as_ref().to_glib_none().0, width);
        }
    }

    fn set_top_height(&self, height: i32) {
        unsafe {
            ffi::panel_dock_set_top_height(self.as_ref().to_glib_none().0, height);
        }
    }

    fn bottom_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"bottom-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `bottom-height` getter")
        }
    }

    fn end_width(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"end-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `end-width` getter")
        }
    }

    fn start_width(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"start-width\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `start-width` getter")
        }
    }

    fn top_height(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"top-height\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `top-height` getter")
        }
    }

    fn connect_panel_drag_begin<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn panel_drag_begin_trampoline<
            P: IsA<Dock>,
            F: Fn(&P, &Widget) + 'static,
        >(
            this: *mut ffi::PanelDock,
            panel: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Dock::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(panel),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"panel-drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    panel_drag_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_panel_drag_end<F: Fn(&Self, &Widget) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn panel_drag_end_trampoline<
            P: IsA<Dock>,
            F: Fn(&P, &Widget) + 'static,
        >(
            this: *mut ffi::PanelDock,
            panel: *mut ffi::PanelWidget,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Dock::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(panel),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"panel-drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    panel_drag_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_bottom_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bottom_height_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::bottom-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bottom_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_bottom_trampoline<
            P: IsA<Dock>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-bottom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_bottom_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_end_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_start_trampoline<
            P: IsA<Dock>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_can_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_can_reveal_top_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::can-reveal-top\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_can_reveal_top_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_end_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_end_width_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::end-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_end_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_bottom_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_bottom_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-bottom\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_bottom_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_end_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_start_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-start\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_start_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_reveal_top_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_reveal_top_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::reveal-top\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_reveal_top_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_start_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_start_width_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::start-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_start_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_top_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_top_height_trampoline<P: IsA<Dock>, F: Fn(&P) + 'static>(
            this: *mut ffi::PanelDock,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Dock::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::top-height\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_top_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Dock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Dock")
    }
}
