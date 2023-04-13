use crate::Application;
use adw::subclass::prelude::*;

pub trait PanelApplicationImpl: AdwApplicationImpl {}

unsafe impl<T: PanelApplicationImpl> IsSubclassable<T> for Application {}
