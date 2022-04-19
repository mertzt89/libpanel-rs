use crate::Frame;
use crate::Grid;
use glib::translate::*;
use glib::Cast;
use gtk::subclass::prelude::*;

pub trait PanelGridImpl: WidgetImpl {
    fn create_frame(&self, grid: &Self::Type) -> Option<Frame> {
        PanelGridImplExt::parent_create_frame(self, grid)
    }
}

pub trait PanelGridImplExt: ObjectSubclass {
    fn parent_create_frame(&self, grid: &Self::Type) -> Option<Frame>;
}

impl<T: PanelGridImpl> PanelGridImplExt for T {
    fn parent_create_frame(&self, grid: &Self::Type) -> Option<Frame> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PanelGridClass;
            if let Some(f) = (*parent_class).create_frame {
                return from_glib_none(f(grid.unsafe_cast_ref::<Grid>().to_glib_none().0));
            }
            None
        }
    }
}

unsafe impl<T: PanelGridImpl> IsSubclassable<T> for Grid {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);
        let klass = class.as_mut();
        klass.create_frame = Some(panel_grid_create_frame::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Widget as IsSubclassable<T>>::instance_init(instance);
    }
}

unsafe extern "C" fn panel_grid_create_frame<T: PanelGridImpl>(
    ptr: *mut ffi::PanelGrid,
) -> *mut ffi::PanelFrame {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Grid> = from_glib_borrow(ptr);

    PanelGridImpl::create_frame(imp, wrap.unsafe_cast_ref())
        .to_glib_none()
        .0
}
