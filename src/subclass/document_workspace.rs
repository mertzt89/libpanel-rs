use super::workspace::WorkspaceImpl;
use crate::{prelude::*, DocumentWorkspace, Frame, Position, Widget};
use glib::translate::*;
use gtk::subclass::prelude::*;

pub trait DocumentWorkspaceImpl: WorkspaceImpl {
    fn create_frame(&self, position: &Position) -> Frame {
        DocumentWorkspaceImplExt::parent_create_frame(self, position)
    }
    fn add_widget(&self, widget: &Widget, position: &Position) -> bool {
        DocumentWorkspaceImplExt::parent_add_widget(self, widget, position)
    }
}

pub trait DocumentWorkspaceImplExt: ObjectSubclass {
    fn parent_create_frame(&self, position: &Position) -> Frame;
    fn parent_add_widget(&self, widget: &Widget, position: &Position) -> bool;
}

impl<T: DocumentWorkspaceImpl> DocumentWorkspaceImplExt for T {
    fn parent_create_frame(&self, position: &Position) -> Frame {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::PanelDocumentWorkspaceClass;
            let f = (*parent_class)
                .create_frame
                .expect("No parent class implementation for \"create_frame\"");
            from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<DocumentWorkspace>()
                    .to_glib_none()
                    .0,
                position.to_glib_none().0,
            ))
        }
    }
    fn parent_add_widget(&self, widget: &Widget, position: &Position) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::PanelDocumentWorkspaceClass;
            if let Some(f) = (*parent_class).add_widget {
                from_glib(f(
                    self.obj()
                        .unsafe_cast_ref::<DocumentWorkspace>()
                        .to_glib_none()
                        .0,
                    widget.to_glib_none().0,
                    position.to_glib_none().0,
                ))
            } else {
                false
            }
        }
    }
}

unsafe impl<T: DocumentWorkspaceImpl> IsSubclassable<T> for DocumentWorkspace {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.create_frame = Some(panel_document_workspace_create_frame::<T>);
        klass.add_widget = Some(panel_document_workspace_add_widget::<T>);
    }
}

unsafe extern "C" fn panel_document_workspace_create_frame<T: DocumentWorkspaceImpl>(
    ptr: *mut ffi::PanelDocumentWorkspace,
    position: *mut ffi::PanelPosition,
) -> *mut ffi::PanelFrame {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let position: Borrowed<Position> = from_glib_borrow(position);

    DocumentWorkspaceImpl::create_frame(imp, &position).to_glib_full()
}

unsafe extern "C" fn panel_document_workspace_add_widget<T: DocumentWorkspaceImpl>(
    ptr: *mut ffi::PanelDocumentWorkspace,
    widget: *mut ffi::PanelWidget,
    position: *mut ffi::PanelPosition,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let widget: Borrowed<Widget> = from_glib_borrow(widget);
    let position: Borrowed<Position> = from_glib_borrow(position);

    DocumentWorkspaceImpl::add_widget(imp, &widget, &position).into_glib()
}
