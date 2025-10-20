use gtk4::glib;
use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use glib::Properties;
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

// FileItem GObject implementation
mod file_item_priv {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::FileItem)]
    pub struct FileItemPriv {
        #[property(get, set)]
        path: RefCell<String>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        is_dir: RefCell<bool>,
        #[property(get, set)]
        is_symlink: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileItemPriv {
        const NAME: &'static str = "FileItem";
        type Type = super::FileItem;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for FileItemPriv {}
}

glib::wrapper! {
    pub struct FileItem(ObjectSubclass<file_item_priv::FileItemPriv>);
}

impl FileItem {
    /// Creates a new FileItem from the given path.
    ///
    /// Returns None if the path doesn't exist or metadata cannot be read.
    pub fn new(path: &Path) -> Option<Self> {
        let metadata = fs::symlink_metadata(path).ok()?;
        let name = path.file_name()?.to_string_lossy().to_string();
        let is_dir = metadata.is_dir();
        let is_symlink = metadata.is_symlink();

        Some(glib::Object::builder()
            .property("path", path.to_string_lossy().to_string())
            .property("name", name)
            .property("is-dir", is_dir)
            .property("is-symlink", is_symlink)
            .build())
    }

    /// Returns the path as a PathBuf.
    pub fn path_buf(&self) -> PathBuf {
        PathBuf::from(self.path())
    }
}
