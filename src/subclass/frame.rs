use crate::Frame;
use gtk::subclass::prelude::*;

pub trait PanelFrameImpl: WidgetImpl {}

unsafe impl<T: PanelFrameImpl> IsSubclassable<T> for Frame {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
