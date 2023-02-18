/* Based on example-window.c
 *
 * Copyright 2021 Christian Hergert <chergert@redhat.com>
 *
 * This file is free software; you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 3 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: LGPL-3.0-or-later
 */

use super::page::ExamplePage;
use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib, CompositeTemplate};
use libpanel::{self as panel, prelude::*};
use std::cell::Cell;

glib::wrapper! {
    pub struct ExampleWindow(ObjectSubclass<ExampleWindowPrivate>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "window.ui")]
pub struct ExampleWindowPrivate {
    #[template_child]
    dock: TemplateChild<panel::Dock>,
    #[template_child]
    grid: TemplateChild<panel::Grid>,
    #[template_child]
    page_menu: TemplateChild<gio::MenuModel>,
    #[template_child]
    frame_header_bar: TemplateChild<gtk::ToggleButton>,
    #[template_child]
    language: TemplateChild<gtk::DropDown>,
    #[template_child]
    primary_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    theme_selector: TemplateChild<panel::ThemeSelector>,
    #[template_child]
    command: TemplateChild<gtk::Label>,
    #[template_child]
    command_bar: TemplateChild<gtk::Label>,

    document_count: Cell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleWindowPrivate {
    const NAME: &'static str = "ExampleWindow";
    type Type = ExampleWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
        Self::Type::bind_template_callbacks(klass);

        klass.install_action("document.new", None, |obj, _name, _args| {
            obj.add_document();
        });
        klass.install_action("project.build", None, |_obj, _name, _args| {});
        use gdk::ModifierType as mods;
        klass.add_binding_action(gdk::Key::n, mods::CONTROL_MASK, "document.new", None);
        klass.add_binding_action(gdk::Key::F9, mods::empty(), "win.reveal-start", None);
        klass.add_binding_action(gdk::Key::F9, mods::CONTROL_MASK, "win.reveal-bottom", None);
        klass.add_binding_action(gdk::Key::F9, mods::SHIFT_MASK, "win.reveal-end", None);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl ExampleWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }
    fn notify_theme_cb(&self, style_manager: &adw::StyleManager) {
        let name = match style_manager.color_scheme() {
            adw::ColorScheme::PreferDark | adw::ColorScheme::ForceDark => "dark",
            adw::ColorScheme::PreferLight | adw::ColorScheme::ForceLight => "light",
            _ => {
                if style_manager.system_supports_color_schemes() {
                    "light"
                } else {
                    "default"
                }
            }
        };
        self.lookup_action("theme")
            .unwrap()
            .downcast::<gio::SimpleAction>()
            .unwrap()
            .set_state(&name.to_variant());
    }
    fn add_document(&self) {
        let imp = self.imp();
        let count = imp.document_count.get() + 1;
        imp.document_count.set(count);
        let title = format!("Untitled Document {}", count);

        let widget = ExamplePage::new();
        widget.set_title(Some(&title));
        widget.set_kind(Some(&panel::WIDGET_KIND_DOCUMENT));
        widget.set_icon_name(Some("text-x-generic-symbolic"));
        widget.set_menu_model(Some(&*imp.page_menu));
        widget.set_can_maximize(true);
        widget.set_modified(true);

        imp.grid.add(&widget);
        widget.raise();
        widget.focus_default();

        widget
            .bind_property("command-bar-text", &*imp.command_bar, "label")
            .build();
        widget
            .bind_property("command-text", &*imp.command, "visible")
            .transform_to(|_, v: Option<&str>| v.map(|s| (!s.is_empty())))
            .build();
        widget
            .bind_property("command-text", &*imp.command, "label")
            .build();
    }
    #[template_callback]
    fn create_frame_cb(_: &panel::Grid, window: &Self) -> panel::Frame {
        let imp = window.imp();
        let frame = panel::Frame::new();

        let status = adw::StatusPage::builder()
            .title("Open a File or Terminal")
            .icon_name("document-new-symbolic")
            .description("Use the page switcher above or use one of the following:")
            .build();
        let shortcuts = gtk::Grid::builder()
            .row_spacing(6)
            .column_spacing(32)
            .halign(gtk::Align::Center)
            .build();
        shortcuts.attach(&gtk::Label::new(Some("New Document")), 0, 0, 1, 1);
        shortcuts.attach(&gtk::Label::new(Some("Ctrl+N")), 1, 0, 1, 1);
        shortcuts.attach(&gtk::Label::new(Some("Close Document")), 0, 1, 1, 1);
        shortcuts.attach(&gtk::Label::new(Some("Ctrl+W")), 1, 1, 1, 1);
        let mut child = shortcuts.first_child();
        while let Some(c) = child {
            c.set_halign(gtk::Align::Start);
            child = c.next_sibling();
        }
        status.set_child(Some(&shortcuts));
        frame.set_placeholder(Some(&status));
        let header = if imp.frame_header_bar.is_active() {
            panel::FrameHeaderBar::builder()
                .show_icon(true)
                .build()
                .upcast()
        } else {
            panel::FrameTabBar::new().upcast::<panel::FrameHeader>()
        };

        frame.set_header(Some(&header));
        let child = gtk::Button::builder()
            .width_request(40)
            .focus_on_click(false)
            .icon_name("go-previous-symbolic")
            .build();
        child.add_css_class("flat");
        header.add_prefix(-100, &child);

        let child = gtk::Button::builder()
            .width_request(40)
            .focus_on_click(false)
            .icon_name("go-next-symbolic")
            .build();
        child.add_css_class("flat");
        header.add_prefix(-50, &child);

        frame
    }
}

impl ObjectImpl for ExampleWindowPrivate {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        let action = gio::SimpleAction::new_stateful(
            "theme",
            Some(glib::VariantTy::STRING),
            &"default".to_variant(),
        );
        action.connect_change_state(|_, param| {
            let manager = adw::StyleManager::default();
            match param.unwrap().str().unwrap() {
                "default" => manager.set_color_scheme(adw::ColorScheme::Default),
                "light" => manager.set_color_scheme(adw::ColorScheme::ForceLight),
                "dark" => manager.set_color_scheme(adw::ColorScheme::ForceDark),
                _ => {}
            }
        });
        obj.add_action(&action);
        let action = gio::SimpleAction::new_stateful(
            "runner",
            Some(glib::VariantTy::STRING),
            &"".to_variant(),
        );
        action.connect_change_state(|action, param| {
            action.set_state(param.unwrap());
        });
        obj.add_action(&action);
        let action = gio::SimpleAction::new_stateful("high-contrast", None, &false.to_variant());
        action.connect_change_state(|a, _| {
            a.set_state(&(!a.state().and_then(|v| v.get::<bool>()).unwrap_or(false)).to_variant());
        });
        obj.add_action(&action);
        let action = gio::SimpleAction::new_stateful("right-to-left", None, &false.to_variant());
        action.connect_change_state(|a, _| {
            a.set_state(&(!a.state().and_then(|v| v.get::<bool>()).unwrap_or(false)).to_variant());
        });
        obj.add_action(&action);
        let style_manager = adw::StyleManager::default();
        style_manager.connect_notify_local(
            None,
            glib::clone!(@weak obj => move |m, _| {
                obj.notify_theme_cb(m);
            }),
        );
        obj.notify_theme_cb(&style_manager);
        obj.add_action(&gio::PropertyAction::new(
            "reveal-start",
            &*self.dock,
            "reveal-start",
        ));
        obj.add_action(&gio::PropertyAction::new(
            "reveal-bottom",
            &*self.dock,
            "reveal-bottom",
        ));
        obj.add_action(&gio::PropertyAction::new(
            "reveal-end",
            &*self.dock,
            "reveal-end",
        ));
        let mut child = self.language.first_child();
        while let Some(c) = child {
            if let Some(popover) = c.downcast_ref::<gtk::Popover>() {
                popover.set_position(gtk::PositionType::Top);
            }
            child = c.next_sibling();
        }
        self.primary_button
            .popover()
            .unwrap()
            .downcast::<gtk::PopoverMenu>()
            .unwrap()
            .add_child(&*self.theme_selector, "theme");
        self.grid.column(0).row(0);
    }
}
impl WidgetImpl for ExampleWindowPrivate {}
impl WindowImpl for ExampleWindowPrivate {}
impl ApplicationWindowImpl for ExampleWindowPrivate {}
impl AdwApplicationWindowImpl for ExampleWindowPrivate {}
