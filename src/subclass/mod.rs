pub mod dock;
pub mod frame;
pub mod frame_header;
pub mod widget;

pub mod prelude {
    pub use super::dock::{DockImpl, DockImplExt};
    pub use super::frame::PanelFrameImpl;
    pub use super::frame_header::{FrameHeaderImpl, FrameHeaderImplExt};
    pub use super::widget::{PanelWidgetImpl, PanelWidgetImplExt};
    pub use gtk::subclass::prelude::*;
}
