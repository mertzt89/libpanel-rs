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

use gtk::glib;
use libpanel::{self as panel, prelude::*, subclass::prelude::*};
use std::cell::RefCell;

glib::wrapper! {
    pub struct ExamplePage(ObjectSubclass<ExamplePagePrivate>)
        @extends gtk::Widget, panel::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[derive(Debug, Default)]
pub struct ExamplePagePrivate {
    text_view: RefCell<Option<gtk::TextView>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExamplePagePrivate {
    const NAME: &'static str = "ExamplePage";
    type Type = ExamplePage;
    type ParentType = panel::Widget;
}

impl ObjectImpl for ExamplePagePrivate {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let scroller = gtk::ScrolledWindow::new();
        obj.set_child(Some(&scroller));
        let text_view = gtk::TextView::builder()
            .monospace(true)
            .left_margin(6)
            .top_margin(6)
            .build();
        scroller.set_child(Some(&text_view));
        self.text_view.replace(Some(text_view));
    }
}
impl WidgetImpl for ExamplePagePrivate {}
impl PanelWidgetImpl for ExamplePagePrivate {
    fn get_default_focus(&self, _widget: &Self::Type) -> Option<gtk::Widget> {
        Some(self.text_view.borrow().as_ref().unwrap().clone().upcast())
    }
}

impl ExamplePage {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ExamplePage")
    }
}
