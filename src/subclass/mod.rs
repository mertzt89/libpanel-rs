pub mod application;
pub mod dock;
pub mod document_workspace;
pub mod frame;
pub mod frame_header;
pub mod grid;
pub mod omni_bar;
pub mod save_delegate;
pub mod widget;
pub mod workbench;
pub mod workspace;

pub mod prelude {
    pub use super::application::PanelApplicationImpl;
    pub use super::dock::{DockImpl, DockImplExt};
    pub use super::document_workspace::{DocumentWorkspaceImpl, DocumentWorkspaceImplExt};
    pub use super::frame::{PanelFrameImpl, PanelFrameImplExt};
    pub use super::frame_header::{FrameHeaderImpl, FrameHeaderImplExt};
    pub use super::grid::{PanelGridImpl, PanelGridImplExt};
    pub use super::omni_bar::OmniBarImpl;
    pub use super::save_delegate::{SaveDelegateImpl, SaveDelegateImplExt};
    pub use super::widget::{PanelWidgetClassSubclassExt, PanelWidgetImpl, PanelWidgetImplExt};
    pub use super::workbench::{WorkbenchClassSubclassExt, WorkbenchImpl, WorkbenchImplExt};
    pub use super::workspace::{WorkspaceClassSubclassExt, WorkspaceImpl};
    pub use gtk::subclass::prelude::*;
}
