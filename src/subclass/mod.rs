pub mod dock;
pub mod frame;
pub mod frame_header;
pub mod grid;
pub mod omni_bar;
pub mod save_delegate;
pub mod widget;

pub mod prelude {
    pub use super::dock::{DockImpl, DockImplExt};
    pub use super::frame::PanelFrameImpl;
    pub use super::frame_header::{FrameHeaderImpl, FrameHeaderImplExt};
    pub use super::grid::{PanelGridImpl, PanelGridImplExt};
    pub use super::omni_bar::PanelOmniBarImpl;
    pub use super::save_delegate::{SaveDelegateImpl, SaveDelegateImplExt};
    pub use super::widget::{PanelWidgetImpl, PanelWidgetImplExt};
    pub use gtk::subclass::prelude::*;
}
