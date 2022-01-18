// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

mod dock;
pub use self::dock::Dock;

mod dock_switcher;
pub use self::dock_switcher::DockSwitcher;

mod frame;
pub use self::frame::Frame;

mod frame_header;
pub use self::frame_header::FrameHeader;

mod frame_header_bar;
pub use self::frame_header_bar::FrameHeaderBar;

mod frame_switcher;
pub use self::frame_switcher::FrameSwitcher;

mod frame_tab_bar;
pub use self::frame_tab_bar::FrameTabBar;

mod grid;
pub use self::grid::Grid;

mod grid_column;
pub use self::grid_column::GridColumn;

mod omni_bar;
pub use self::omni_bar::OmniBar;

mod paned;
pub use self::paned::Paned;

mod save_delegate;
pub use self::save_delegate::SaveDelegate;

mod statusbar;
pub use self::statusbar::Statusbar;

mod theme_selector;
pub use self::theme_selector::ThemeSelector;

mod widget;
pub use self::widget::Widget;

mod enums;
pub use self::enums::DockPosition;

pub mod functions;

mod constants;
pub use self::constants::VERSION_S;
pub use self::constants::WIDGET_KIND_ANY;
pub use self::constants::WIDGET_KIND_DOCUMENT;
pub use self::constants::WIDGET_KIND_UNKNOWN;
pub use self::constants::WIDGET_KIND_UTILITY;

#[doc(hidden)]
pub mod traits {
    pub use super::dock::DockExt;
    pub use super::frame::FrameExt;
    pub use super::frame_header::FrameHeaderExt;
    pub use super::omni_bar::OmniBarExt;
    pub use super::save_delegate::SaveDelegateExt;
    pub use super::widget::WidgetExt;
}
#[doc(hidden)]
pub mod builders {
    pub use super::dock::DockBuilder;
    pub use super::dock_switcher::DockSwitcherBuilder;
    pub use super::frame::FrameBuilder;
    pub use super::frame_header_bar::FrameHeaderBarBuilder;
    pub use super::frame_switcher::FrameSwitcherBuilder;
    pub use super::frame_tab_bar::FrameTabBarBuilder;
    pub use super::grid::GridBuilder;
    pub use super::grid_column::GridColumnBuilder;
    pub use super::omni_bar::OmniBarBuilder;
    pub use super::paned::PanedBuilder;
    pub use super::save_delegate::SaveDelegateBuilder;
    pub use super::statusbar::StatusbarBuilder;
    pub use super::theme_selector::ThemeSelectorBuilder;
    pub use super::widget::WidgetBuilder;
}
