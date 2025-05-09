use crate::{prelude::*, Frame, Grid};
use glib::translate::*;
use gtk::subclass::prelude::*;

pub trait PanelGridImpl: WidgetImpl {
    fn create_frame(&self) -> Option<Frame> {
        PanelGridImplExt::parent_create_frame(self)
    }
}

pub trait PanelGridImplExt: ObjectSubclass {
    fn parent_create_frame(&self) -> Option<Frame>;
}

impl<T: PanelGridImpl> PanelGridImplExt for T {
    fn parent_create_frame(&self) -> Option<Frame> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelGridClass;
            if let Some(f) = (*parent_class).create_frame {
                return from_glib_none(f(self.obj().unsafe_cast_ref::<Grid>().to_glib_none().0));
            }
            None
        }
    }
}

unsafe impl<T: PanelGridImpl> IsSubclassable<T> for Grid {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.create_frame = Some(panel_grid_create_frame::<T>);
    }
}

unsafe extern "C" fn panel_grid_create_frame<T: PanelGridImpl>(
    ptr: *mut ffi::PanelGrid,
) -> *mut ffi::PanelFrame {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    PanelGridImpl::create_frame(imp).to_glib_none().0
}
