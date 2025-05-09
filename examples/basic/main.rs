/* Based on main.c
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

mod page;
mod window;
use gtk::prelude::*;

fn startup(_app: &gtk::Application) {
    libpanel::init();
}

fn activate(app: &gtk::Application) {
    let window = window::ExampleWindow::new(app);
    window.present();
}

fn main() -> glib::ExitCode {
    let app = gtk::Application::new(
        Some("org.gnome.gitlab.world.rust.libpanel-rs.Example"),
        gtk::gio::ApplicationFlags::FLAGS_NONE,
    );
    app.connect_startup(startup);
    app.connect_activate(activate);
    app.run()
}
