use crate::OmniBar;
use gtk::subclass::prelude::*;

pub trait OmniBarImpl: WidgetImpl {}

unsafe impl<T: OmniBarImpl> IsSubclassable<T> for OmniBar {}
