#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
#[doc(hidden)]
pub use glib;
#[doc(hidden)]
pub use gio;
#[doc(hidden)]
pub use gtk;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("libpanel may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libpanel.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
#[allow(clippy::type_complexity)]
mod auto;

pub use auto::functions::*;
pub use auto::*;

pub mod builders {
    pub use crate::auto::builders::*;
}
pub mod prelude;
pub mod subclass;

mod save_delegate;
