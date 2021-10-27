// Generated by gir (https://github.com/gtk-rs/gir @ a97e6087cf6b)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ b827978e7d18)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
    clippy::approx_constant,
    clippy::type_complexity,
    clippy::unreadable_literal,
    clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use adw_sys as adw;
use gdk_sys as gdk;
use gio_sys as gio;
use glib_sys as glib;
use gobject_sys as gobject;
use gtk_sys as gtk;

#[allow(unused_imports)]
use libc::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
    intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE,
};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type PanelDockPosition = c_int;
pub const PANEL_DOCK_POSITION_START: PanelDockPosition = 0;
pub const PANEL_DOCK_POSITION_END: PanelDockPosition = 1;
pub const PANEL_DOCK_POSITION_TOP: PanelDockPosition = 2;
pub const PANEL_DOCK_POSITION_BOTTOM: PanelDockPosition = 3;
pub const PANEL_DOCK_POSITION_CENTER: PanelDockPosition = 4;

// Constants
pub const PANEL_MAJOR_VERSION: c_int = 1;
pub const PANEL_MICRO_VERSION: c_int = 0;
pub const PANEL_MINOR_VERSION: c_int = 0;
pub const PANEL_VERSION_S: *const c_char = b"1.0.0-alpha0\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_ANY: *const c_char = b"*\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_DOCUMENT: *const c_char = b"document\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_UNKNOWN: *const c_char = b"unknown\0" as *const u8 as *const c_char;
pub const PANEL_WIDGET_KIND_UTILITY: *const c_char = b"utility\0" as *const u8 as *const c_char;

// Callbacks
pub type PanelFrameCallback = Option<unsafe extern "C" fn(*mut PanelFrame, gpointer)>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PanelDockSwitcherClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelDockSwitcherClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelDockSwitcherClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PanelFrameHeaderInterface {
    pub parent_iface: gobject::GTypeInterface,
    pub page_changed: Option<unsafe extern "C" fn(*mut PanelFrameHeader, *mut PanelWidget)>,
    pub can_drop: Option<unsafe extern "C" fn(*mut PanelFrameHeader, *mut PanelWidget) -> gboolean>,
    pub pack_start: Option<unsafe extern "C" fn(*mut PanelFrameHeader, c_int, *mut gtk::GtkWidget)>,
    pub pack_end: Option<unsafe extern "C" fn(*mut PanelFrameHeader, c_int, *mut gtk::GtkWidget)>,
}

impl ::std::fmt::Debug for PanelFrameHeaderInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameHeaderInterface @ {:p}", self))
            .field("parent_iface", &self.parent_iface)
            .field("page_changed", &self.page_changed)
            .field("can_drop", &self.can_drop)
            .field("pack_start", &self.pack_start)
            .field("pack_end", &self.pack_end)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PanelGridClass {
    pub parent_class: gtk::GtkWidgetClass,
}

impl ::std::fmt::Debug for PanelGridClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGridClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
    pub _reserved: [gpointer; 8],
}

impl ::std::fmt::Debug for PanelSaveDelegateClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelSaveDelegateClass @ {:p}", self))
            .field("parent_class", &self.parent_class)
            .field("save_async", &self.save_async)
            .field("save_finish", &self.save_finish)
            .field("save", &self.save)
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PanelWidgetClass {
    pub parent_instance: gtk::GtkWidgetClass,
    pub get_default_focus: Option<unsafe extern "C" fn(*mut PanelWidget) -> *mut gtk::GtkWidget>,
    pub _reserved: [gpointer; 16],
}

impl ::std::fmt::Debug for PanelWidgetClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelWidgetClass @ {:p}", self))
            .field("parent_instance", &self.parent_instance)
            .field("get_default_focus", &self.get_default_focus)
            .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
pub struct PanelDockSwitcher(c_void);

impl ::std::fmt::Debug for PanelDockSwitcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelDockSwitcher @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct PanelFrameHeaderBar(c_void);

impl ::std::fmt::Debug for PanelFrameHeaderBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameHeaderBar @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelFrameSwitcher(c_void);

impl ::std::fmt::Debug for PanelFrameSwitcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameSwitcher @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelFrameTabBar(c_void);

impl ::std::fmt::Debug for PanelFrameTabBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelFrameTabBar @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
pub struct PanelGrid(c_void);

impl ::std::fmt::Debug for PanelGrid {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGrid @ {:p}", self)).finish()
    }
}

#[repr(C)]
pub struct PanelGridColumn(c_void);

impl ::std::fmt::Debug for PanelGridColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelGridColumn @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct PanelStatusbar(c_void);

impl ::std::fmt::Debug for PanelStatusbar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PanelStatusbar @ {:p}", self))
            .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct PanelFrameHeader(c_void);

impl ::std::fmt::Debug for PanelFrameHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "PanelFrameHeader @ {:p}", self)
    }
}

#[link(name = "panel-1")]
extern "C" {

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
    pub fn panel_dock_get_can_reveal_bottom(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_end(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_start(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_can_reveal_top(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_bottom(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_end(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_start(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_get_reveal_top(self_: *mut PanelDock) -> gboolean;
    pub fn panel_dock_set_bottom_height(self_: *mut PanelDock, height: c_int);
    pub fn panel_dock_set_end_width(self_: *mut PanelDock, width: c_int);
    pub fn panel_dock_set_reveal_bottom(self_: *mut PanelDock, reveal_bottom: gboolean);
    pub fn panel_dock_set_reveal_end(self_: *mut PanelDock, reveal_end: gboolean);
    pub fn panel_dock_set_reveal_start(self_: *mut PanelDock, reveal_start: gboolean);
    pub fn panel_dock_set_reveal_top(self_: *mut PanelDock, reveal_top: gboolean);
    pub fn panel_dock_set_start_width(self_: *mut PanelDock, width: c_int);
    pub fn panel_dock_set_top_height(self_: *mut PanelDock, height: c_int);

    //=========================================================================
    // PanelDockSwitcher
    //=========================================================================
    pub fn panel_dock_switcher_get_type() -> GType;
    pub fn panel_dock_switcher_new() -> *mut gtk::GtkWidget;
    pub fn panel_dock_switcher_get_dock(self_: *mut PanelDockSwitcher) -> *mut PanelDock;
    pub fn panel_dock_switcher_set_dock(self_: *mut PanelDockSwitcher, dock: *mut PanelDock);

    //=========================================================================
    // PanelFrame
    //=========================================================================
    pub fn panel_frame_get_type() -> GType;
    pub fn panel_frame_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_add(self_: *mut PanelFrame, panel: *mut PanelWidget);
    pub fn panel_frame_get_empty(self_: *mut PanelFrame) -> gboolean;
    pub fn panel_frame_get_header(self_: *mut PanelFrame) -> *mut PanelFrameHeader;
    pub fn panel_frame_get_n_pages(self_: *mut PanelFrame) -> c_uint;
    pub fn panel_frame_get_page(self_: *mut PanelFrame, n: c_uint) -> *mut PanelWidget;
    pub fn panel_frame_get_pages(self_: *mut PanelFrame) -> *mut gtk::GtkSelectionModel;
    pub fn panel_frame_get_placeholder(self_: *mut PanelFrame) -> *mut gtk::GtkWidget;
    pub fn panel_frame_get_visible_child(self_: *mut PanelFrame) -> *mut PanelWidget;
    pub fn panel_frame_remove(self_: *mut PanelFrame, panel: *mut PanelWidget);
    pub fn panel_frame_set_header(self_: *mut PanelFrame, header: *mut PanelFrameHeader);
    pub fn panel_frame_set_placeholder(self_: *mut PanelFrame, placeholder: *mut gtk::GtkWidget);
    pub fn panel_frame_set_visible_child(self_: *mut PanelFrame, widget: *mut PanelWidget);

    //=========================================================================
    // PanelFrameHeaderBar
    //=========================================================================
    pub fn panel_frame_header_bar_get_type() -> GType;
    pub fn panel_frame_header_bar_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_header_bar_get_background_rgba(
        self_: *mut PanelFrameHeaderBar,
    ) -> *const gdk::GdkRGBA;
    pub fn panel_frame_header_bar_get_foreground_rgba(
        self_: *mut PanelFrameHeaderBar,
    ) -> *const gdk::GdkRGBA;
    pub fn panel_frame_header_bar_get_menu_model(
        self_: *mut PanelFrameHeaderBar,
    ) -> *mut gio::GMenuModel;
    pub fn panel_frame_header_bar_get_menu_popover(
        self_: *mut PanelFrameHeaderBar,
    ) -> *mut gtk::GtkPopoverMenu;
    pub fn panel_frame_header_bar_get_show_icon(self_: *mut PanelFrameHeaderBar) -> gboolean;
    pub fn panel_frame_header_bar_set_background_rgba(
        self_: *mut PanelFrameHeaderBar,
        background_rgba: *const gdk::GdkRGBA,
    );
    pub fn panel_frame_header_bar_set_foreground_rgba(
        self_: *mut PanelFrameHeaderBar,
        foreground_rgba: *const gdk::GdkRGBA,
    );
    pub fn panel_frame_header_bar_set_menu_model(
        self_: *mut PanelFrameHeaderBar,
        model: *mut gio::GMenuModel,
    );
    pub fn panel_frame_header_bar_set_show_icon(
        self_: *mut PanelFrameHeaderBar,
        show_icon: gboolean,
    );

    //=========================================================================
    // PanelFrameSwitcher
    //=========================================================================
    pub fn panel_frame_switcher_get_type() -> GType;
    pub fn panel_frame_switcher_new() -> *mut gtk::GtkWidget;
    pub fn panel_frame_switcher_get_background_rgba(
        self_: *mut PanelFrameSwitcher,
    ) -> *const gdk::GdkRGBA;
    pub fn panel_frame_switcher_get_foreground_rgba(
        self_: *mut PanelFrameSwitcher,
    ) -> *const gdk::GdkRGBA;
    pub fn panel_frame_switcher_set_background_rgba(
        self_: *mut PanelFrameSwitcher,
        background_rgba: *const gdk::GdkRGBA,
    );
    pub fn panel_frame_switcher_set_foreground_rgba(
        self_: *mut PanelFrameSwitcher,
        foreground_rgba: *const gdk::GdkRGBA,
    );

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
    pub fn panel_grid_get_column(self_: *mut PanelGrid, column: c_uint) -> *mut PanelGridColumn;
    pub fn panel_grid_get_most_recent_column(self_: *mut PanelGrid) -> *mut PanelGridColumn;
    pub fn panel_grid_get_most_recent_frame(self_: *mut PanelGrid) -> *mut PanelFrame;
    pub fn panel_grid_get_n_columns(self_: *mut PanelGrid) -> c_uint;

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
    // PanelSaveDelegate
    //=========================================================================
    pub fn panel_save_delegate_get_type() -> GType;
    pub fn panel_save_delegate_new() -> *mut PanelSaveDelegate;
    pub fn panel_save_delegate_get_icon(self_: *mut PanelSaveDelegate) -> *mut gio::GIcon;
    pub fn panel_save_delegate_get_icon_name(self_: *mut PanelSaveDelegate) -> *const c_char;
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
    pub fn panel_save_delegate_set_progress(self_: *mut PanelSaveDelegate, progress: c_double);
    pub fn panel_save_delegate_set_subtitle(self_: *mut PanelSaveDelegate, subtitle: *const c_char);
    pub fn panel_save_delegate_set_title(self_: *mut PanelSaveDelegate, title: *const c_char);

    //=========================================================================
    // PanelStatusbar
    //=========================================================================
    pub fn panel_statusbar_get_type() -> GType;
    pub fn panel_statusbar_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // PanelWidget
    //=========================================================================
    pub fn panel_widget_get_type() -> GType;
    pub fn panel_widget_new() -> *mut gtk::GtkWidget;
    pub fn panel_widget_focus_default(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_background_rgba(self_: *mut PanelWidget) -> *const gdk::GdkRGBA;
    pub fn panel_widget_get_busy(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_can_maximize(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_child(self_: *mut PanelWidget) -> *mut gtk::GtkWidget;
    pub fn panel_widget_get_default_focus(self_: *mut PanelWidget) -> *mut gtk::GtkWidget;
    pub fn panel_widget_get_foreground_rgba(self_: *mut PanelWidget) -> *const gdk::GdkRGBA;
    pub fn panel_widget_get_icon(self_: *mut PanelWidget) -> *mut gio::GIcon;
    pub fn panel_widget_get_icon_name(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_get_kind(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_get_menu_model(self_: *mut PanelWidget) -> *mut gio::GMenuModel;
    pub fn panel_widget_get_modified(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_needs_attention(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_reorderable(self_: *mut PanelWidget) -> gboolean;
    pub fn panel_widget_get_save_delegate(self_: *mut PanelWidget) -> *mut PanelSaveDelegate;
    pub fn panel_widget_get_title(self_: *mut PanelWidget) -> *const c_char;
    pub fn panel_widget_mark_busy(self_: *mut PanelWidget);
    pub fn panel_widget_maximize(self_: *mut PanelWidget);
    pub fn panel_widget_raise(self_: *mut PanelWidget);
    pub fn panel_widget_set_background_rgba(
        self_: *mut PanelWidget,
        background_rgba: *const gdk::GdkRGBA,
    );
    pub fn panel_widget_set_can_maximize(self_: *mut PanelWidget, can_maximize: gboolean);
    pub fn panel_widget_set_child(self_: *mut PanelWidget, child: *mut gtk::GtkWidget);
    pub fn panel_widget_set_foreground_rgba(
        self_: *mut PanelWidget,
        foreground_rgba: *const gdk::GdkRGBA,
    );
    pub fn panel_widget_set_icon(self_: *mut PanelWidget, icon: *mut gio::GIcon);
    pub fn panel_widget_set_icon_name(self_: *mut PanelWidget, icon_name: *const c_char);
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
    pub fn panel_frame_header_can_drop(
        self_: *mut PanelFrameHeader,
        widget: *mut PanelWidget,
    ) -> gboolean;
    pub fn panel_frame_header_get_frame(self_: *mut PanelFrameHeader) -> *mut PanelFrame;
    pub fn panel_frame_header_pack_end(
        self_: *mut PanelFrameHeader,
        priority: c_int,
        child: *mut gtk::GtkWidget,
    );
    pub fn panel_frame_header_pack_start(
        self_: *mut PanelFrameHeader,
        priority: c_int,
        child: *mut gtk::GtkWidget,
    );
    pub fn panel_frame_header_page_changed(self_: *mut PanelFrameHeader, widget: *mut PanelWidget);
    pub fn panel_frame_header_set_frame(self_: *mut PanelFrameHeader, frame: *mut PanelFrame);

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn panel_finalize();
    pub fn panel_get_resource() -> *mut gio::GResource;
    pub fn panel_init();

}
