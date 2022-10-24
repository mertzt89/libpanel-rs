/* Based on example-page.c
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

use gtk::{glib, prelude::*};
use libpanel::{self as panel, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use std::{cell::RefCell, time::Duration};

glib::wrapper! {
    pub struct ExamplePage(ObjectSubclass<ExamplePagePrivate>)
        @extends gtk::Widget, panel::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[derive(Debug, Default)]
pub struct ExamplePagePrivate {
    text_view: RefCell<Option<gtk::TextView>>,
    _im_context: RefCell<Option<gtk::IMContext>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExamplePagePrivate {
    const NAME: &'static str = "ExamplePage";
    type Type = ExamplePage;
    type ParentType = panel::Widget;
}

fn tick(delegate: &panel::SaveDelegate, task: &gio::Task<bool>) -> Result<bool, glib::Error> {
    if let Some(cancellable) = task.cancellable() {
        cancellable.set_error_if_cancelled()?;
    }
    if task.had_error() {
        return Ok(true);
    }
    let progress = (delegate.progress() + 0.005).clamp(0.0, 1.0);
    delegate.set_progress(progress);
    Ok(progress >= 1.0)
}

impl ObjectImpl for ExamplePagePrivate {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecString::builder("command-text")
                    .read_only()
                    .build(),
                glib::ParamSpecString::builder("command-bar-text")
                    .read_only()
                    .build(),
            ]
        });
        PROPERTIES.as_ref()
    }
    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "command-text" => "".to_value(),
            "command-bar-text" => "".to_value(),
            _ => unimplemented!(),
        }
    }
    fn constructed(&self) {
        self.parent_constructed();
        let scroller = gtk::ScrolledWindow::new();
        let obj = self.obj();
        obj.set_child(Some(&scroller));

        self.text_view.replace(Some(gtk::TextView::new()));

        let text_view = self.text_view.borrow();
        let text_view = text_view.as_ref().unwrap();
        text_view.set_monospace(true);
        text_view.set_left_margin(6);
        text_view.set_top_margin(6);
        scroller.set_child(Some(text_view));

        let save_delegate = panel::SaveDelegate::builder()
            .subtitle("~/Documents")
            .build();
        save_delegate.connect_save(|delegate, task| {
            glib::clone!(@strong delegate, @strong task => async move {
                loop {
                    glib::timeout_future(Duration::from_millis(30)).await;
                    if tick(&delegate, &task)? {
                        break;
                    }
                }
                Ok(())
            })
        });
        save_delegate.connect_discard(glib::clone!(@weak obj => move |_| obj.force_close()));
        save_delegate.connect_close(glib::clone!(@weak obj => move |_| obj.force_close()));
        obj.bind_property("title", &save_delegate, "title")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        obj.bind_property("icon", &save_delegate, "icon")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        obj.set_save_delegate(Some(&save_delegate));
    }
}
impl WidgetImpl for ExamplePagePrivate {}
impl PanelWidgetImpl for ExamplePagePrivate {
    fn default_focus(&self, _widget: &Self::Type) -> Option<gtk::Widget> {
        Some(self.text_view.borrow().as_ref().unwrap().clone().upcast())
    }
}

impl ExamplePage {
    pub fn new() -> Self {
        glib::Object::new(&[])
    }
}
