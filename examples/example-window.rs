/* example-window.c
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

use gtk::{gio, glib, subclass::prelude::*, CompositeTemplate};
use libadwaita::{self as adw, prelude::*, subclass::prelude::*};
use libpanel::{self as panel, prelude::*};
use once_cell::sync::Lazy as SyncLazy;
use std::{cell::Cell, ops::Deref, str::FromStr};

glib::wrapper! {
    pub struct ExampleWindow(ObjectSubclass<ExampleWindowPrivate>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "example-window.ui")]
pub struct ExampleWindowPrivate {
    #[template_child]
    dock: TemplateChild<panel::Dock>,
    #[template_child]
    grid: TemplateChild<panel::Grid>,
    #[template_child]
    page_menu: TemplateChild<gio::MenuModel>,
    #[template_child]
    frame_header_bar: TemplateChild<gtk::ToggleButton>,

    document_count: Cell<u32>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleWindowPrivate {
    const NAME: &'static str = "ExampleWindow";
    type Type = ExampleWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);

        klass.install_action("document.new", None, |obj, _name, _args| {
            obj.add_document();
        });
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

impl ObjectImpl for ExampleWindowPrivate {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        self.grid.connect_create_frame(
            glib::clone!(@weak obj => @default-return panel::Frame::new(), move |_| {
                obj.create_frame()
            }),
        );
        obj.add_action(&gio::PropertyAction::new(
            "reveal-start",
            self.dock.deref(),
            "reveal-start",
        ));
        obj.add_action(&gio::PropertyAction::new(
            "reveal-bottom",
            self.dock.deref(),
            "reveal-bottom",
        ));
        obj.add_action(&gio::PropertyAction::new(
            "reveal-end",
            self.dock.deref(),
            "reveal-end",
        ));
        self.grid.column(0).row(0);
    }
}
impl WidgetImpl for ExampleWindowPrivate {}
impl WindowImpl for ExampleWindowPrivate {}
impl ApplicationWindowImpl for ExampleWindowPrivate {}
impl AdwApplicationWindowImpl for ExampleWindowPrivate {}

impl ExampleWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create ExampleWindow")
    }
    async fn save(
        widget: panel::Widget,
        delegate: panel::SaveDelegate,
    ) -> Result<(), glib::Error> {
        println!("Actually save the file");

        widget.set_modified(false);
        delegate.set_progress(1.0);

        Ok(())
    }
    fn apply_theme_color(widget: &panel::Widget) {
        static WHITE: SyncLazy<gdk::RGBA> = SyncLazy::new(|| gdk::RGBA::from_str("#fff").unwrap());
        static GREY: SyncLazy<gdk::RGBA> =
            SyncLazy::new(|| gdk::RGBA::from_str("#1e1e1e").unwrap());
        static BLACK: SyncLazy<gdk::RGBA> = SyncLazy::new(|| gdk::RGBA::from_str("#000").unwrap());

        if adw::StyleManager::default().is_dark() {
            widget.set_background_rgba(Some(&GREY));
            widget.set_foreground_rgba(Some(&WHITE));
        } else {
            widget.set_background_rgba(Some(&WHITE));
            widget.set_foreground_rgba(Some(&BLACK));
        }
    }
    fn add_document(&self) {
        let imp = ExampleWindowPrivate::from_instance(self);
        let count = imp.document_count.get() + 1;
        imp.document_count.set(count);
        let title = format!("Untitled Document {}", count);
        let buffer = gtk::TextBuffer::builder().text(&title).build();
        let text_view = gtk::TextView::builder()
            .left_margin(6)
            .top_margin(6)
            .buffer(&buffer)
            .build();
        let save_delegate = panel::SaveDelegate::builder()
            .title(&title)
            .subtitle("~/Documents")
            .build();

        let widget = panel::Widget::builder()
            .title(&title)
            .kind(&panel::WIDGET_KIND_DOCUMENT)
            .icon_name("text-x-generic-symbolic")
            .menu_model(imp.page_menu.deref())
            .can_maximize(true)
            .save_delegate(&save_delegate)
            .modified(true)
            .child(&gtk::ScrolledWindow::builder().child(&text_view).build())
            .build();

        adw::StyleManager::default().connect_notify_local(
            None,
            glib::clone!(@weak widget => move |_, _| {
                Self::apply_theme_color(&widget);
            }),
        );
        Self::apply_theme_color(&widget);

        widget.connect_get_default_focus(
            glib::clone!(@weak text_view => @default-return None, move |_| {
                Some(text_view.upcast())
            }),
        );
        save_delegate.connect_save(
            glib::clone!(@weak widget => @default-panic, move |delegate| {
                Self::save(widget, delegate.clone())
            })
        );

        imp.grid.add(&widget);
        widget.raise();
        widget.focus_default();
    }
    fn create_frame(&self) -> panel::Frame {
        let imp = ExampleWindowPrivate::from_instance(self);
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

fn startup(_app: &gtk::Application) {
    panel::init();
}

fn activate(app: &gtk::Application) {
    let window = ExampleWindow::new(app);
    window.present();
}

fn main() {
    let app = gtk::Application::new(
        Some("org.gnome.gitlab.jf.libpanel-rs.example"),
        gtk::gio::ApplicationFlags::FLAGS_NONE,
    );
    app.connect_startup(startup);
    app.connect_activate(activate);
    std::process::exit(app.run());
}
