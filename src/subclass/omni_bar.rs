use crate::OmniBar;
use gtk::subclass::prelude::*;

pub trait PanelOmniBarImpl: WidgetImpl {}

unsafe impl<T: PanelOmniBarImpl> IsSubclassable<T> for OmniBar {
    fn class_init(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <gtk::Widget as IsSubclassable<T>>::instance_init(instance);
    }
}
