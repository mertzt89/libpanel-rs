// Generated by gir (https://github.com/gtk-rs/gir @ e43ea0b348f0)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 01c4ec663b3f)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type PanelArea = c_int;
pub const PANEL_AREA_START: PanelArea = 0;
pub const PANEL_AREA_END: PanelArea = 1;
pub const PANEL_AREA_TOP: PanelArea = 2;
pub const PANEL_AREA_BOTTOM: PanelArea = 3;
pub const PANEL_AREA_CENTER: PanelArea = 4;

// Constants
pub const PANEL_MAJOR_VERSION: c_int = 1;
pub const PANEL_MICRO_VERSION: c_int = 0;
pub const PANEL_MINOR_VERSION: c_int = 0;
pub const PANEL_VERSION_S: *const c_char = b"1.0.0\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_ANY: *const c_char = b"*\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_DOCUMENT: *const c_char = b"document\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_UNKNOWN: *const c_char = b"unknown\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_UTILITY: *const c_char = b"utility\0" as *const u8 as *const c_char;

// Callbacks
pub type PanelFrameCallback = Option<unsafe extern "C" fn(*mut PanelFrame, gpointer)>;

// Records
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelDockClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub panel_drag_begin: Option<unsafe extern "C" fn(*mut PanelDock, *mut PanelWidget)>,
    pub panel_drag_end: Option<unsafe extern "C" fn(*mut PanelDock, *mut PanelWidget)>,
}

impl ::std::fmt::Debug for PanelDockClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelDockClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("panel_drag_begin", &self.panel_drag_begin)
            .field("panel_drag_end", &self.panel_drag_end)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrameClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub _reserved: [gpointer; 8],
}

impl ::std::fmt::Debug for PanelFrameClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrameHeaderBarClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelFrameHeaderBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameHeaderBarClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrameHeaderInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub page_changed: Option<unsafe extern "C" fn(*mut PanelFrameHeader, *mut PanelWidget)>,
    pub can_drop: Option<unsafe extern "C" fn(*mut PanelFrameHeader, *mut PanelWidget) -> gboolean>,
    pub add_prefix: Option<unsafe extern "C" fn(*mut PanelFrameHeader, c_int, *mut gtk::GtkWidget)>,
    pub add_suffix: Option<unsafe extern "C" fn(*mut PanelFrameHeader, c_int, *mut gtk::GtkWidget)>,
}

impl ::std::fmt::Debug for PanelFrameHeaderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameHeaderInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("page_changed", &self.page_changed)
            .field("can_drop", &self.can_drop)
            .field("add_prefix", &self.add_prefix)
            .field("add_suffix", &self.add_suffix)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrameSwitcherClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelFrameSwitcherClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameSwitcherClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrameTabBarClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelFrameTabBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameTabBarClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelGridClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub create_frame: Option<unsafe extern "C" fn(*mut PanelGrid) -> *mut PanelFrame>,
    pub _reserved: [gpointer; 12],
}

impl ::std::fmt::Debug for PanelGridClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGridClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("create_frame", &self.create_frame)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelGridColumnClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelGridColumnClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGridColumnClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelOmniBarClass {
    pub parent_class: gtk::GtkWidgetClass,
    pub _reserved: [gpointer; 8],
}

impl ::std::fmt::Debug for PanelOmniBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelOmniBarClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelPanedClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelPanedClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelPanedClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelPositionClass {
    pub parent_class: gobject::GObjectClass,
}

impl ::std::fmt::Debug for PanelPositionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelPositionClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelSaveDelegateClass {
    pub parent_class: gobject::GObjectClass,
    pub save_async: Option<
        unsafe extern "C" fn(
            *mut PanelSaveDelegate,
            *mut gio::GCancellable,
            gio::GAsyncReadyCallback,
            gpointer,
        ),
    >,
    pub save_finish: Option<
        unsafe extern "C" fn(
            *mut PanelSaveDelegate,
            *mut gio::GAsyncResult,
            *mut *mut glib::GError,
        ) -> gboolean,
    >,
    pub save: Option<unsafe extern "C" fn(*mut PanelSaveDelegate, *mut gio::GTask) -> gboolean>,
    pub discard: Option<unsafe extern "C" fn(*mut PanelSaveDelegate)>,
    pub close: Option<unsafe extern "C" fn(*mut PanelSaveDelegate)>,
    pub _reserved: [gpointer; 8],
}

impl ::std::fmt::Debug for PanelSaveDelegateClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelSaveDelegateClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("save_async", &self.save_async)
            .field("save_finish", &self.save_finish)
            .field("save", &self.save)
            .field("discard", &self.discard)
            .field("close", &self.close)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelSaveDialogClass {
    pub parent_class: adw::AdwMessageDialogClass,
}

impl ::std::fmt::Debug for PanelSaveDialogClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelSaveDialogClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelStatusbarClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelStatusbarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelStatusbarClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelThemeSelectorClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelThemeSelectorClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelThemeSelectorClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelToggleButtonClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelToggleButtonClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelToggleButtonClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelWidgetClass {
    pub parent_instance: gtk::GtkWidgetClass,
    pub get_default_focus: Option<unsafe extern "C" fn(*mut PanelWidget) -> *mut gtk::GtkWidget>,
    pub presented: Option<unsafe extern "C" fn(*mut PanelWidget)>,
    pub _reserved: [gpointer; 8],
}

impl ::std::fmt::Debug for PanelWidgetClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelWidgetClass @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .field("get_default_focus", &self.get_default_focus)
            .field("presented", &self.presented)
            .finish()
    }
}

// Classes
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelDock {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for PanelDock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelDock @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelFrame {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for PanelFrame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrame @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct PanelFrameHeaderBar {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelFrameHeaderBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameHeaderBar @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelFrameSwitcher {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelFrameSwitcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameSwitcher @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelFrameTabBar {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelFrameTabBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameTabBar @ {:p}", self))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelGrid {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for PanelGrid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGrid @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct PanelGridColumn {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelGridColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGridColumn @ {:p}", self))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelOmniBar {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for PanelOmniBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelOmniBar @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct PanelPaned {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelPaned {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelPaned @ {:p}", self)).finish()
    }
}

#[repr(C)]
pub struct PanelPosition {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelPosition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelPosition @ {:p}", self))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelSaveDelegate {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for PanelSaveDelegate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelSaveDelegate @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

#[repr(C)]
pub struct PanelSaveDialog {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelSaveDialog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelSaveDialog @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelStatusbar {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelStatusbar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelStatusbar @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelThemeSelector {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelThemeSelector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelThemeSelector @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelToggleButton {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelToggleButton {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelToggleButton @ {:p}", self))
            .finish()
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PanelWidget {
    pub parent_instance: gtk::GtkWidget,
}

impl ::std::fmt::Debug for PanelWidget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelWidget @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct PanelFrameHeader {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

impl ::std::fmt::Debug for PanelFrameHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PanelFrameHeader @ {:p}", self)
    }
}

#[link(name = "panel-1")]
extern "C" {

    //=========================================================================
    // PanelArea
    //=========================================================================
    pub fn panel_area_get_type() -> GType;

    //=========================================================================
    // PanelWidgetClass
    //=========================================================================
    pub fn panel_widget_class_install_action(
        widget_class: *mut PanelWidgetClass,
        action_name: *const c_char,
        parameter_type: *const c_char,
        activate: gtk::GtkWidgetActionActivateFunc,
    );
    pub fn panel_widget_class_install_property_action(
        widget_class: *mut PanelWidgetClass,
        action_name: *const c_char,
        property_name: *const c_char,
    );

    //=========================================================================
    // PanelDock
    //=========================================================================
    pub fn panel_dock_get_type() -> GType;
    pub fn panel_dock_new() -> *mut gtk::GtkWidget;
    pub fn panel_dock_foreach_frame(
        self_: *mut PanelDock,
        callback: PanelFrameCallback,
        user_data: gpointer,
    );
    pub fn panel_dock_get_can_reveal_area(self_: *mut PanelDock, area: PanelArea) -> gboolean;
    pub fn panel_dock_get_can_reveal_bottom(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_end(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_start(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_top(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_area(self_: *mut PanelDock, area: PanelArea) -> gboolean;
    pub fn panel_dock_get_reveal_bottom(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_end(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_start(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_top(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_remove(self_: *mut PanelDock, child: *mut gtk::GtkWidget);
    pub fn panel_dock_set_bottom_height(self_: *mut PanelDock, height: c_int);
    pub fn panel_dock_set_end_width(self_: *mut PanelDock, width: c_int);
    pub fn panel_dock_set_reveal_area(self_: *mut PanelDock, area: PanelArea, reveal: gboolean);
    pub fn panel_dock_set_reveal_bottom(self_: *mut PanelDock, reveal_bottom: gboolean);
    pub fn panel_dock_set_reveal_end(self_: *mut PanelDock, reveal_end: gboolean);
    pub fn panel_dock_set_reveal_start(self_: *mut PanelDock, reveal_start: gboolean);
    pub fn panel_dock_set_reveal_top(self_: *mut PanelDock, reveal_top: gboolean);
    pub fn panel_dock_set_start_width(self_: *mut PanelDock, width: c_int);
    pub fn panel_dock_set_top_height(self_: *mut PanelDock, height: c_int);

    //=========================================================================
    // PanelFrame
    //=========================================================================
    pub fn panel_frame_get_type() -> GType;
    pub fn panel_frame_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_add(self_: *mut PanelFrame, panel: *mut PanelWidget);
    pub fn panel_frame_add_before(
        self_: *mut PanelFrame,
        panel: *mut PanelWidget,
        sibling: *mut PanelWidget,
    );
    pub fn panel_frame_get_closeable(self_: *mut PanelFrame) -> gboolean;
    pub fn panel_frame_get_empty(self_: *mut PanelFrame) -> gboolean;
    pub fn panel_frame_get_header(self_: *mut PanelFrame) -> *mut PanelFrameHeader;
    pub fn panel_frame_get_n_pages(self_: *mut PanelFrame) -> c_uint;
    pub fn panel_frame_get_page(self_: *mut PanelFrame, n: c_uint) -> *mut PanelWidget;
    pub fn panel_frame_get_pages(self_: *mut PanelFrame) -> *mut gtk::GtkSelectionModel;
    pub fn panel_frame_get_placeholder(self_: *mut PanelFrame) -> *mut gtk::GtkWidget;
    pub fn panel_frame_get_position(self_: *mut PanelFrame) -> *mut PanelPosition;
    pub fn panel_frame_get_requested_size(self_: *mut PanelFrame) -> c_int;
    pub fn panel_frame_get_visible_child(self_: *mut PanelFrame) -> *mut PanelWidget;
    pub fn panel_frame_remove(self_: *mut PanelFrame, panel: *mut PanelWidget);
    pub fn panel_frame_set_header(self_: *mut PanelFrame, header: *mut PanelFrameHeader);
    pub fn panel_frame_set_placeholder(self_: *mut PanelFrame, placeholder: *mut gtk::GtkWidget);
    pub fn panel_frame_set_requested_size(self_: *mut PanelFrame, requested_size: c_int);
    pub fn panel_frame_set_visible_child(self_: *mut PanelFrame, widget: *mut PanelWidget);

    //=========================================================================
    // PanelFrameHeaderBar
    //=========================================================================
    pub fn panel_frame_header_bar_get_type() -> GType;
    pub fn panel_frame_header_bar_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_header_bar_get_menu_popover(
        self_: *mut PanelFrameHeaderBar,
    ) -> *mut gtk::GtkPopoverMenu;
    pub fn panel_frame_header_bar_get_show_icon(self_: *mut PanelFrameHeaderBar) -> gboolean;
    pub fn panel_frame_header_bar_set_show_icon(
        self_: *mut PanelFrameHeaderBar,
        show_icon: gboolean,
    );

    //=========================================================================
    // PanelFrameSwitcher
    //=========================================================================
    pub fn panel_frame_switcher_get_type() -> GType;
    pub fn panel_frame_switcher_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // PanelFrameTabBar
    //=========================================================================
    pub fn panel_frame_tab_bar_get_type() -> GType;
    pub fn panel_frame_tab_bar_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_tab_bar_get_autohide(self_: *mut PanelFrameTabBar) -> gboolean;
    pub fn panel_frame_tab_bar_get_expand_tabs(self_: *mut PanelFrameTabBar) -> gboolean;
    pub fn panel_frame_tab_bar_get_inverted(self_: *mut PanelFrameTabBar) -> gboolean;
    pub fn panel_frame_tab_bar_set_autohide(self_: *mut PanelFrameTabBar, autohide: gboolean);
    pub fn panel_frame_tab_bar_set_expand_tabs(self_: *mut PanelFrameTabBar, expand_tabs: gboolean);
    pub fn panel_frame_tab_bar_set_inverted(self_: *mut PanelFrameTabBar, inverted: gboolean);

    //=========================================================================
    // PanelGrid
    //=========================================================================
    pub fn panel_grid_get_type() -> GType;
    pub fn panel_grid_new() -> *mut gtk::GtkWidget;
    pub fn panel_grid_add(self_: *mut PanelGrid, widget: *mut PanelWidget);
    pub fn panel_grid_agree_to_close_async(
        self_: *mut PanelGrid,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn panel_grid_agree_to_close_finish(
        self_: *mut PanelGrid,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn panel_grid_foreach_frame(
        self_: *mut PanelGrid,
        callback: PanelFrameCallback,
        user_data: gpointer,
    );
    pub fn panel_grid_get_column(self_: *mut PanelGrid, column: c_uint) -> *mut PanelGridColumn;
    pub fn panel_grid_get_most_recent_column(self_: *mut PanelGrid) -> *mut PanelGridColumn;
    pub fn panel_grid_get_most_recent_frame(self_: *mut PanelGrid) -> *mut PanelFrame;
    pub fn panel_grid_get_n_columns(self_: *mut PanelGrid) -> c_uint;
    pub fn panel_grid_insert_column(self_: *mut PanelGrid, position: c_uint);

    //=========================================================================
    // PanelGridColumn
    //=========================================================================
    pub fn panel_grid_column_get_type() -> GType;
    pub fn panel_grid_column_new() -> *mut gtk::GtkWidget;
    pub fn panel_grid_column_foreach_frame(
        self_: *mut PanelGridColumn,
        callback: PanelFrameCallback,
        user_data: gpointer,
    );
    pub fn panel_grid_column_get_empty(self_: *mut PanelGridColumn) -> gboolean;
    pub fn panel_grid_column_get_most_recent_frame(self_: *mut PanelGridColumn) -> *mut PanelFrame;
    pub fn panel_grid_column_get_n_rows(self_: *mut PanelGridColumn) -> c_uint;
    pub fn panel_grid_column_get_row(self_: *mut PanelGridColumn, row: c_uint) -> *mut PanelFrame;

    //=========================================================================
    // PanelOmniBar
    //=========================================================================
    pub fn panel_omni_bar_get_type() -> GType;
    pub fn panel_omni_bar_new() -> *mut gtk::GtkWidget;
    pub fn panel_omni_bar_add_prefix(
        self_: *mut PanelOmniBar,
        priority: c_int,
        widget: *mut gtk::GtkWidget,
    );
    pub fn panel_omni_bar_add_suffix(
        self_: *mut PanelOmniBar,
        priority: c_int,
        widget: *mut gtk::GtkWidget,
    );
    pub fn panel_omni_bar_get_popover(self_: *mut PanelOmniBar) -> *mut gtk::GtkPopover;
    pub fn panel_omni_bar_get_progress(self_: *mut PanelOmniBar) -> c_double;
    pub fn panel_omni_bar_remove(self_: *mut PanelOmniBar, widget: *mut gtk::GtkWidget);
    pub fn panel_omni_bar_set_popover(self_: *mut PanelOmniBar, popover: *mut gtk::GtkPopover);
    pub fn panel_omni_bar_set_progress(self_: *mut PanelOmniBar, progress: c_double);
    pub fn panel_omni_bar_start_pulsing(self_: *mut PanelOmniBar);
    pub fn panel_omni_bar_stop_pulsing(self_: *mut PanelOmniBar);

    //=========================================================================
    // PanelPaned
    //=========================================================================
    pub fn panel_paned_get_type() -> GType;
    pub fn panel_paned_new() -> *mut gtk::GtkWidget;
    pub fn panel_paned_append(self_: *mut PanelPaned, child: *mut gtk::GtkWidget);
    pub fn panel_paned_get_n_children(self_: *mut PanelPaned) -> c_uint;
    pub fn panel_paned_get_nth_child(self_: *mut PanelPaned, nth: c_uint) -> *mut gtk::GtkWidget;
    pub fn panel_paned_insert(self_: *mut PanelPaned, position: c_int, child: *mut gtk::GtkWidget);
    pub fn panel_paned_insert_after(
        self_: *mut PanelPaned,
        child: *mut gtk::GtkWidget,
        sibling: *mut gtk::GtkWidget,
    );
    pub fn panel_paned_prepend(self_: *mut PanelPaned, child: *mut gtk::GtkWidget);
    pub fn panel_paned_remove(self_: *mut PanelPaned, child: *mut gtk::GtkWidget);

    //=========================================================================
    // PanelPosition
    //=========================================================================
    pub fn panel_position_get_type() -> GType;
    pub fn panel_position_new() -> *mut PanelPosition;
    pub fn panel_position_new_from_variant(variant: *mut glib::GVariant) -> *mut PanelPosition;
    pub fn panel_position_equal(a: *mut PanelPosition, b: *mut PanelPosition) -> gboolean;
    pub fn panel_position_get_area(self_: *mut PanelPosition) -> PanelArea;
    pub fn panel_position_get_area_set(self_: *mut PanelPosition) -> gboolean;
    pub fn panel_position_get_column(self_: *mut PanelPosition) -> c_uint;
    pub fn panel_position_get_column_set(self_: *mut PanelPosition) -> gboolean;
    pub fn panel_position_get_depth(self_: *mut PanelPosition) -> c_uint;
    pub fn panel_position_get_depth_set(self_: *mut PanelPosition) -> gboolean;
    pub fn panel_position_get_row(self_: *mut PanelPosition) -> c_uint;
    pub fn panel_position_get_row_set(self_: *mut PanelPosition) -> gboolean;
    pub fn panel_position_is_indeterminate(self_: *mut PanelPosition) -> gboolean;
    pub fn panel_position_set_area(self_: *mut PanelPosition, area: PanelArea);
    pub fn panel_position_set_area_set(self_: *mut PanelPosition, area_set: gboolean);
    pub fn panel_position_set_column(self_: *mut PanelPosition, column: c_uint);
    pub fn panel_position_set_column_set(self_: *mut PanelPosition, column_set: gboolean);
    pub fn panel_position_set_depth(self_: *mut PanelPosition, depth: c_uint);
    pub fn panel_position_set_depth_set(self_: *mut PanelPosition, depth_set: gboolean);
    pub fn panel_position_set_row(self_: *mut PanelPosition, row: c_uint);
    pub fn panel_position_set_row_set(self_: *mut PanelPosition, row_set: gboolean);
    pub fn panel_position_to_variant(self_: *mut PanelPosition) -> *mut glib::GVariant;

    //=========================================================================
    // PanelSaveDelegate
    //=========================================================================
    pub fn panel_save_delegate_get_type() -> GType;
    pub fn panel_save_delegate_new() -> *mut PanelSaveDelegate;
    pub fn panel_save_delegate_close(self_: *mut PanelSaveDelegate);
    pub fn panel_save_delegate_discard(self_: *mut PanelSaveDelegate);
    pub fn panel_save_delegate_get_icon(self_: *mut PanelSaveDelegate) -> *mut gio::GIcon;
    pub fn panel_save_delegate_get_icon_name(self_: *mut PanelSaveDelegate) -> *const c_char;
    pub fn panel_save_delegate_get_is_draft(self_: *mut PanelSaveDelegate) -> gboolean;
    pub fn panel_save_delegate_get_progress(self_: *mut PanelSaveDelegate) -> c_double;
    pub fn panel_save_delegate_get_subtitle(self_: *mut PanelSaveDelegate) -> *const c_char;
    pub fn panel_save_delegate_get_title(self_: *mut PanelSaveDelegate) -> *const c_char;
    pub fn panel_save_delegate_save_async(
        self_: *mut PanelSaveDelegate,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn panel_save_delegate_save_finish(
        self_: *mut PanelSaveDelegate,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn panel_save_delegate_set_icon(self_: *mut PanelSaveDelegate, icon: *mut gio::GIcon);
    pub fn panel_save_delegate_set_icon_name(self_: *mut PanelSaveDelegate, icon: *const c_char);
    pub fn panel_save_delegate_set_is_draft(self_: *mut PanelSaveDelegate, is_draft: gboolean);
    pub fn panel_save_delegate_set_progress(self_: *mut PanelSaveDelegate, progress: c_double);
    pub fn panel_save_delegate_set_subtitle(self_: *mut PanelSaveDelegate, subtitle: *const c_char);
    pub fn panel_save_delegate_set_title(self_: *mut PanelSaveDelegate, title: *const c_char);

    //=========================================================================
    // PanelSaveDialog
    //=========================================================================
    pub fn panel_save_dialog_get_type() -> GType;
    pub fn panel_save_dialog_new() -> *mut gtk::GtkWidget;
    pub fn panel_save_dialog_add_delegate(
        self_: *mut PanelSaveDialog,
        delegate: *mut PanelSaveDelegate,
    );
    pub fn panel_save_dialog_get_close_after_save(self_: *mut PanelSaveDialog) -> gboolean;
    pub fn panel_save_dialog_run_async(
        self_: *mut PanelSaveDialog,
        cancellable: *mut gio::GCancellable,
        callback: gio::GAsyncReadyCallback,
        user_data: gpointer,
    );
    pub fn panel_save_dialog_run_finish(
        self_: *mut PanelSaveDialog,
        result: *mut gio::GAsyncResult,
        error: *mut *mut glib::GError,
    ) -> gboolean;
    pub fn panel_save_dialog_set_close_after_save(
        self_: *mut PanelSaveDialog,
        close_after_save: gboolean,
    );

    //=========================================================================
    // PanelStatusbar
    //=========================================================================
    pub fn panel_statusbar_get_type() -> GType;
    pub fn panel_statusbar_new() -> *mut gtk::GtkWidget;
    pub fn panel_statusbar_add_prefix(
        self_: *mut PanelStatusbar,
        priority: c_int,
        widget: *mut gtk::GtkWidget,
    );
    pub fn panel_statusbar_add_suffix(
        self_: *mut PanelStatusbar,
        priority: c_int,
        widget: *mut gtk::GtkWidget,
    );
    pub fn panel_statusbar_remove(self_: *mut PanelStatusbar, widget: *mut gtk::GtkWidget);

    //=========================================================================
    // PanelThemeSelector
    //=========================================================================
    pub fn panel_theme_selector_get_type() -> GType;
    pub fn panel_theme_selector_new() -> *mut gtk::GtkWidget;
    pub fn panel_theme_selector_get_action_name(self_: *mut PanelThemeSelector) -> *const c_char;
    pub fn panel_theme_selector_set_action_name(
        self_: *mut PanelThemeSelector,
        action_name: *const c_char,
    );

    //=========================================================================
    // PanelToggleButton
    //=========================================================================
    pub fn panel_toggle_button_get_type() -> GType;
    pub fn panel_toggle_button_new(dock: *mut PanelDock, area: PanelArea) -> *mut gtk::GtkWidget;

    //=========================================================================
    // PanelWidget
    //=========================================================================
    pub fn panel_widget_get_type() -> GType;
    pub fn panel_widget_new() -> *mut gtk::GtkWidget;
    pub fn panel_widget_action_set_enabled(
        widget: *mut PanelWidget,
        action_name: *const c_char,
        enabled: gboolean,
    );
    pub fn panel_widget_close(self_: *mut PanelWidget);
    pub fn panel_widget_focus_default(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_force_close(self_: *mut PanelWidget);
    pub fn panel_widget_get_busy(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_can_maximize(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_child(self_: *mut PanelWidget) -> *mut gtk::GtkWidget;
    pub fn panel_widget_get_default_focus(self_: *mut PanelWidget) -> *mut gtk::GtkWidget;
    pub fn panel_widget_get_icon(self_: *mut PanelWidget) -> *mut gio::GIcon;
    pub fn panel_widget_get_icon_name(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_get_id(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_get_kind(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_get_menu_model(self_: *mut PanelWidget) -> *mut gio::GMenuModel;
    pub fn panel_widget_get_modified(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_needs_attention(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_position(self_: *mut PanelWidget) -> *mut PanelPosition;
    pub fn panel_widget_get_reorderable(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_save_delegate(self_: *mut PanelWidget) -> *mut PanelSaveDelegate;
    pub fn panel_widget_get_title(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_insert_action_group(
        self_: *mut PanelWidget,
        prefix: *const c_char,
        group: *mut gio::GActionGroup,
    );
    pub fn panel_widget_mark_busy(self_: *mut PanelWidget);
    pub fn panel_widget_maximize(self_: *mut PanelWidget);
    pub fn panel_widget_raise(self_: *mut PanelWidget);
    pub fn panel_widget_set_can_maximize(self_: *mut PanelWidget, can_maximize: gboolean);
    pub fn panel_widget_set_child(self_: *mut PanelWidget, child: *mut gtk::GtkWidget);
    pub fn panel_widget_set_icon(self_: *mut PanelWidget, icon: *mut gio::GIcon);
    pub fn panel_widget_set_icon_name(self_: *mut PanelWidget, icon_name: *const c_char);
    pub fn panel_widget_set_id(self_: *mut PanelWidget, id: *const c_char);
    pub fn panel_widget_set_kind(self_: *mut PanelWidget, kind: *const c_char);
    pub fn panel_widget_set_menu_model(self_: *mut PanelWidget, menu_model: *mut gio::GMenuModel);
    pub fn panel_widget_set_modified(self_: *mut PanelWidget, modified: gboolean);
    pub fn panel_widget_set_needs_attention(self_: *mut PanelWidget, needs_attention: gboolean);
    pub fn panel_widget_set_reorderable(self_: *mut PanelWidget, reorderable: gboolean);
    pub fn panel_widget_set_save_delegate(
        self_: *mut PanelWidget,
        save_delegate: *mut PanelSaveDelegate,
    );
    pub fn panel_widget_set_title(self_: *mut PanelWidget, title: *const c_char);
    pub fn panel_widget_unmark_busy(self_: *mut PanelWidget);
    pub fn panel_widget_unmaximize(self_: *mut PanelWidget);

    //=========================================================================
    // PanelFrameHeader
    //=========================================================================
    pub fn panel_frame_header_get_type() -> GType;
    pub fn panel_frame_header_add_prefix(
        self_: *mut PanelFrameHeader,
        priority: c_int,
        child: *mut gtk::GtkWidget,
    );
    pub fn panel_frame_header_add_suffix(
        self_: *mut PanelFrameHeader,
        priority: c_int,
        child: *mut gtk::GtkWidget,
    );
    pub fn panel_frame_header_can_drop(
        self_: *mut PanelFrameHeader,
        widget: *mut PanelWidget,
    ) -> gboolean;
    pub fn panel_frame_header_get_frame(self_: *mut PanelFrameHeader) -> *mut PanelFrame;
    pub fn panel_frame_header_page_changed(self_: *mut PanelFrameHeader, widget: *mut PanelWidget);
    pub fn panel_frame_header_set_frame(self_: *mut PanelFrameHeader, frame: *mut PanelFrame);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn panel_finalize();
    pub fn panel_get_resource() -> *mut gio::GResource;
    pub fn panel_init();

}
